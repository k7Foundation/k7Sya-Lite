use std::fs;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Interpreter {
    pub file_path: String,
    pub verbose: bool,
    variables: RefCell<HashMap<String, (String, String)>>,
}

impl Interpreter {
    pub fn new(path: &str, verbose: bool) -> Self {
        Self {
            file_path: path.to_string(),
            verbose,
            variables: RefCell::new(HashMap::new()),
        }
    }

    pub fn run(&self) {
        if self.verbose { println!("  [DEBUG] Reading: {}", self.file_path); }

        match fs::read_to_string(&self.file_path) {
            Ok(content) => self.parse(&content),
            Err(e) => println!("  [!] Error reading file: {}", e),
        }
    }

    fn evaluate(&self, expr: &str) -> String {
        let mut result = expr.trim().to_string();
        let vars = self.variables.borrow();

        // Подстановка переменных
        for (name, (_, value)) in vars.iter() {
            let clean_val = value.replace("\"", "");
            result = result.replace(name, &clean_val);
        }

        // Обработка конкатенации ~
        if result.contains('~') {
            return result.split('~')
                .map(|s| s.trim().replace("\"", ""))
                .collect::<Vec<String>>()
                .join("");
        }

        // Базовая математика +
        if result.contains('+') {
            let sum: i32 = result.split('+')
                .filter_map(|p| p.trim().parse::<i32>().ok())
                .sum();
            return sum.to_string();
        }

        result.replace("\"", "")
    }

    fn parse(&self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") { continue; }

            if trimmed.starts_with("var ") {
                let parts: Vec<&str> = trimmed.split('=').collect();
                if parts.len() == 2 {
                    let head = parts[0].replace("var", "");
                    let define: Vec<&str> = head.split(':').collect();
                    if define.len() == 2 {
                        let name = define[0].trim().to_string();
                        let d_type = define[1].trim().to_string();
                        let val = self.evaluate(parts[1]);
                        
                        if self.verbose { println!("  [MEM] {} : {} = {}", name, d_type, val); }
                        self.variables.borrow_mut().insert(name, (d_type, val));
                    }
                }
            } else if trimmed.starts_with("println(") {
                let inner = trimmed.trim_start_matches("println(").trim_end_matches(')');
                println!("{}", self.evaluate(inner));
            }
        }
    }
}