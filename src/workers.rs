//! Modules rendered on worker threads.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SendError, Sender, channel};
use std::thread::{self, Scope};
use std::time::Instant;

use indexmap::IndexSet;

use crate::context::Context;
use crate::modules;
use crate::segment::Segment;

const BINARY_BRANCH_COUNT: usize = 2;

pub struct Rendered {
    pub position: usize,
    pub segments: Vec<Segment>,
}

/// Submits modules for rendering and receives their results.
pub struct Workers<'run> {
    /// Absent when the first worker could not be started or none was requested.
    job_sender: Option<Sender<usize>>,
    result_receiver: Receiver<Rendered>,
    state: &'run WorkerState<'run>,
}

impl Workers<'_> {
    /// Queue a plan position, falling back to synchronous rendering if needed.
    /// Submission order does not guarantee completion order.
    pub fn render(&self, position: usize) {
        if self
            .job_sender
            .as_ref()
            .is_some_and(|job_sender| job_sender.send(position).is_ok())
        {
            return;
        }

        // This handle owns the receiver, so synchronous delivery cannot fail.
        let _ = self.state.render_and_send(position);
    }

    /// Wait for one result or the deadline, then drain available results.
    /// Without a deadline, wait indefinitely.
    pub fn wait(&self, deadline: Option<Instant>) -> Vec<Rendered> {
        self.receive_first(deadline)
            .into_iter()
            .chain(self.result_receiver.try_iter())
            .collect()
    }

    fn receive_first(&self, deadline: Option<Instant>) -> Option<Rendered> {
        match deadline {
            Some(deadline) => self
                .result_receiver
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .ok(),
            None => self.result_receiver.recv().ok(),
        }
    }

    fn collect_segments(&self, module_count: usize) -> Vec<Vec<Segment>> {
        let mut segments = vec![Vec::new(); module_count];

        for rendered in self.result_receiver.iter().take(module_count) {
            segments[rendered.position] = rendered.segments;
        }

        segments
    }
}

impl Drop for Workers<'_> {
    fn drop(&mut self) {
        // Set before closing the queue, including on unwind; publishes no other state.
        self.state.stopping.store(true, Ordering::Relaxed);
    }
}

/// Shared inputs and synchronization, borrowed by every worker.
struct WorkerState<'run> {
    context: &'run Context<'run>,
    module_names: &'run IndexSet<String>,
    job_receiver: Mutex<Receiver<usize>>,
    result_sender: Sender<Rendered>,
    stopping: AtomicBool,
}

impl WorkerState<'_> {
    fn is_stopping(&self) -> bool {
        self.stopping.load(Ordering::Relaxed)
    }

    fn next_position(&self) -> Option<usize> {
        // The statement ends the lock guard's lifetime before any rendering.
        let position = self
            .job_receiver
            .lock()
            .expect("workers never panic while holding the job queue")
            .recv()
            .ok()?;

        (!self.is_stopping()).then_some(position)
    }

    fn render_and_send(&self, position: usize) -> Result<(), SendError<Rendered>> {
        self.result_sender.send(Rendered {
            position,
            segments: render_module(self.context, &self.module_names[position]),
        })
    }

    fn run(&self) {
        while let Some(position) = self.next_position() {
            if self.render_and_send(position).is_err() {
                break;
            }
        }
    }
}

/// Renders a module, turning an unwinding panic into an empty result.
fn render_module(context: &Context, module_name: &str) -> Vec<Segment> {
    match catch_unwind(AssertUnwindSafe(|| modules::handle(module_name, context))) {
        Ok(Some(module)) => module.segments,
        Ok(None) => Vec::new(),
        Err(_) => {
            log::error!("The {module_name} module panicked while rendering");
            Vec::new()
        }
    }
}

/// Run `body` with a scoped worker tree sized for the plan.
/// Return or unwind stops queued work and joins workers; in-flight work finishes.
pub fn with_workers<Output>(
    context: &Context,
    module_names: &IndexSet<String>,
    waiting_module_count: Option<usize>,
    body: impl FnOnce(&Workers) -> Output,
) -> Output {
    let worker_count = crate::num_module_threads(module_names.len(), waiting_module_count);
    let (job_sender, job_receiver) = channel();
    let (result_sender, result_receiver) = channel();
    let state = WorkerState {
        context,
        module_names,
        job_receiver: Mutex::new(job_receiver),
        result_sender,
        stopping: AtomicBool::new(false),
    };

    thread::scope(|scope| {
        let workers = Workers {
            job_sender: start_workers(scope, worker_count, &state).then_some(job_sender),
            result_receiver,
            state: &state,
        };

        body(&workers)
    })
}

/// Starts the root of a subtree with at most `worker_count` workers.
/// A failed child leaves the surviving workers to service the shared queue.
fn start_workers<'scope, 'environment>(
    scope: &'scope Scope<'scope, 'environment>,
    worker_count: usize,
    state: &'scope WorkerState<'environment>,
) -> bool {
    if worker_count == 0 || state.is_stopping() {
        return false;
    }

    let started = thread::Builder::new().spawn_scoped(scope, move || {
        for child_worker_count in split_worker_budget(worker_count - 1) {
            start_workers(scope, child_worker_count, state);
        }

        state.run();
    });

    match started {
        Ok(_) => true,
        Err(error) => {
            log::debug!("Unable to start a module worker: {error}");
            false
        }
    }
}

/// Divides the remaining workers between the two child subtrees.
fn split_worker_budget(remaining_workers: usize) -> [usize; BINARY_BRANCH_COUNT] {
    let first_branch = remaining_workers / BINARY_BRANCH_COUNT;
    [first_branch, remaining_workers - first_branch]
}

/// Renders every module, returning its segments in plan order.
pub fn render_all(context: &Context, module_names: &IndexSet<String>) -> Vec<Vec<Segment>> {
    if module_names.is_empty() {
        return Vec::new();
    }

    with_workers(context, module_names, None, |workers| {
        for position in 0..module_names.len() {
            workers.render(position);
        }

        workers.collect_segments(module_names.len())
    })
}
