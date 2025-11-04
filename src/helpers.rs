use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    if d.as_secs_f64() >= 1.0 {
        format!("{:.3}s", d.as_secs_f64())
    } else if d.as_millis() >= 1 {
        format!("{:.3}ms", d.as_secs_f64() * 1_000.0)
    } else if d.as_micros() >= 1 {
        format!("{:.3}µs", d.as_secs_f64() * 1_000_000.0)
    } else {
        format!("{}ns", d.as_nanos())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn ns_range() {
        assert_eq!(format_duration(Duration::from_nanos(0)), "0ns");
        assert_eq!(format_duration(Duration::from_nanos(1)), "1ns");
        assert_eq!(format_duration(Duration::from_nanos(999)), "999ns");
    }

    #[test]
    fn us_range() {
        assert_eq!(format_duration(Duration::from_nanos(1_000)), "1.000µs");
        assert_eq!(format_duration(Duration::from_nanos(1_234)), "1.234µs");
        assert_eq!(format_duration(Duration::from_nanos(999_499)), "999.499µs");
    }

    #[test]
    fn ms_range() {
        assert_eq!(format_duration(Duration::from_micros(1_000)), "1.000ms");
        assert_eq!(format_duration(Duration::from_micros(1_500)), "1.500ms");
        assert_eq!(format_duration(Duration::from_millis(999)), "999.000ms");
        assert_eq!(format_duration(Duration::from_millis(1_000)), "1.000s");
    }

    #[test]
    fn s_and_above() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1.000s");
        assert_eq!(format_duration(Duration::from_millis(1_234)), "1.234s");
        assert_eq!(format_duration(Duration::from_secs(65)), "65.000s");
    }
}
