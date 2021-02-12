use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        io::stderr()
            .write_all(format!("Specify only one file\n").as_ref())
            .ok();
        exit(1);
    }

    let path = &args[1];
    match File::open(path) {
        Ok(f) => {
            let line_count = foo(f);
            io::stdout()
                .write_all(format!("     {} {}\n", line_count, path).as_ref())
                .ok();
        }
        Err(_) => {
            io::stderr()
                .write_all(format!("wc: {}: No such file or directory\n", path).as_ref())
                .ok();
            exit(1);
        }
    }
}

fn foo(mut f: File) -> usize {
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024 * 4];
    let mut byte_count: usize = 0;
    let mut line_count: usize = 0;
    let mut word_count: usize = 0;

    loop {
        let size = f.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }

        for b in &buffer[..size] {
            if *b == b'\n' {
                line_count += 1;
            }
        }
    }

    line_count
}
