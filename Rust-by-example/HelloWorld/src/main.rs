/// Documentation comments are written like this
/// They are also written like this.

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");


    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // In general, the '{}' will be automatically replaced with any
    // arguments. These will be stringified.
    println!("--------------------");
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside '{}'
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("--- Positional arguments ---");
    println!("{0}, this is {1}, this is {0}", "Alice", "Bob");

    // As can be named arguments.
    println!(" {subject} {verb} {object} ",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // Different formatting can be invoked by specifying the format charcater after ':'
    println!("Base 10: {} ", 69420);
    println!("Base 2 (binary): {:b} ", 69420);
    println!("Base 8 (octal): {:o} ", 69420);
    println!("Base 16 (hex): {:x} ", 69420);
    println!("Base 16 (hex): {:X} ", 69420);

    // You can right-align text with a specified width. This will output "     1". (four white spaces and a "1", for a total width of 5)
    println!(" {number:>5} ", number=1);

    // You can even use named arguments in the format specifier by appending a '$' to the argument name.
    println!(" {number:0>width$} ", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt:Display can be formatted with '{}'. User defined types require more complicated handling and do not implement fmt:Display by default.

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement `fmt::Display`.
    // println!("This struct `{}` won't print...", Structure(3));

    // For rust 1.58 nd above, you can directly capture the argument from a surrounding variable. Just like the above, this will output  "     1". (four white spaces and a "1", for a total width of 5)
    let number: f64 = 1.0;
    let width: usize = 5;
    println!(" {number:>width$} ");
}

// Regular comments are written like this
/*  Block comments are written like this
and extend beyond a single line */




