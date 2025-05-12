//! This module utilises the [fuse](https://github.com/libfuse/libfuse) API to mock a filesystem
//! that is stalling, say, due to network latency.
//!
//! As documented
//! [here](https://github.com/libfuse/libfuse?tab=readme-ov-file#supported-platforms),
//! libfuse targets Linux, BSD and OSX to some extent.
//! For now, we limit to Linux. libfuse package must be installed.

use fuser::Filesystem;
