use std::fs::File;
use std::io::{Read, Write};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        todo!()
    }

    let path = &args[1];
    let max_line = 10;
    match File::open(path) {
        Ok(f) => {
            print_head(f, max_line);
        }
        Err(_) => {}
    }
}

fn print_head<T: Read>(mut reader: T, max_line: usize) {
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
                if line_count == max_line {
                    break;
                }
            }
        }

        stdout
            .write_all(&buffer[..print_size])
            .expect("failed to write");

        if line_count == max_line {
            break;
        }
    }
}
