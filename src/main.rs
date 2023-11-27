use std::env;
use std::process::Command;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: nb <file_path>");
        std::process::exit(1);
    }
    let mut path = env::current_dir().expect("");
    path.push(&args[1]);
    if !path.exists() {
    	println!("File has to exist!");
        std::process::exit(1);
    }
    let ext = path.extension().and_then(OsStr::to_str).expect("");
    match ext {
    	"py" => {
    		let _ = Command::new("python3").arg(&args[1]).status().expect("");
    	},
    	"java" => {
    		let file = path.file_stem().and_then(OsStr::to_str).expect("");
    		let _ = Command::new("javac").arg(&args[1]).status().expect("");
    		let _ = Command::new("java").arg(file).status().expect("");
    		let _ = Command::new("rm").arg(format!("{}.class", file)).status().expect("");
    	},
    	&_ => println!("This file type is currently not supported :(")
    }
}
