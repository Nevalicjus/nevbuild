use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::File,
    path::Path,
    process::{exit, Command}
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: nb <file_path>");
        exit(1);
    }
    let mut path = env::current_dir().expect("");
    path.push(&args[1]);
    if !path.exists() && args[1] != "-h" {
    	println!("File has to exist!");
        exit(1);
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

	if args.len() > 1 && args[1] == "-h" {
	    println!("NevBuild (nb) v0.5.0 (c) Maciej Bromirski\n---");
	    println!("nb takes the file you give it, gets it file extension, and if it has a");
	    println!("command associated with it, it will run it (with substitutions: {{name}}, {{ext}}, {{file}})\n");
	    println!("below you'll find the list of all configured associations,");
	    println!("and if you have overwritten any, you'll also see what the default for them was");
	    println!("you can overwrite or make new associations with a yaml file at ~/.nb.conf\n");
	    let l: usize = config.keys().map(String::len).max().unwrap();
	    let pad: String = " ".repeat(l);
	    let mut keys: Vec<&String> = config.keys().collect();
	    keys.sort();
        for ext in keys {
            let cmd: &String = config.get(ext).unwrap();
            match defaults.get(ext) {
                Some(d) => {
                    if cmd != d {
                        println!("{ext:<l$} -> {cmd}\n{pad} d: {d}");
                    } else {
                        println!("{ext:<l$} -> {cmd}");
                    }
                }
                None => {
                    println!("{ext:<l$} -> {cmd}");
                }
            }
        }
        exit(1);
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
		exit(1);
	}
	let pargs: String = args.iter().skip(2).cloned().collect::<Vec<String>>().join(" ");
    let status = Command::new("sh").arg("-c").arg(format!("{} {}", cmd, pargs)).status();
    match status {
        Ok(_) => {},
        Err(err) => {
            println!("Error executing command: {}", err);
        }
    }
    exit(0);
}
