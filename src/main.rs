use std::env;
use std::process::Command;
use std::ffi::OsStr;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

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
    let home = env::var_os("HOME").expect("Cant get user's home");
    let cffp = format!("{}/.nb.conf", home.to_str().unwrap());
    let cfyaml = Path::new(&cffp);
    let mut config: HashMap<String, String>;
    if cfyaml.exists() {
    	let cf = File::open(cffp).expect("Unable to open file");
		config = serde_yaml::from_reader(cf).expect("Failed to parse YAML");
    } else {
    	config = HashMap::new();
    }
    let defaults: HashMap<String, String> = HashMap::from([
    	("c".to_string(), "gcc -o out {file} && ./out && rm out".to_string()),
    	("cpp".to_string(), "g++ -o out {file} && ./out && rm out".to_string()),
    	("exs".to_string(), "elixir {file}".to_string()),
    	("go".to_string(), "go run {file}".to_string()),
    	("java".to_string(), "javac {file} && java {name} && rm {name}.class".to_string()),
    	("js".to_string(), "node {file}".to_string()),
    	("kt".to_string(), "kotlinc {file} -include-runtime -d out.jar && java -jar out.jar && rm out.jar".to_string()),
		("lua".to_string(), "lua {file}".to_string()),
		("py".to_string(), "python3 {file}".to_string()),
		("rb".to_string(), "ruby {file}".to_string()),
		("rs".to_string(), "rustc -o out {file} && ./out && rm out".to_string()),
		("swift".to_string(), "swift {file}".to_string()),
		("ts".to_string(), "node {file}".to_string()),
		("v".to_string(), "v run {file}".to_string()),
	]);
	for (ext, cmd) in defaults.iter() {
		if !config.contains_key(ext) {
			config.insert(ext.clone(), cmd.clone());
		}
	}
    let name = path.file_stem().and_then(OsStr::to_str).expect("");
    let ext = path.extension().and_then(OsStr::to_str).expect("");
    let file = &args[1];
    let cmd: String = config.get(ext)
    .cloned()
    .unwrap_or_default()
    .replace("{file}", file)
	.replace("{name}", name)
	.replace("{ext}", ext);
	if cmd.is_empty() {
		println!("This file type is currently unsupported :(");
		std::process::exit(1);
	}
	let pargs: String = args.iter().skip(2).cloned().collect::<Vec<String>>().join(" ");
    let status = Command::new("sh").arg("-c").arg(format!("{} {}", cmd, pargs)).status();
    match status {
        Ok(_) => {},
        Err(err) => {
            println!("Error executing command: {}", err);
        }
    }
}
