// this is a comment

/*
 * this is a multi-line block comment
 *
 */

// this is the main function
fn main() {
    // this is executed when compiled binary is called

    // Print text to console (io::stdout)
    println!("Hello World!");

    // Print to console w/no newline
    print!("hello\n");

    // Print to standard error (io::stderr) [similar eprint! to print!]
    eprintln!("error msg1");

    let x = 5 + 10;
    // '{}' will be replaced with args and stringified
    println!("The value of x is = {}", x);

    // positional argument patterns can be used as well
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments can specify formatting as well
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // formatting is also applied in brackets
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // right-align text with width specifier (here 5 spaces then a 1)
    println!("{number:>width$}", number=1, width=6);

    // character padding can also be applied, here to format an output with preceding 0's
    println!("{number:0>width$}", number=1, width=6);
}
