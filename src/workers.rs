//! Modules rendered on worker threads.

use std::iter;
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, mpsc};
use std::thread::{self, Scope};
use std::time::Instant;

use indexmap::IndexSet;

use crate::context::Context;
use crate::modules;
use crate::segment::Segment;

/// What one module rendered to.
pub struct Rendered {
    /// The module's position in the plan.
    pub position: usize,
    pub segments: Vec<Segment>,
}

/// Hands modules to worker threads to render, and receives what they render.
pub struct Workers<'run> {
    /// `None` when no worker thread could be started, so that modules render
    /// on the thread that asks for them instead.
    jobs: Option<mpsc::Sender<usize>>,
    rendered: mpsc::Sender<Rendered>,
    results: mpsc::Receiver<Rendered>,
    render: &'run (dyn Fn(usize) -> Rendered + Sync),
}

impl Workers<'_> {
    /// Renders the module at `position` in the plan as soon as a worker is
    /// free. Modules render in the order they are asked for.
    pub fn render(&self, position: usize) {
        let position = match &self.jobs {
            Some(jobs) => match jobs.send(position) {
                Ok(()) => return,
                Err(unsent) => unsent.0,
            },
            None => position,
        };
        // The receiving end is this struct's own, so the send cannot fail.
        let _ = self.rendered.send((self.render)(position));
    }

    /// Waits for a module to finish rendering, or for `deadline` to pass if
    /// there is one, and returns every module finished by then: none if the
    /// deadline passed first. The channel never closes, since this holds a
    /// sender to it.
    pub fn wait(&self, deadline: Option<Instant>) -> Vec<Rendered> {
        let first = match deadline {
            Some(deadline) => self
                .results
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .ok(),
            None => self.results.recv().ok(),
        };
        first
            .into_iter()
            .chain(iter::from_fn(|| self.results.try_recv().ok()))
            .collect()
    }
}

/// Runs `body` with workers that render `modules`, `waiting` of which wait on
/// subprocesses or the filesystem, if that is known. Once `body` returns, the
/// workers finish the module they are rendering, and render no more.
///
/// A module that waits takes a worker of its own, so there may be many more
/// workers than processors. They start one another, each two more before it
/// takes a module, so that they double in number while they render, and the
/// thread asking for modules starts only the first.
pub fn with_workers<R>(
    context: &Context,
    modules: &IndexSet<String>,
    waiting: Option<usize>,
    body: impl FnOnce(&Workers) -> R,
) -> R {
    // A module that panics renders nothing, rather than taking the prompt,
    // and every module still rendering, down with it.
    let render = |position| Rendered {
        position,
        segments: panic::catch_unwind(AssertUnwindSafe(|| {
            modules::handle(&modules[position], context)
        }))
        .unwrap_or_else(|_| {
            log::error!("The {} module panicked while rendering", modules[position]);
            None
        })
        .map_or_else(Vec::new, |module| module.segments),
    };
    let (jobs, queue) = mpsc::channel();
    let queue = Mutex::new(queue);
    let (rendered, results) = mpsc::channel();
    let finished = AtomicBool::new(false);

    let work = || {
        // Only taking the next job is serialized, never rendering it.
        let next = || {
            queue
                .lock()
                .expect("no worker panics holding the queue")
                .recv()
        };
        while let Ok(position) = next() {
            if finished.load(Ordering::Relaxed) || rendered.send(render(position)).is_err() {
                break;
            }
        }
    };

    thread::scope(|scope| {
        let count = crate::num_module_threads(modules.len(), waiting);
        let started = count > 0 && start(scope, 0, count, &work);
        let workers = Workers {
            jobs: started.then_some(jobs),
            rendered: rendered.clone(),
            results,
            render: &render,
        };
        let result = body(&workers);
        // Dropping the queue wakes any idle worker to stop.
        finished.store(true, Ordering::Relaxed);
        drop(workers);
        result
    })
}

/// Starts worker `index` of `count`, which starts its two children in the tree
/// of workers before it works. Returns whether it started.
fn start<'scope, 'env>(
    scope: &'scope Scope<'scope, 'env>,
    index: usize,
    count: usize,
    work: &'env (dyn Fn() + Sync),
) -> bool {
    let started = thread::Builder::new().spawn_scoped(scope, move || {
        for child in [2 * index + 1, 2 * index + 2] {
            if child < count {
                start(scope, child, count, work);
            }
        }
        work();
    });
    if let Err(error) = &started {
        log::debug!("Unable to start a module worker: {error}");
    }
    started.is_ok()
}

/// Renders every module, returning the segments of each in plan order.
pub fn render_all(context: &Context, modules: &IndexSet<String>) -> Vec<Vec<Segment>> {
    with_workers(context, modules, None, |workers| {
        (0..modules.len()).for_each(|position| workers.render(position));
        let mut segments = vec![Vec::new(); modules.len()];
        for rendered in iter::repeat_with(|| workers.wait(None))
            .flatten()
            .take(modules.len())
        {
            segments[rendered.position] = rendered.segments;
        }
        segments
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::test::default_context;

    /// Each module waits for the other to have started, so they finish only
    /// if two workers render them at the same time.
    #[test]
    fn workers_render_modules_at_the_same_time() {
        let directory = tempfile::tempdir().unwrap();
        let mut context = default_context().set_config(toml::toml! {
            command_timeout = 10_000
            [custom.first]
            command = "touch first; until test -f second; do sleep 0.01; done; echo 1"
            when = true
            shell = ["/bin/sh"]
            format = "$output"
            [custom.second]
            command = "touch second; until test -f first; do sleep 0.01; done; echo 2"
            when = true
            shell = ["/bin/sh"]
            format = "$output"
        });
        context.current_dir = directory.path().to_path_buf();
        let modules = IndexSet::from(["custom.first".to_owned(), "custom.second".to_owned()]);

        let rendered: Vec<String> = render_all(&context, &modules)
            .iter()
            .map(|segments| segments.iter().map(Segment::value).collect())
            .collect();

        assert_eq!(["1", "2"], rendered.as_slice());
    }
}
