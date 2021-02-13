use std::fs::File;
use std::io::{Read, Write};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_size = 10;
    if args.len() <= 1 {
        print_head(io::stdin(), max_size)
    }

    if args.len() == 2 {
        let path = &args[1];
        print_file_head(path, max_size);
        return;
    }

    let mut i = 0;
    let paths = &args[1..];
    for path in paths {
        println!("==> {} <==", path);
        print_file_head(path, max_size);
        i += 1;
        if i != paths.len() {
            println!()
        }
    }
}

fn print_file_head(path: &str, max_count: usize) {
    match File::open(path) {
        Ok(f) => {
            print_head(f, max_count);
        }
        Err(_) => {}
    }
}

fn print_head<T: Read>(mut reader: T, max_count: usize) {
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024 * 4];
    let mut line_count = 0;
    loop {
        let size = reader.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }

        let mut print_size = 0;
        for b in &buffer[..size] {
            print_size += 1;

            if *b == b'\n' {
                line_count += 1;
                if line_count == max_count {
                    break;
                }
            }
        }

        stdout
            .write_all(&buffer[..print_size])
            .expect("failed to write");

        if line_count == max_count {
            break;
        }
    }
}
