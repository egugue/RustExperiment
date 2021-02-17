use std::fs::File;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_line = 10;
    if args.len() <= 1 {
        todo!()
    }

    if args.len() == 2 {
        let path = &args[1];
        match File::open(path) {
            Ok(f) => {
                print_tail(f, max_line);
            }
            Err(_) => {
                todo!()
            }
        }
        return;
    }

    todo!();
}

fn calc_seek_from<T: Read + Seek>(reader: &mut T, max_line: usize, buffer: &mut [u8]) -> SeekFrom {
    let buf_size = buffer.len() as i64;
    let file_size = reader.seek(SeekFrom::End(0)).expect("failed to read") as i64;

    let mut read_line_count = 1;
    let mut i = 0;
    let mut read_bytes_count = 0;
    while read_line_count <= max_line {
        i += 1;
        let pos_from_end = i * buf_size;
        if pos_from_end - file_size > buf_size {
            break; // reached to head
        }
        let from = if pos_from_end > file_size {
            SeekFrom::Start(0)
        } else {
            SeekFrom::End(-pos_from_end)
        };
        reader.seek(from).expect("failed to seek");

        let size = reader.read(&mut buffer[..]).expect("failed to read");
        for b in buffer[..size].iter().rev() {
            if *b == b'\n' && read_bytes_count != 0 {
                read_line_count += 1;
                if read_line_count > max_line {
                    break;
                }
            }
            read_bytes_count += 1;
        }
    }

    SeekFrom::End(-read_bytes_count)
}

fn print_tail<T: Read + Seek>(mut reader: T, max_line: usize) {
    let mut buffer = [0; 1024 * 4];
    let seek_from = calc_seek_from(&mut reader, max_line, &mut buffer);

    let mut writer = BufWriter::new(io::stdout());
    reader.seek(seek_from).expect("failed to seek");
    loop {
        let size = reader.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }
        writer.write(&buffer[..size]).expect("failed to write");
    }

    writer.flush().expect("failed to write");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_calc_seek_from() {
        // buffer is smaller than file size
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n4"), 1, &mut [0; 1]),
            SeekFrom::End(-1)
        );
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n44"), 1, &mut [0; 1]),
            SeekFrom::End(-2)
        );
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n4"), 2, &mut [0; 1]),
            SeekFrom::End(-3)
        );

        // buffer is bigger than file size
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n4"), 1, &mut [0; 1024]),
            SeekFrom::End(-1)
        );
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n44"), 1, &mut [0; 1024]),
            SeekFrom::End(-2)
        );
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n4"), 2, &mut [0; 1024]),
            SeekFrom::End(-3)
        );
        assert_eq!(
            calc_seek_from(&mut Cursor::new("1\n2\n3\n4"), 100, &mut [0; 1024]),
            SeekFrom::End(-7)
        );
    }
}
