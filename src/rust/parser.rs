// parser.rs

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Content {
    pub title: String,
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Line {
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Segment {
    Plain(String),
    Annotated { base: String, reading: String },
}

// Recursive descent parser implementation with escape support
pub fn parse_problem(input: &str) -> Content {
    // Split the input into lines
    let mut lines_iter = input.lines();
    // Parse the title line
    let title_line = lines_iter.next().unwrap_or("");
    let title = if title_line.starts_with("#title") {
        title_line.trim_start_matches("#title").trim().to_string()
    } else {
        "".to_string()
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
    while pos < chars.len() {
        if chars[pos] == '\\' {
            // Escape sequence in plain text: add the next character as literal.
            let mut literal = String::new();
            pos += 1; // Skip the backslash
            if pos < chars.len() {
                literal.push(chars[pos]);
                pos += 1;
            }
            segments.push(Segment::Plain(literal));
        } else if chars[pos] == '(' {
            // Parse an annotated segment starting with '('
            let (annotated, new_pos) = parse_annotated(&chars, pos);
            segments.push(annotated);
            pos = new_pos;
        } else {
            // Parse plain text until the next '(', or until an escape sequence is encountered.
            let start = pos;
            let mut plain = String::new();
            while pos < chars.len() && chars[pos] != '(' {
                if chars[pos] == '\\' {
                    pos += 1; // Skip the backslash
                    if pos < chars.len() {
                        plain.push(chars[pos]);
                        pos += 1;
                    }
                } else {
                    plain.push(chars[pos]);
                    pos += 1;
                }
            }
            if !plain.is_empty() {
                segments.push(Segment::Plain(plain));
            }
        }
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