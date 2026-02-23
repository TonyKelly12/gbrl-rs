//! Bed extension Y-axis translation layer.
//!
//! Intercepts Y move commands and splits those that exceed the gantry limit:
//! first segment moves gantry Y to the limit, then the bed axis (MOTOR4, e.g. A)
//! carries the overflow. Transparent to the caller — they get a list of commands
//! to send; no other module needs to know about the bed extension.

/// Default gantry Y limit in mm (24 inches). Moves beyond this are split;
/// overflow is sent as bed-axis (A) moves.
pub const DEFAULT_GANTRY_Y_LIMIT_MM: f64 = 609.6;

/// Configuration for the bed extension translator.
#[derive(Clone, Debug)]
pub struct MotionConfig {
    /// Gantry Y travel limit in mm. Y moves beyond this are split.
    pub gantry_y_limit_mm: f64,
    /// G-code axis letter for the bed rail (MOTOR4). Typically 'A'.
    pub bed_axis: char,
}

impl Default for MotionConfig {
    fn default() -> Self {
        Self {
            gantry_y_limit_mm: DEFAULT_GANTRY_Y_LIMIT_MM,
            bed_axis: 'A',
        }
    }
}

/// Extracts a numeric value after a given axis letter (e.g. 'Y' -> 10.5 from "Y10.5").
fn parse_axis_value(line: &str, axis: char) -> Option<f64> {
    let upper = axis.to_uppercase().next().unwrap_or(axis);
    let lower = axis.to_lowercase().next().unwrap_or(axis);
    for (i, c) in line.chars().enumerate() {
        if c == upper || c == lower {
            let rest = line.get(i + 1..)?;
            let end = rest
                .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                .map(|i| i)
                .unwrap_or(rest.len());
            let num_str = rest.get(..end)?.trim();
            return num_str.parse().ok();
        }
    }
    None
}

