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

}

// Regular comments are written like this
/*  Block comments are written like this
and extend beyond a single line */




