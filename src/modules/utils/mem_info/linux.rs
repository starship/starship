use super::MemoryInfo;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

// procfs says kb but it's kib
// try to parse the unit but right now the kernel is hardcoded to use kib
fn to_kib(value: usize, unit: Option<&str>) -> Option<usize> {
    let in_kib = match unit.map(|s| s.to_lowercase()).as_deref() {
        Some("b") | None => value / 1024,
        Some("kb") | Some("kib") => value,
        Some("mb") | Some("mib") => value * 1024,
        Some("gb") | Some("gib") => value * 1024_usize.pow(2),
        Some("tb") | Some("tib") => value * 1024_usize.pow(3),
        _ => return None,
    };

    Some(in_kib)
}

fn parse_meminfo(reader: &mut impl BufRead) -> Result<MemoryInfo> {
    let mut free = None;
    let mut avail = None;
    let mut total = None;
    let mut swap_free = None;
    let mut swap_total = None;

    let mut found = 0;

    for l in reader.lines() {
        let line = l?;
        let mut split = line.split_whitespace();
        let key = match split.next() {
            Some("MemTotal:") => &mut total,
            Some("MemFree:") => &mut free,
            Some("MemAvailable:") => &mut avail,
            Some("SwapTotal:") => &mut swap_total,
            Some("SwapFree:") => &mut swap_free,
            _ => continue,
        };

        if key.is_some() {
            return Err(Error::new(ErrorKind::InvalidData, "Encountered key twice!"));
        }

        let value = split
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing Value!"))?
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let unit = split.next();

        *key = to_kib(value, unit);

        found += 1;

        if found == 5 {
            break;
        }
    }

    if (found == 4 && avail.is_some()) || found < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to find expected keys!",
        ));
    }

    Ok(MemoryInfo {
        avail,
        free,
        total: total.unwrap(),
        swap_free: swap_free.unwrap(),
        swap_total: swap_total.unwrap(),
    })
}

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        let f = File::open("/proc/meminfo")?;
        let mut buf = BufReader::new(f);

        parse_meminfo(&mut buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn linux_parse_no_avail() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 kB\n",
            "MemFree:        50 kB\n",
            "SwapTotal:      90 kB\n",
            "SwapFree:       80 kB\n"
        ));

        let info = parse_meminfo(&mut input).unwrap();

        assert_eq!(info.avail, None);
    }

    #[test]
    fn linux_parse_no_free() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 kB\n",
            "MemAvailable:   50 kB\n",
            "SwapTotal:      90 kB\n",
            "SwapFree:       80 kB\n"
        ));

        let info = parse_meminfo(&mut input);

        assert!(info.is_err());
    }

    #[test]
    fn linux_parse_no_free_avail() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 kB\n",
            "SwapTotal:      90 kB\n",
            "SwapFree:       80 kB\n"
        ));

        let info = parse_meminfo(&mut input);

        assert!(info.is_err());
    }

    #[test]
    fn linux_parse_dup_line() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 kB\n",
            "MemFree:        50 kB\n",
            "MemAvailable:   50 kB\n",
            "SwapTotal:      90 kB\n",
            "MemAvailable:   1000 kB\n",
            "SwapFree:       80 kB\n"
        ));

        let info = parse_meminfo(&mut input);

        assert!(info.is_err());
    }

    #[test]
    fn linux_parse_early_exit() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 kB\n",
            "MemFree:        50 kB\n",
            "MemAvailable:   50 kB\n",
            "SwapTotal:      90 kB\n",
            "SwapFree:       80 kB\n",
            "MemAvailable:   0 kB\n"
        ));

        let info = parse_meminfo(&mut input);

        assert!(info.is_ok());
    }

    #[test]
    fn linux_parse_correct() {
        let mut input = Cursor::new(concat!(
            "MemTotal:       100 gB\n",
            "MemFree:        50 mB\n",
            "MemAvailable:   60 kiB\n",
            "SwapTotal:      90 B\n",
            "SwapFree:       80 kB\n"
        ));

        let info = parse_meminfo(&mut input).unwrap();

        assert_eq!(info.total, 100 * 1024_usize.pow(2));
        assert_eq!(info.free.unwrap(), 50 * 1024);
        assert_eq!(info.avail.unwrap(), 60);
        assert_eq!(info.swap_total, 0);
        assert_eq!(info.swap_free, 80);
    }
}
