use super::{Context, Module, RootModuleConfig};
use crate::configs::linux_netns::LinuxNetNsConfig;
use crate::formatter::StringFormatter;
use procfs::process::Process;
use std::fs;

/// Creates a module giving the current network namespace
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let netns_name = get_netns_name()?;

    let mut module = context.new_module("linux_netns");
    let config = LinuxNetNsConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "ns" => Some(Ok(&netns_name)),
                _ => None,
            })
            .parse(None)
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `linux_netns`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// return the network namespace name of the current process:
///   - a friendly name when the current process' network namespace was named
///     explicitly, e.g. using `ip netns add` command  [1]
///   - when the namespace doesn't have a friendly name, a name derived from the
///     numerical ID of the network namespace (base 10 or 16) whe the current
///     process' network namespace can be determined to be different from that
///     of one of its parent processes [2]
///   - in the remaining cases, no namespace is returned (None):
///     - process is in the "root" network namespace [3]
///     - the network namespace could not be determined, or it wasn't possible
///       if that namespace was root or not [4]
fn get_netns_name() -> Option<String> {
    let mut refp: Option<(Process, NetNsId)> = None;

    for p in ParentProcessIterator::new() {
        // the try on the next line implies that a failure to retrieve the netns
        // of p (process in the iteration) leads to None being returned. In
        // other words if we didn't identify a (named) NetNS up until this
        // point, we consider that the process is in a root or undetermined
        // NetNS
        let netns = netns_id_for_process(&p)?; // [4]
        match refp {
            None => {
                if let Some(name) = name_for_netns_id(netns, &p) {
                    return Some(name); // [1]
                }
                // Current process NetNS has no explicit name
                refp = Some((p, netns));
            }
            Some((_, cns)) => {
                if netns != cns {
                    // parent with a different NetNS, return the refp's
                    // netns (in numerical form)
                    return Some(format!("{:x}", cns)); // [2]
                }
            }
        }
    }
    // Reached the root process without any NetNS change, we're in the "root"
    // network namespace so None is returned
    None // [3]
}

type NetNsId = usize;

#[derive(Default)]
struct ParentProcessIterator {
    pid: Option<libc::pid_t>,
}

impl ParentProcessIterator {
    fn new() -> ParentProcessIterator {
        ParentProcessIterator::default()
    }
}

impl Iterator for ParentProcessIterator {
    type Item = Process;

    fn next(&mut self) -> Option<Process> {
        let p = match self.pid {
            None => Process::myself(),
            Some(pid) => Process::new(pid),
        };
        let p = p.ok()?;

        self.pid = Some(p.stat.ppid);
        Some(p)
    }
}

/// retrieves the numeric namespace identifier for a process
fn netns_id_for_process(proc: &Process) -> Option<NetNsId> {
    let nspath = format!("/proc/{}/ns/net", proc.pid);

    fs::read_link(nspath).ok().and_then(|l| {
        l.to_str()?
            .splitn(2, "net:[")
            .nth(1)?
            .splitn(2, ']')
            .next()
            .and_then(|nsid| nsid.parse::<NetNsId>().ok())
    })
}

/// return the name of a network namespace based on its numeric identifier.
fn name_for_netns_id(nsid: NetNsId, p: &Process) -> Option<String> {
    let miv = p.mountinfo().ok()?;
    let nsroot = format!("net:[{}]", nsid);

    // look for mount info matching the netnsid. We stop on the first match
    // so aliases (i.e. other mountpoints for the same NS) will be ignored
    for mi in miv {
        if mi.root == nsroot {
            let fname = mi.mount_point.as_path().file_name();

            if let Some(fname) = fname.and_then(|f| f.to_str()) {
                return Some(fname.to_string());
            }
            break;
        }
    }
    None
}
