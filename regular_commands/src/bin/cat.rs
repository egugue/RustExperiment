use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{env, io};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let path = args.get(1).expect("Specify at least one file");
    // let mut f = File::open(path).expect(format!("{}: No such file", path).as_str());

    write_to_stdout(io::stdin());
}

fn write_to_stdout<T: Read>(mut reader: T) {
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024 * 4];
    loop {
        let size = reader.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }
        stdout.write(&buffer[..size]).expect("failed to write");
    }
}
