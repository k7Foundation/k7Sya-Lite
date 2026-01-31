mod int;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = String::new();
    let mut verbose = false;

    for arg in args.iter().skip(1) {
        if arg == "-v" {
            verbose = true;
        } else if arg.starts_with("file=") {
            file_path = arg.replace("file=", "");
        }
    }

    if file_path.is_empty() {
        println!("k7Sya Lite v0.1a\nUsage: .\\k7sya_lite.exe file=\"path\" [-v]");
        return;
    }

    let interpreter = int::Interpreter::new(&file_path, verbose);
    interpreter.run();
}