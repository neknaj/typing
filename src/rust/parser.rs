// parser.rs

use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Content {
    pub title: Line,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
pub enum Segment {
    Plain { text: String },
    Annotated { base: String, reading: String },
}


// Implement the Display trait for Line.
// We iterate over all segments and output each one.
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.segments {
            write!(f, "{}", segment)?;
        }
        Ok(())
    }
}


// Implement the Display trait for Segment.
// When formatting a Segment, if it is Annotated, only the 'base' is printed.
impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Plain { text } => {
                // Print the text for the Plain variant.
                write!(f, "{}", text)
            }
            Segment::Annotated { base, reading: _ } => {
                // Print only the base for the Annotated variant.
                write!(f, "{}", base)
            }
        }
    }
}


// Recursive descent parser implementation with escape support
pub fn parse_problem(input: &str) -> Content {
    // Split the input into lines
    let mut lines_iter = input.lines();
    // Parse the title line
    let title_line = lines_iter.next().unwrap_or("");
    let title = if title_line.starts_with("#title") {
        Line {
            segments: parse_line(&title_line.trim_start_matches("#title").trim().to_string()),
        }
    } else {
        Line {
            segments: Vec::new(),
        }
    };

    // Parse the remaining lines into Line structures
    let mut lines = Vec::new();
    for line in lines_iter {
        if line.trim().is_empty() {
            continue;
        }
        let segments = parse_line(line);
        lines.push(Line { segments });
    }
    Content { title, lines }
}

fn parse_line(line: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut pos = 0;
    let mut plain = String::new(); // Accumulate plain text

    while pos < chars.len() {
        match chars[pos] {
            '\\' => {
                // Escape sequence in plain text: add the next character as literal.
                pos += 1; // Skip the backslash
                if pos < chars.len() {
                    plain.push(chars[pos]);
                    pos += 1;
                }
            }
            '(' => {
                // Before parsing annotated segment, push any accumulated plain text
                if !plain.is_empty() {
                    segments.push(Segment::Plain { text: plain.clone() });
                    plain.clear();
                }
                // Parse an annotated segment starting with '('
                let (annotated, new_pos) = parse_annotated(&chars, pos);
                segments.push(annotated);
                pos = new_pos;
            }
            '/' => {
                // Slash in plain text acts as a delimiter.
                if !plain.is_empty() {
                    segments.push(Segment::Plain { text: plain.clone() });
                    plain.clear();
                }
                pos += 1; // Skip the slash delimiter.
            }
            ch => {
                plain.push(ch);
                pos += 1;
            }
        }
    }
    // Push any remaining plain text after loop finishes.
    if !plain.is_empty() {
        segments.push(Segment::Plain { text: plain });
    }
    segments
}

fn parse_annotated(chars: &Vec<char>, start: usize) -> (Segment, usize) {
    // We assume the character at `start` is '('.
    let mut pos = start + 1; // Skip '('
    let mut base = String::new();
    // Collect characters for the base until '/' is found.
    while pos < chars.len() && chars[pos] != '/' {
        if chars[pos] == '\\' {
            pos += 1; // Skip the backslash
            if pos < chars.len() {
                base.push(chars[pos]);
                pos += 1;
            }
        } else {
            // If a closing ')' is found unexpectedly, break out.
            if chars[pos] == ')' {
                break;
            }
            base.push(chars[pos]);
            pos += 1;
        }
    }
    // Skip the '/' character if present.
    if pos < chars.len() && chars[pos] == '/' {
        pos += 1;
    }
    let mut reading = String::new();
    // Collect characters for the reading until a ')' is encountered.
    while pos < chars.len() && chars[pos] != ')' {
        if chars[pos] == '\\' {
            pos += 1; // Skip the backslash
            if pos < chars.len() {
                reading.push(chars[pos]);
                pos += 1;
            }
        } else {
            reading.push(chars[pos]);
            pos += 1;
        }
    }
    // Skip the closing ')'
    if pos < chars.len() && chars[pos] == ')' {
        pos += 1;
    }
    (Segment::Annotated { base, reading }, pos)
}
