//! Frames delivered as a file, for a shell that cannot keep reading a pipe
//! while it waits for input: fish, which has no way to watch a descriptor and
//! whose `read` has no timeout.
//!
//! The stream runs in the background, and announces the first prompts on
//! standard output, which the shell reads through to its end. From then on, it
//! rewrites a file holding both prompts whenever they change, and signals the
//! shell to read it again. Reading a file never blocks, and however many
//! signals arrive at once, it holds the latest prompts. The file is named after
//! the stream's process, so that a stream the shell has stopped, and that has
//! yet to notice, cannot write over the prompts of the one after it.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use nix::errno::Errno;
use nix::sys::signal::{self, Signal};
use nix::unistd::{self, ForkResult, Pid};

use super::frame::{Frame, Sink};

/// Leaves the stream to a background process, so that the shell waits only
/// for the first prompts, which the background process then announces.
///
/// A fork carries only the thread that calls it, so it has to happen before
/// any other starts.
pub fn detach() {
    // SAFETY: nothing has started a thread yet, so the child has all the
    // threads, and all the locks, its parent had.
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => std::process::exit(0),
        Ok(ForkResult::Child) => {
            // Its own session, so that a signal to the shell's foreground job
            // cannot reach a stream that outlives the command line.
            let _ = unistd::setsid();
            // A stream never reads the terminal, and holding it open would keep
            // it from closing after the shell has gone.
            if let Ok(null) = File::open("/dev/null") {
                let _ = unistd::dup2_stdin(null);
            }
        }
        // In the foreground, the shell waits for the whole stream instead.
        Err(_) => {}
    }
}

/// The latest prompts, as `main\0right\0timings\0`, in a file in `directory`
/// named after the process streaming them.
pub struct Snapshot {
    directory: PathBuf,
    shell: Pid,
    main: String,
    right: String,
    timings: String,
    /// Whether the first prompts have been announced.
    announced: bool,
}

impl Snapshot {
    pub fn new(directory: PathBuf, shell: i32) -> Self {
        Self {
            directory,
            shell: Pid::from_raw(shell),
            main: String::new(),
            right: String::new(),
            timings: String::new(),
            announced: false,
        }
    }

    /// Replaces the file whole, so that a reader sees either the old one or
    /// the new one.
    fn write(&self) -> io::Result<()> {
        let path = self.directory.join(std::process::id().to_string());
        let written = path.with_extension("new");
        fs::write(
            &written,
            [&self.main, "\0", &self.right, "\0", &self.timings, "\0"].concat(),
        )?;
        fs::rename(written, path)
    }

    /// Hands the first prompts to the shell waiting on standard output, then
    /// lets that wait end.
    fn announce(&mut self) -> io::Result<()> {
        let mut output = io::stdout().lock();
        write!(
            output,
            "{}\0{}\0{}\0",
            self.main,
            self.right,
            std::process::id()
        )?;
        output.flush()?;
        unistd::dup2_stdout(File::options().write(true).open("/dev/null")?)?;
        self.announced = true;
        Ok(())
    }

    /// Sends `signal` to the shell, if it is still there to read the prompts.
    fn signal(&self, signal: Option<Signal>) -> io::Result<()> {
        match signal::kill(self.shell, signal) {
            Err(Errno::ESRCH) => Err(io::ErrorKind::BrokenPipe.into()),
            result => result.map_err(io::Error::from),
        }
    }
}

impl Sink for Snapshot {
    fn send(&mut self, frame: Frame) -> io::Result<()> {
        match frame {
            // The shell learns the process id with the first prompts.
            Frame::Process => Ok(()),
            Frame::Prompt { main, right } => {
                main.clone_into(&mut self.main);
                right.clone_into(&mut self.right);
                if self.announced {
                    self.write()?;
                    self.signal(Some(Signal::SIGUSR1))
                } else {
                    self.announce()
                }
            }
            Frame::Complete(timings) => {
                timings.clone_into(&mut self.timings);
                self.write()
            }
            // Only a missing shell could stop the stream.
            Frame::Heartbeat => self.signal(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_file_is_rewritten_whole_and_named_after_the_process_streaming() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join(std::process::id().to_string());
        let mut snapshot = Snapshot::new(
            directory.path().to_path_buf(),
            std::process::id().cast_signed(),
        );

        snapshot.send(Frame::Complete("character=1")).unwrap();
        assert_eq!("\0\0character=1\0", fs::read_to_string(&path).unwrap());
        let mut entries: Vec<_> = fs::read_dir(directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        entries.sort();
        assert_eq!(vec![path.file_name().unwrap().to_owned()], entries);
    }

    #[test]
    fn a_shell_that_has_gone_ends_the_stream() {
        let mut gone = crate::utils::create_command("true")
            .unwrap()
            .spawn()
            .unwrap();
        let pid = gone.id().cast_signed();
        gone.wait().unwrap();
        let directory = tempfile::tempdir().unwrap();
        let mut snapshot = Snapshot::new(directory.path().to_path_buf(), pid);

        let error = snapshot.send(Frame::Heartbeat).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
    }
}
