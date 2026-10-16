//! Fish cannot watch a pipe while it waits for input, so a stream publishes
//! to it through a directory of its own instead: each pair of prompts
//! replaces a file named `prompt` whole, after which fish is sent `SIGUSR1`.
//! The directory is named on standard output, which then closes, once the
//! first pair is there. Fish ends the stream by creating `stop` in it, and it
//! ends on its own once fish has gone; either way, it removes the directory.

use std::fs::{self, File};
use std::io::{self, Write};
use std::os::fd::{AsFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::{env, process, thread};

use nix::errno::Errno;
use nix::fcntl::{self, AtFlags, OFlag};
use nix::sys::signal::{self, Signal};
use nix::sys::stat::{self, Mode};
use nix::unistd::{self, ForkResult, Pid};

use super::{Prompts, Sink, encode};

/// How often a stream looks for whether fish still wants it.
const LOOK: Duration = Duration::from_secs(1);

/// Detach before starting any threads; the child announces the first prompt.
pub fn detach() {
    // SAFETY: called before thread startup, so no locks are held by other threads.
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => process::exit(0),
        Ok(ForkResult::Child) => {
            // Isolate the stream from foreground job signals.
            let _ = unistd::setsid();
            // Release the terminal; the stream never reads it.
            if let Ok(null) = File::open("/dev/null") {
                let _ = unistd::dup2_stdin(null);
            }
        }
        // Fish waits for startup EOF; a foreground refresh loop would never
        // finish. Fail promptly instead of starting the long-lived publisher.
        Err(_) => process::exit(1),
    }
}

/// Keeps the prompts last published, for fish to read whenever it is
/// signaled, until it lets go of the stream, which ends the process.
pub fn linger() -> ! {
    loop {
        thread::park();
    }
}

pub struct Snapshot {
    directory: Arc<Directory>,
    /// The pipe fish waits on until the first prompts are published.
    waiting: Option<OwnedFd>,
}

/// The directory a stream publishes in, and the fish it publishes for.
struct Directory {
    path: PathBuf,
    handle: File,
    shell: Pid,
}

impl Snapshot {
    /// Publishes for the fish `shell`, which waits on standard output.
    pub fn for_fish(shell: Pid) -> io::Result<Self> {
        // Nothing else is written to the pipe fish waits on.
        let waiting = io::stdout().as_fd().try_clone_to_owned()?;
        unistd::dup2_stdout(File::options().write(true).open("/dev/null")?)?;
        let snapshot = Self::new(shell, waiting)?;
        let watched = Arc::clone(&snapshot.directory);
        thread::Builder::new()
            .name("stream-shell".to_owned())
            .spawn(move || {
                loop {
                    thread::sleep(LOOK);
                    if watched.released() {
                        watched.remove();
                        process::exit(0);
                    }
                }
            })
            .inspect_err(|_| snapshot.directory.remove())?;
        Ok(snapshot)
    }

    fn new(shell: Pid, waiting: OwnedFd) -> io::Result<Self> {
        let path = unistd::mkdtemp(&env::temp_dir().join("starship-XXXXXX"))?;
        let handle = File::open(&path).inspect_err(|_| {
            let _ = fs::remove_dir(&path);
        })?;
        Ok(Self {
            directory: Arc::new(Directory {
                path,
                handle,
                shell,
            }),
            waiting: Some(waiting),
        })
    }

    fn publish(&mut self, prompts: &Prompts) -> io::Result<()> {
        let directory = &self.directory;
        if directory.released() {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        // Replaced whole, so that fish reads one pair or another, never part.
        let mut published = File::from(fcntl::openat(
            &directory.handle,
            "prompt.new",
            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_CLOEXEC,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?);
        published.write_all(encode(prompts).as_bytes())?;
        fcntl::renameat(&directory.handle, "prompt.new", &directory.handle, "prompt")?;
        match self.waiting.take() {
            Some(waiting) => {
                let mut waiting = File::from(waiting);
                waiting.write_all(directory.path.as_os_str().as_bytes())?;
                waiting.write_all(b"\0")
            }
            None => signal::kill(directory.shell, Signal::SIGUSR1).map_err(io::Error::from),
        }
    }
}

impl Directory {
    /// Whether fish has let go of the stream: asked it to stop, or gone.
    fn released(&self) -> bool {
        stat::fstatat(&self.handle, "stop", AtFlags::AT_SYMLINK_NOFOLLOW).is_ok()
            || signal::kill(self.shell, None) == Err(Errno::ESRCH)
    }

    fn remove(&self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

impl Sink for Snapshot {
    /// A stream that cannot publish has ended, and its directory with it.
    fn draw(&mut self, prompts: &Prompts) -> io::Result<()> {
        self.publish(prompts)
            .inspect_err(|_| self.directory.remove())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    /// A process to stand for fish, which `SIGUSR1` ends.
    fn fish() -> std::process::Child {
        crate::utils::create_command("sleep")
            .unwrap()
            .arg("30")
            .spawn()
            .unwrap()
    }

    fn snapshot(shell: &std::process::Child) -> (Snapshot, File) {
        let (reading, writing) = unistd::pipe().unwrap();
        let snapshot = Snapshot::new(Pid::from_raw(shell.id().cast_signed()), writing).unwrap();
        (snapshot, File::from(reading))
    }

    #[test]
    fn fish_reads_whole_pairs_without_nuls_once_told_where() {
        let mut fish = fish();
        let (mut snapshot, mut waiting) = snapshot(&fish);
        let path = snapshot.directory.path.clone();

        snapshot
            .draw(&["ma\0in".to_owned(), "ri\0ght".to_owned()])
            .unwrap();
        let mut announced = Vec::new();
        waiting.read_to_end(&mut announced).unwrap();
        assert_eq!(
            [path.as_os_str().as_bytes(), b"\0"].concat(),
            announced,
            "the pipe closes once it names the directory"
        );
        assert_eq!(
            "main\0right\0",
            fs::read_to_string(path.join("prompt")).unwrap()
        );

        snapshot
            .draw(&["next\n> ".to_owned(), String::new()])
            .unwrap();
        assert_eq!(
            "next\n> \0\0",
            fs::read_to_string(path.join("prompt")).unwrap()
        );
        assert!(!fish.wait().unwrap().success(), "fish was signaled");
        snapshot.directory.remove();
    }

    #[test]
    fn fish_lets_go_by_asking_the_stream_to_stop_or_by_going() {
        let mut fish = fish();
        let (mut snapshot, _waiting) = snapshot(&fish);
        let path = snapshot.directory.path.clone();
        let prompts = ["main".to_owned(), "right".to_owned()];
        File::create(path.join("stop")).unwrap();

        assert_eq!(
            io::ErrorKind::BrokenPipe,
            snapshot.draw(&prompts).unwrap_err().kind()
        );
        assert!(!path.exists(), "the stream removes its directory");
        fish.kill().unwrap();
        fish.wait().unwrap();
        let (snapshot, _waiting) = self::snapshot(&fish);
        assert!(snapshot.directory.released(), "fish has gone");
        snapshot.directory.remove();
    }
}
