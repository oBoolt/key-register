use std::{
    env, fs,
    io::{self, Read, Write},
    path, process,
};

use chrono::{DateTime, Utc};

fn main() {
    if let Err(e) = run() {
        println!("\x1b[31mError\x1b[m: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), &'static str> {
    let config = Config::build(env::args())?;

    if config.key_value == String::from("open") {
        if let Err(e) = show_storage_file(&config) {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    return Err(
                        "Storage file not found, use the first command to register the key value",
                    )
                }
                _ => return Err("Something gone wrong"),
            }
        }
    }

    let mut file = match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.file_path)
    {
        Ok(x) => x,
        Err(e) => {
            dbg!(&e);
            return Err("Something gone wrong");
        }
    };

    let formatted_date = config.date.format("%d/%b/%Y").to_string();
    let content = format!("{}: {}\r\n", formatted_date, config.key_value);
    match file.write(content.as_bytes()) {
        Ok(_bytes) => {
            println!("\x1b[32mSuccess\x1b[m: the value was registered");
        }
        Err(_e) => return Err("Something gone wrong"),
    };

    Ok(())
}

fn show_storage_file(config: &Config) -> io::Result<()> {
    let file = fs::File::open(&config.file_path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let lines: Vec<(usize, &str)> = content.split("\n").enumerate().collect();
    for (i, line) in lines {
        if line == "" {
            break;
        }
        println!("{:0width$} \x1b[2m|\x1b[0m {}", i + 1, line, width = 2)
    }

    Ok(())
}

#[derive(Debug)]
struct Config {
    file_path: path::PathBuf,
    key_value: String,
    date: DateTime<Utc>,
}

impl Config {
    fn build(args: env::Args) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("\x1b[31mError\x1b[m: you must provide the key value\r\n\x1b[36mUsage\x1b[m: keys <value | open>");
        }

        let file_path = match env::var("TF2_KEY_REGISTER_PATH") {
            Ok(x) => {
                let binding = path::Path::new(&x);
                binding.to_owned()
            }
            Err(_e) => {
                let mut path = env::current_dir().unwrap();
                path.push("keys");
                path.set_extension("txt");
                path
            }
        };

        let key_value = &args.collect::<Vec<_>>()[1];
        let now = Utc::now();

        Ok(Self {
            file_path,
            key_value: key_value.to_string(),
            date: now,
        })
    }
}
