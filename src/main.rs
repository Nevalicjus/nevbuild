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

fn c(file: &str) {
	let _ = Command::new("gcc").arg("-o").arg("out").arg(file).status().expect("");
	let _ = Command::new("./out").status().expect("");
	let _ = Command::new("rm").arg("out").status().expect("");
}

fn cpp(file: &str) {
	let _ = Command::new("g++").arg("-o").arg("out").arg(file).status().expect("");
	let _ = Command::new("./out").status().expect("");
	let _ = Command::new("rm").arg("out").status().expect("");	
}

fn elixir(file: &str) {
	let _ = Command::new("elixir").arg(file).status().expect("");
}

fn js(file: &str) {
	let _ = Command::new("node").arg(file).status().expect("");
}

fn kotlin(file: &str) {
	let _ = Command::new("kotlinc").arg(file).arg("-include-runtime").arg("-d").arg("out.jar").status().expect("");
	let _ = Command::new("java").arg("-jar").arg("out.jar").status().expect("");
	let _ = Command::new("rm").arg("out.jar").status().expect("");
}

fn lua(file: &str) {
	let _ = Command::new("lua").arg(file).status().expect("");
}

fn ruby(file: &str) {
	let _ = Command::new("ruby").arg(file).status().expect("");
}

fn rust(file: &str) {
	let _ = Command::new("rustc").arg(file).arg("-o").arg("out").status().expect("");
	let _ = Command::new("./out").status().expect("");
	let _ = Command::new("rm").arg("out").status().expect("");
}

fn swift(file: &str) {
	let _ = Command::new("swift").arg(file).status().expect("");
}

fn v(file: &str) {
	let _ = Command::new("v").arg("run").arg(file).status().expect("");
}

fn go(file: &str) {
	let _ = Command::new("go").arg("run").arg(file).status().expect("");
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
    	"c" => c(file),
    	"cpp" => cpp(file),
    	"exs" => elixir(file),
    	"java" => java(name, file),
    	"go" => go(file),
    	"js" | "ts" => js(file),
    	"kt" => kotlin(file),
    	"lua" => lua(file),
    	"py" => py(file),
    	"rb" => ruby(file),
    	"rs" => rust(file),
    	"swift" => swift(file),
    	"v" => v(file),
    	&_ => unsupported()
    }
}
