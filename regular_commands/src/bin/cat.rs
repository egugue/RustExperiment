use std::fs::File;
use std::io::{Error, Read, Write};
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        write_to_stdout(io::stdin());
        return;
    }

    for path in &args[1..] {
        match File::open(path) {
            Ok(f) => write_to_stdout(f),
            Err(_) => {
                io::stderr()
                    .write_all(format!("cat: {} No such file or directory\n", path).as_ref());
                exit(1);
            }
        }
    }
}

fn write_to_stdout<T: Read>(mut reader: T) {
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024 * 4];
    loop {
        let size = reader.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }
        stdout.write_all(&buffer[..size]).expect("failed to write");
    }
}
