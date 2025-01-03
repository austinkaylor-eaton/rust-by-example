/*
Printing is handled by a series of macros defined in std::fmt some of which include:
    - format!: write formatted text to String
    - print!: same as format! but the text is printed to the console (io::stdout).
    - println!: same as print! but a newline is appended.
    - eprint!: same as format! but the text is printed to the standard error (io::stderr).
    - eprintln!: same as eprint!but a newline is appended.
 */
/// Run all the different versions of formatted print
pub(crate) fn run() {
    // Print to console
    println!("Hello from the formatted_print module");

    // Basic formatting
    // In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
    // 'Brad' will be placed in place of the first `{}`
    // 'Mass' will be placed in place of the second `{}`.
    println!("{} is from {}", "Brad", "Mass");

    // Positional arguments start at 0 immediately after the format string
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Different formatting can be invoked by specifying the format character after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001

    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=2); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);
}