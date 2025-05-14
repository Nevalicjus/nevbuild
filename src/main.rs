use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs,
    path::Path,
    process::{exit, Command}
};
use serde::{Serialize, Deserialize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = env::var_os("HOME").expect("/home/root/");
    let conf: Config = Config::read_config(Path::new(&format!("{}/.nbconf", home.to_str().unwrap())));
    match args.len() {
        0 | 1 => {
            println!("Usage: nb <file>");
            exit(1);
        }
        2.. => {
            if args[1] == "-h" {
                print_help(&conf);
            } else {
                match eval_file(conf, &args) {
                    Ok(_) => {
                        exit(0);
                    }
                    Err(x) => {
                        println!("Error: {}", x);
                        exit(1);
                    }
                }
            }
        }
    }
}


fn print_help(conf: &Config) {
    let help_str: &'static str = "NevBuild (nb) v0.7.0 (c) Maciej Bromirski\n\
        ---\n\
        nb takes the file you give it, gets it file extension, and if it has a\n\
        command associated with it, it will run it (with substitutions: {{name}}, {{ext}}, {{file}})\n\
        \n\
        below you'll find the list of all configured associations\n\
        you can overwrite or make new associations with a yaml file at ~/.nb.conf\n\
    ";
    println!("{}", help_str);
    for (ext, cmd) in conf.langs.iter() {
        println!(" - .{} -> {}", ext, cmd);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    version: String,
    langs: HashMap<String, String>
}

impl Config {
    fn new() -> Config {
        return Config {
            version: String::from("0.0"),
            langs: HashMap::new()
        }
    }

    fn default() -> Config {
        return serde_yaml::from_str(include_str!("./../def_nb.conf")).unwrap();
    }

    fn from_file(path: &Path) -> Config {
        if path.exists() == false {
            return Config::new();
        } else {
            let f = fs::File::open(path).expect("Unable to open file");
            return serde_yaml::from_reader(f).expect("Failed to parse yaml");
        }
    }

    fn read_config(path: &Path) -> Config {
        let mut conf: Config = Config::from_file(path);
        let def_conf: Config = Config::default();
        for (ext, cmd) in def_conf.langs.iter() {
            if !conf.langs.contains_key(ext) {
                conf.langs.insert(ext.clone(), cmd.clone());
            }
        }
        return conf;
    }
}

fn eval_file(conf: Config, args: &Vec<String>) -> Result<String, String> {
    println!("nb: Evaluating file {}", args[1]);
    let path = Path::new(&args[1]);
    if !fs::exists(&path).unwrap() {
        return Err(String::from("File doesn't exist"));
    }
    let name = path.file_stem().and_then(OsStr::to_str).expect("");
    let ext = path.extension().and_then(OsStr::to_str).expect("");
    let file = &args[1];
    let cmd: String = conf.langs.get(ext)
        .cloned()
        .unwrap_or_default()
        .replace("{file}", file)
        .replace("{name}", name)
        .replace("{ext}", ext);
	    
	if cmd.is_empty() {
        return Err(String::from("This file type is currently unsupported"));
	}
	let pargs: String = args.iter().skip(2).cloned().collect::<Vec<String>>().join(" ");

    let status = Command::new("sh").arg("-c").arg(format!("{} {}", cmd, pargs)).status();
    match status {
        Ok(_) => {
            return Ok(String::new());
        },
        Err(err) => {
            println!("Error executing command: {}", err);
            exit(1);
        }
    }
}
