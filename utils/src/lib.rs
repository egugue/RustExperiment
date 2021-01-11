#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_export]
macro_rules! println_function_name {
    () => {{
        println!("----- fn: {} -----", $crate::function!())
    }}
}

#[macro_export]
macro_rules! println_file_name {
    () => {{
        println!("----------------------------------------------------------------");
        println!("----- File: {} -----", std::file!());
        println!("------------------------------=---------------------------------");
    }}
}
