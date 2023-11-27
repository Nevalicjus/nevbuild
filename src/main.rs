use std::env;
use std::process::Command;
use std::ffi::OsStr;

fn py(file: &str) {
	let _ = Command::new("python3").arg(file).status().expect("");
}

fn java(name: &str, file: &str) {
	let _ = Command::new("javac").arg(file).status().expect("");
	let _ = Command::new("java").arg(name).status().expect("");
	let _ = Command::new("rm").arg(format!("{}.class", name)).status().expect("");
}

fn unsupported() {
	println!("This file type is currently unsupported :(")
}

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
    let name = path.file_stem().and_then(OsStr::to_str).expect("");
    let ext = path.extension().and_then(OsStr::to_str).expect("");
    let file = &args[1];
    match ext {
    	"py" => py(file),
    	"java" => java(name, file),
    	&_ => unsupported()
    }
}
