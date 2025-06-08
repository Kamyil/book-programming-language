use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Value {
    Integer(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nothing,
}

impl Value {
    fn to_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Nothing => false,
        }
    }
    fn to_f64(&self) -> f64 {
        match self {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => panic!("Cannot convert {:?} to number", self),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Nothing => "nothing".into(),
        }
    }
}

struct Interpreter {
    vars: HashMap<String, Value>,
    consts: HashSet<String>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            consts: HashSet::new(),
        }
    }

    fn parse_value(&self, token: &str) -> Value {
        let t = token.trim();
        // Bool & nothing
        if t == "true" {
            return Value::Bool(true);
        }
        if t == "false" {
            return Value::Bool(false);
        }
        if t == "nothing" {
            return Value::Nothing;
        }

        // Single-quoted string
        if t.starts_with('\'') && t.ends_with('\'') {
            return Value::Str(t[1..t.len() - 1].to_string());
        }

        // Double-quoted string → do {var} interpolation
        if t.starts_with('"') && t.ends_with('"') {
            let mut out = String::new();
            let inner = &t[1..t.len() - 1];
            let mut chars = inner.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '{' {
                    let mut name = String::new();
                    while let Some(&n) = chars.peek() {
                        chars.next();
                        if n == '}' {
                            break;
                        }
                        name.push(n);
                    }
                    let val = self
                        .vars
                        .get(name.trim())
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| format!("{{UNKNOWN:{}}}", name));
                    out.push_str(&val);
                } else {
                    out.push(c);
                }
            }
            return Value::Str(out);
        }

        // Numbers
        if let Ok(i) = t.parse::<i64>() {
            return Value::Integer(i);
        }
        if let Ok(f) = t.parse::<f64>() {
            return Value::Float(f);
        }

        panic!("Unrecognized literal: {}", t);
    }

    fn eval_condition(&self, cond: &str) -> bool {
        for clause in cond.split(" and ").map(str::trim) {
            if clause.contains(" is greater than ") {
                let parts: Vec<_> = clause.splitn(2, " is greater than ").collect();
                let lhs = self.vars.get(parts[0].trim()).expect("unknown var");
                let rhs = self.parse_value(parts[1]);
                if lhs.to_f64() <= rhs.to_f64() {
                    return false;
                }
            } else if clause.contains(" is less than ") {
                let parts: Vec<_> = clause.splitn(2, " is less than ").collect();
                let lhs = self.vars.get(parts[0].trim()).expect("unknown var");
                let rhs = self.parse_value(parts[1]);
                if lhs.to_f64() >= rhs.to_f64() {
                    return false;
                }
            } else if clause.contains(" is not ") {
                let parts: Vec<_> = clause.splitn(2, " is not ").collect();
                let lhs = self.vars.get(parts[0].trim()).expect("unknown var");
                let rhs = self.parse_value(parts[1]);
                if lhs.to_string() == rhs.to_string() {
                    return false;
                }
            } else if clause.contains(" is ") {
                let parts: Vec<_> = clause.splitn(2, " is ").collect();
                let lhs = self.vars.get(parts[0].trim()).expect("unknown var");
                let rhs = self.parse_value(parts[1]);
                if lhs.to_string() != rhs.to_string() {
                    return false;
                }
            } else {
                panic!("Cannot parse condition: {}", clause);
            }
        }
        true
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let lines: Vec<_> = stdin.lock().lines().flatten().collect();
        let mut idx = 0;
        while idx < lines.len() {
            idx = self.run_lines(&lines, idx);
        }
    }

    fn run_lines(&mut self, lines: &[String], i: usize) -> usize {
        let line = lines[i].trim();
        if line.starts_with("if ") {
            // find matching end/else-if/else at top level
            let mut depth = 1;
            let mut end_idx = i + 1;
            let mut else_ifs = vec![];
            let mut else_idx = None;
            for j in (i + 1)..lines.len() {
                let l = lines[j].trim();
                if l.starts_with("if ") {
                    depth += 1
                }
                if l == "end" {
                    depth -= 1
                }
                if depth == 1 && l.starts_with("else if ") {
                    else_ifs.push(j)
                }
                if depth == 1 && l == "else" {
                    else_idx = Some(j)
                }
                if depth == 0 {
                    end_idx = j;
                    break;
                }
            }

            // extract condition text
            let mut cond_txt = &line["if ".len()..];
            if cond_txt.ends_with(" then") {
                cond_txt = &cond_txt[..cond_txt.len() - 5];
            }
            let cond_txt = cond_txt.trim();

            // if
            if self.eval_condition(cond_txt) {
                let stop = else_ifs.first().cloned().or(else_idx).unwrap_or(end_idx);
                let mut k = i + 1;
                while k < stop {
                    k = self.run_lines(lines, k)
                }
                return end_idx + 1;
            }

            // else ifs
            for &ei in &else_ifs {
                let mut txt = &lines[ei].trim()["else if ".len()..];
                if txt.ends_with(" then") {
                    txt = &txt[..txt.len() - 5]
                }
                if self.eval_condition(txt.trim()) {
                    let stop = else_ifs
                        .iter()
                        .filter(|&&x| x > ei)
                        .cloned()
                        .min()
                        .or(else_idx)
                        .unwrap_or(end_idx);
                    let mut k = ei + 1;
                    while k < stop {
                        k = self.run_lines(lines, k)
                    }
                    return end_idx + 1;
                }
            }

            // else
            if let Some(ei) = else_idx {
                let mut k = ei + 1;
                while k < end_idx {
                    k = self.run_lines(lines, k)
                }
            }
            return end_idx + 1;
        }

        // single line
        self.run_line(line);
        i + 1
    }

    fn run_line(&mut self, line: &str) {
        if line.is_empty() || line.starts_with("//") {
            return;
        }

        if let Some(rest) = line.strip_prefix("let ") {
            let parts: Vec<_> = rest.splitn(2, " be ").collect();
            let name = parts[0].trim().to_string();
            let val = self.parse_value(parts[1]);
            self.vars.insert(name, val);
            return;
        }
        if let Some(rest) = line.strip_prefix("constant ") {
            let parts: Vec<_> = rest.splitn(2, " is ").collect();
            let name = parts[0].trim().to_string();
            let val = self.parse_value(parts[1]);
            self.consts.insert(name.clone());
            self.vars.insert(name, val);
            return;
        }
        if line.contains(" becomes ") {
            let parts: Vec<_> = line.splitn(2, " becomes ").collect();
            let name = parts[0].trim();
            if self.consts.contains(name) {
                panic!("Cannot reassign constant `{}`", name);
            }
            let val = self.parse_value(parts[1]);
            self.vars.insert(name.to_string(), val);
            return;
        }
        if let Some(arg) = line.strip_prefix("call print with ") {
            let tok = arg.trim();
            // if it's a literal string/number/etc, parse it,
            // otherwise look up a variable
            let v = if tok.starts_with('"') || tok.starts_with('\'') {
                self.parse_value(tok)
            } else {
                self.vars
                    .get(tok)
                    .unwrap_or_else(|| panic!("Unknown var `{}`", tok))
                    .clone()
            };
            println!("{}", v.to_string());
            return;
        }

        panic!("Unrecognized statement: {}", line);
    }
}

fn main() {
    println!("Book-lang REPL (vars, conditionals & interpolation). Ctrl-D to exit.");
    Interpreter::new().run();
}
