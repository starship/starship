//! Fish transport: announce `main\0right\0pid\0` on stdout, then publish
//! `main\0right\0timings\0` by atomic rename and notify fish with SIGUSR1.
//! Private stream leases isolate publishers; heartbeats check shell liveness.

use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use nix::errno::Errno;
use nix::sys::signal::{self, Signal};
use nix::unistd::{self, ForkResult, Pid};

use super::frame::{Frame, Sink};

/// Detach before starting any threads; the child announces the first prompt.
pub fn detach() {
    // SAFETY: called before thread startup, so no locks are held by other threads.
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => std::process::exit(0),
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
        Err(_) => std::process::exit(1),
    }
}

pub struct Snapshot {
    directory: PathBuf,
    lease: File,
    shell: Pid,
    main: String,
    right: String,
    timings: String,
    announced: bool,
}

impl Snapshot {
    pub fn new(directory: PathBuf, shell: i32) -> io::Result<Self> {
        let lease = File::options()
            .write(true)
            .create_new(true)
            .open(directory.join(".active"))?;
        Ok(Self {
            directory,
            lease,
            shell: Pid::from_raw(shell),
            main: String::new(),
            right: String::new(),
            timings: String::new(),
            announced: false,
        })
    }

    /// Fish revokes this stream by unlinking its private lease, never by PID.
    fn check_lease(&self) -> io::Result<()> {
        if self.lease.metadata()?.nlink() == 0 {
            Err(io::ErrorKind::BrokenPipe.into())
        } else {
            Ok(())
        }
    }

    /// Readers see a complete old or new snapshot.
    fn write(&self) -> io::Result<()> {
        let path = self.directory.join(std::process::id().to_string());
        let written = path.with_extension("new");
        fs::write(
            &written,
            [&self.main, "\0", &self.right, "\0", &self.timings, "\0"].concat(),
        )?;
        fs::rename(written, path)
    }

    /// Announce startup and close stdout so fish can finish reading.
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

    fn signal(&self, signal: Option<Signal>) -> io::Result<()> {
        self.check_lease()?;
        match signal::kill(self.shell, signal) {
            Err(Errno::ESRCH) => Err(io::ErrorKind::BrokenPipe.into()),
            result => result.map_err(io::Error::from),
        }
    }
}

impl Sink for Snapshot {
    fn send(&mut self, frame: Frame) -> io::Result<()> {
        self.check_lease()?;
        match frame {
            // The shell learns the process id with the first prompts.
            Frame::Process => Ok(()),
            Frame::Prompt { main, right } => {
                self.main = main.replace('\0', "");
                self.right = right.replace('\0', "");
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
            Frame::Heartbeat => self.signal(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nul_bytes_in_prompt_values_do_not_create_snapshot_fields() {
        let directory = tempfile::tempdir().unwrap();
        let mut snapshot = Snapshot::new(directory.path().to_path_buf(), i32::MAX).unwrap();
        snapshot.announced = true;
        // The shell PID deliberately does not exist; writing precedes signaling.
        let error = snapshot
            .send(Frame::Prompt {
                main: "ma\0in",
                right: "ri\0ght",
            })
            .unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        let path = directory.path().join(std::process::id().to_string());
        assert_eq!("main\0right\0\0", fs::read_to_string(path).unwrap());
        assert_eq!("main", snapshot.main);
        assert_eq!("right", snapshot.right);
    }

    #[test]
    fn completion_updates_preserve_the_prompt_pair() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join(std::process::id().to_string());
        let mut snapshot = Snapshot::new(
            directory.path().to_path_buf(),
            std::process::id().cast_signed(),
        )
        .unwrap();
        // Avoid announcing on the test process's stdout or sending it SIGUSR1.
        snapshot.main = "main\n> ".to_owned();
        snapshot.right = "right".to_owned();

        for timings in ["character=1", ""] {
            snapshot.send(Frame::Complete(timings)).unwrap();
            assert_eq!(
                format!("main\n> \0right\0{timings}\0"),
                fs::read_to_string(&path).unwrap(),
            );
        }
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
        let mut snapshot = Snapshot::new(directory.path().to_path_buf(), pid).unwrap();

        let error = snapshot.send(Frame::Heartbeat).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
    }

    #[test]
    fn revoking_a_private_lease_ends_every_kind_of_publication() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join(".active");
        let mut snapshot = Snapshot::new(
            directory.path().to_path_buf(),
            std::process::id().cast_signed(),
        )
        .unwrap();
        fs::remove_file(&path).unwrap();
        // A replacement file cannot give an old publisher ownership again.
        File::create(path).unwrap();

        for frame in [
            Frame::Process,
            Frame::Prompt {
                main: "main",
                right: "right",
            },
            Frame::Complete("timings"),
            Frame::Heartbeat,
        ] {
            assert_eq!(
                io::ErrorKind::BrokenPipe,
                snapshot.send(frame).unwrap_err().kind()
            );
        }
        assert!(
            !directory
                .path()
                .join(std::process::id().to_string())
                .exists()
        );
        drop(snapshot);
        directory.close().unwrap();
    }
}
