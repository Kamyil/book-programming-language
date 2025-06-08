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

struct Interpreter {
    vars: HashMap<String, Value>,
    consts: HashSet<String>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            vars: HashMap::new(),
            consts: HashSet::new(),
        }
    }

    fn parse_value(&self, token: &str) -> Option<Value> {
        let t = token.trim();
        if t == "true" {
            Some(Value::Bool(true))
        } else if t == "false" {
            Some(Value::Bool(false))
        } else if t == "nothing" {
            Some(Value::Nothing)
        } else if t.starts_with('\'') && t.ends_with('\'') {
            Some(Value::Str(t[1..t.len() - 1].to_string()))
        } else if let Ok(i) = t.parse::<i64>() {
            Some(Value::Integer(i))
        } else if let Ok(f) = t.parse::<f64>() {
            Some(Value::Float(f))
        } else {
            None
        }
    }

    fn run_line(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            return;
        }

        // let foo be 42
        if let Some(rest) = line.strip_prefix("let ") {
            let parts: Vec<_> = rest.splitn(2, " be ").collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let val = self.parse_value(parts[1]).expect("invalid literal");
                if self.vars.contains_key(&name) {
                    panic!("Variable `{}` already declared", name);
                }
                self.vars.insert(name, val);
                return;
            }
        }

        // constant BAR is 'hello'
        if let Some(rest) = line.strip_prefix("constant ") {
            let parts: Vec<_> = rest.splitn(2, " is ").collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let val = self.parse_value(parts[1]).expect("invalid literal");
                if self.vars.contains_key(&name) {
                    panic!("Name `{}` already used", name);
                }
                self.vars.insert(name.clone(), val);
                self.consts.insert(name);
                return;
            }
        }

        // foo becomes 100
        if let Some(parts) = line.split_once(" becomes ") {
            let name = parts.0.trim();
            let val = self.parse_value(parts.1).expect("invalid literal");
            if !self.vars.contains_key(name) {
                panic!("Undefined variable `{}`", name);
            }
            if self.consts.contains(name) {
                panic!("Cannot reassign constant `{}`", name);
            }
            self.vars.insert(name.to_string(), val);
            return;
        }

        // call print with foo
        if let Some(rest) = line.strip_prefix("call print with ") {
            let arg = rest.trim();
            if let Some(v) = self.vars.get(arg) {
                println!("{:?}", v);
            } else {
                panic!("Undefined variable `{}`", arg);
            }
            return;
        }

        panic!("Unrecognized statement: {}", line);
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines().flatten() {
            self.run_line(&line);
        }
    }
}

fn main() {
    println!("Book-lang REPL (vars only). Enter lines, Ctrl+D to exit.");
    let mut interp = Interpreter::new();
    interp.run();
}
