pub mod _7;
pub mod _7_dir;

pub fn main() {
    _7_dir::code_in_dir::function_in_dir();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
