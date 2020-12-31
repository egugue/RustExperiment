pub fn main() {
    dbg!();

    let arg1 = format!("aaa");
    let arg2 = format!("bbb");
    let longest_ref = longest(&arg1, &arg2);
    // can compile because each ref's lifetime is the same.
    dbg!(longest_ref);

    let outer_arg = format!("aaa");
    let outer_longest: &str;
    {
        let inner_arg: String = format!("a");
        outer_longest = longest(&outer_arg, &inner_arg);
        // can compile because the required lifetime bound is this inner block and all refs can live as long as or longer than the block.
        dbg!(outer_longest);
    }
    // won't compile because inner_arg cannot live as long as outer_longest ref.
    // dbg!(outer_longest);

    let outer_longest: &str;
    {
        let inner_arg1 = "a";
        let inner_arg2 = "bb";
        outer_longest = longest(&inner_arg1, &inner_arg2);
    }
    // can compile because each inner_arg is created with string literals which can live during running this program.
    dbg!(outer_longest);
    return;
}

fn dangling_reference() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // won't compile because x's lifetime is shorter than r so that reference r will be an invalid ref.
    // println!("r: {}", r);

    let r;
    let x = 5;
    r = &x;
    // can compile because r and x are the same lifetime
    println!("r: {}", r);
}

// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_string(x: String, y: String) -> String {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
