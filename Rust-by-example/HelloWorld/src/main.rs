/// Documentation comments are written like this
/// They are also written like this.

// Regular comments are written like this
/*  Block comments are written like this
and extend beyond a single line */

/**
 * This is on text formatting and printing
 * 
*/

// fn main() {
//     println!("Hello, world!");
//     println!("I'm a Rustacean!");


//     // You can manipulate expressions more easily with block comments
//     // than with line comments. Try deleting the comment delimiters
//     // to change the result:
//     let x = 5 + /* 90 + */ 5;
//     println!("Is `x` 10 or 100? x = {}", x);

//     // In general, the '{}' will be automatically replaced with any
//     // arguments. These will be stringified.
//     println!("--------------------");
//     println!("{} days", 31);

//     // Positional arguments can be used. Specifying an integer inside '{}'
//     // determines which additional argument will be replaced. Arguments start
//     // at 0 immediately after the format string.
//     println!("--- Positional arguments ---");
//     println!("{0}, this is {1}, this is {0}", "Alice", "Bob");

//     // As can be named arguments.
//     println!(" {subject} {verb} {object} ",
//         object="the lazy dog",
//         subject="the quick brown fox",
//         verb="jumps over");

//     // Different formatting can be invoked by specifying the format charcater after ':'
//     println!("Base 10: {} ", 69420);
//     println!("Base 2 (binary): {:b} ", 69420);
//     println!("Base 8 (octal): {:o} ", 69420);
//     println!("Base 16 (hex): {:x} ", 69420);
//     println!("Base 16 (hex): {:X} ", 69420);

//     // You can right-align text with a specified width. This will output "     1". (four white spaces and a "1", for a total width of 5)
//     println!(" {number:>5} ", number=1);

//     // You can even use named arguments in the format specifier by appending a '$' to the argument name.
//     println!(" {number:0>width$} ", number=1, width=5);

//     // Rust even checks to make sure the correct number of arguments are used.
//     println!("My name is {0}, {1} {0}", "Bond", "James");

//     // Only types that implement fmt:Display can be formatted with '{}'. User defined types require more complicated handling and do not implement fmt:Display by default.

//     // #[allow(dead_code)]
//     // struct Structure(i32);

//     // This will not compile because `Structure` does not implement `fmt::Display`.
//     // println!("This struct `{}` won't print...", Structure(3));

//     // For rust 1.58 nd above, you can directly capture the argument from a surrounding variable. Just like the above, this will output  "     1". (four white spaces and a "1", for a total width of 5)
//     let number: f64 = 1.0;
//     let width: usize = 5;
//     println!(" {number:>width$} ");
   
// }


/**  
     * Debugging Section
     * 
     * All types which want to use std::fmt formatting traits require an implementation to be printable. Automatic implementations are provide for many types such as in the std library. All others must be manually implemented somehow.
     * The fmt::Debug trait makes this very straight forward. All types can derive the fmt::Debug implementation. This is not true for fmt::Display which must be manually implemented.
     * 
    */

    // This structure cannot be printed with either `fmt::Display` or `fmt::Debug`.
    // struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation required to make this `struct` printable with `fmt::Debug`.
    // #[derive(Debug)]
    // struct DebugPrintable(i32);

    // // All std library types are automatically printable with {:?} too.

    // // Derive the 'fmt:Debug' implementation for 'Structure'. 'Structure' is a structure which contains a single 'i32'.
    // #[derive(Debug)]
    // struct Structure(i32);

    // // Put a 'Structure' inside of the structure 'Deep'. Make it printable also.
    // #[derive(Debug)]
    // struct Deep(Structure);


// fn main() {
//     // Printing with `{:?}` is similar to with `{}`.
//     println!(" {:?} months in a year. ", 12);
//     println!(" {:?} ", (3, 4));
//     println!("{1:?} {0:?} is the {actor:?} name.",
//              "Slater",
//              "Christian",
//              actor="actor's");
    
//     // 'Structure' is printable!
//     println!(" Now {:?} will print! ", Structure(3));

//     // The problem with 'derive' is there is no control over how the results look. What if I want this to show just a '7'?
//     println!(" Now {:?} will print print! ", Deep(Structure(7)));
// }

// So 'fmt::Debug' definitely makes this printable but sacrifices some elegance. Rust also provides pretty printing with [:#?].

// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

// fn main() {
//     let name = "Peter";
//     let age = 27;
//     let peter = Person { name, age };

//     // Pretty print
//     println!("{:#?}", peter);
// }



/**
 *  Display
 * 'fmt::Display' is similar to 'fmt::Debug' but it is much compact and clean and makes it possible to customize the output appearance. This is done by using the print marker {}.
 * 
 */

// Import (via 'use') the 'fmt' module to make it available.

// fn main() {
//     use std::fmt;
    
//     // Define a structure for which 'fmt::Display' will be implemented. This is a tuple struct named 'Structure' that contains an 'i32'.
//     struct Structure(i32);

//     // To use the '{}' marker, the trait 'fmt::Display' must be implemented manually for the type.
//     impl fmt::Display for Structure {
//         // This trait requires 'fmt' with this exact signature.
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             // Write strictly the first element into the supplied output stream: 'f'. Returns 'fmt::Result' which indicates whether the operation succeeded or failed. Note that 'write!' uses syntax which is very similar to 'println!'.
//             write!(f, "{}", self.0)
//         }
//     }

// }

// fmt::Display may be cleaner than fmt::Debug but this presents a problem for the std library. How should ambiguous types be displayed? For example, if the std library implemented a single style for all Vec<T>, what style should it be? Would it be either of these two?
// Vec<path>: /:/etc:/home/username:/bin (split on :)
// Vec<number>: 1,2,3 (split on ,)
// No, because there is no ideal style for all types and the std library doesn't presume to dictate one. fmt::Display is not implemented for Vec<T> or for any other generic containers. fmt::Debug must then be used for these generic cases.

// This is not a problem though because for any new container type which is not generic,fmt::Display can be implemented.


use std::fmt; // Import 'fmt'

// A structure holding two numbers. 'Debug' will be derived so the results can be contrasted with 'Display'.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use 'self.number to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}