/// Returns true if the line is a rapid (G0) or linear (G1) move.
fn is_move_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with(';') {
        return false;
    }
    // Match G0 or G1 as whole word (at start or after space)
    let mut i = 0;
    while i < trimmed.len() {
        if trimmed.get(i..i + 2) == Some("G0") || trimmed.get(i..i + 2) == Some("G1") {
            let after = trimmed.get(i + 2..).unwrap_or("");
            if after.is_empty() || after.starts_with(' ') || after.starts_with('\t')
                || after.starts_with('X') || after.starts_with('x')
                || after.starts_with('Y') || after.starts_with('y')
                || after.starts_with('Z') || after.starts_with('z')
                || after.starts_with('F') || after.starts_with('f')
                || after.starts_with('A') || after.starts_with('a')
            {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Replaces the Y value in a g-code line with a new value (keeps format roughly).
fn replace_y_in_line(line: &str, new_y: f64) -> String {
    let mut result = String::with_capacity(line.len() + 16);
    let mut i = 0;
    let bytes = line.as_bytes();
    while i < bytes.len() {
        if (bytes[i] == b'Y' || bytes[i] == b'y') && i + 1 < bytes.len() {
            let next = bytes[i + 1] as char;
            if next == '-' || next == '.' || next.is_ascii_digit() {
                result.push(bytes[i] as char);
                i += 1;
                while i < bytes.len()
                    && (bytes[i] == b'-' || bytes[i] == b'.' || (bytes[i] as char).is_ascii_digit())
                {
                    i += 1;
                }
                result.push_str(&format!("{:.4}", new_y));
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

/// Builds a bed-axis move line (e.g. "G1 A10.5 F300").
fn bed_axis_line(config: &MotionConfig, distance_mm: f64, feed: Option<f64>) -> String {
    let ax = config.bed_axis.to_uppercase().next().unwrap_or(config.bed_axis);
    let mut s = format!("G1 {}{:.4}", ax, distance_mm);
    if let Some(f) = feed {
        s.push_str(&format!(" F{:.4}", f));
    }
    s
}

/// Translation state (modal and current position).
struct TranslateState {
    absolute: bool,
    current_y_mm: f64,
}

/// Translates a sequence of g-code lines: splits Y moves that exceed the gantry limit
/// into gantry move + bed-axis move. Returns the new list of lines to send.
///
/// Tracks G90/G91 (absolute/relative) and current Y position. Non-move lines and
/// moves without Y are passed through unchanged.
pub fn translate_lines(lines: &[impl AsRef<str>], config: &MotionConfig) -> Vec<String> {
    let mut state = TranslateState {
        absolute: true,
        current_y_mm: 0.0,
    };
    let limit = config.gantry_y_limit_mm;
    let mut out: Vec<String> = Vec::new();

    for line in lines {
        let line = line.as_ref().trim();
        if line.is_empty() || line.starts_with(';') {
            out.push(line.to_string());
            continue;
        }

        // Modal: G90 / G91
        if line.contains("G90") || line.contains("g90") {
            state.absolute = true;
        }
        if line.contains("G91") || line.contains("g91") {
            state.absolute = false;
        }

        if !is_move_line(line) {
            out.push(line.to_string());
            continue;
        }

        let y_opt = parse_axis_value(line, 'Y');
        let feed = parse_axis_value(line, 'F');

        let Some(y_value) = y_opt else {
            out.push(line.to_string());
            continue;
        };

        let target_y = if state.absolute {
            y_value
        } else {
            state.current_y_mm + y_value
        };

        if target_y <= limit {
            out.push(line.to_string());
            state.current_y_mm = target_y;
            continue;
        }

        // Split: move gantry to limit, then bed for the rest
        let to_limit = limit - state.current_y_mm;
        let overflow = target_y - limit;

        if to_limit > 0.0 {
            let first_line = if state.absolute {
                replace_y_in_line(line, limit)
            } else {
                replace_y_in_line(line, to_limit)
            };
            out.push(first_line);
        }
        out.push(bed_axis_line(config, overflow, feed));
        state.current_y_mm = target_y;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let c = MotionConfig::default();
        assert_eq!(c.gantry_y_limit_mm, DEFAULT_GANTRY_Y_LIMIT_MM);
        assert_eq!(c.bed_axis, 'A');
    }

    #[test]
    fn test_parse_axis_value() {
        assert_eq!(parse_axis_value("G1 Y10.5 F300", 'Y'), Some(10.5));
        assert_eq!(parse_axis_value("G1 X1 Y-2.5 Z0", 'Y'), Some(-2.5));
        assert_eq!(parse_axis_value("G1 X1 Z0", 'Y'), None);
        assert_eq!(parse_axis_value("F500", 'F'), Some(500.0));
    }

    #[test]
    fn test_is_move_line() {
        assert!(is_move_line("G0 Y10"));
        assert!(is_move_line("G1 X10 Y20 F300"));
        assert!(is_move_line("G1Y100"));
        assert!(!is_move_line("G28"));
        assert!(!is_move_line("; comment"));
    }

    #[test]
    fn test_translate_no_split() {
        let config = MotionConfig::default();
        let lines = ["G90", "G1 Y100 F300"];
        let out = translate_lines(&lines, &config);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], "G90");
        assert_eq!(out[1], "G1 Y100 F300");
    }

    #[test]
    fn test_translate_split_absolute() {
        let config = MotionConfig::default();
        // Y 700 exceeds limit 609.6
        let lines = ["G90", "G1 Y700 F300"];
        let out = translate_lines(&lines, &config);
        assert_eq!(out.len(), 3);
        assert_eq!(out[0], "G90");
        assert_eq!(out[1], "G1 Y609.6000 F300");
        assert!(out[2].starts_with("G1 A"));
        assert!(out[2].contains("90.4")); // 700 - 609.6
        assert!(out[2].contains("F300"));
    }

    #[test]
    fn test_translate_split_relative() {
        let config = MotionConfig::default();
        // Start at 600, move Y 50 relative -> 650, over limit
        let lines = ["G91", "G1 Y600 F200", "G1 Y50 F200"];
        let out = translate_lines(&lines, &config);
        assert_eq!(out.len(), 4);
        assert_eq!(out[0], "G91");
        assert_eq!(out[1], "G1 Y600 F200"); // under limit
        assert!(out[2].contains("9.6") && out[2].contains("F200")); // to limit (609.6 - 600)
        assert!(out[3].starts_with("G1 A"));
        assert!(out[3].contains("40.4")); // 50 - 9.6
    }

    #[test]
    fn test_translate_passthrough_non_move() {
        let config = MotionConfig::default();
        let lines = ["M3 S1000", "G0 X10", "G1 Y500 F300"];
        let out = translate_lines(&lines, &config);
        assert_eq!(out.len(), 3);
        assert_eq!(out[0], "M3 S1000");
        assert_eq!(out[1], "G0 X10");
        assert_eq!(out[2], "G1 Y500 F300");
    }
}
