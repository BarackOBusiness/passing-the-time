mod references;
mod slices;
mod fizzbuss;
mod overloading;
mod implicit_conversions;
mod arrays_and_for_loops;

fn main() {
    println!("Hello, world!");

    references::main();
    slices::main();
    slices::string_slices();
    // fizzbuss::main(); // let's keep this out of the code for now, it takes a lot of space
    overloading::main();
    implicit_conversions::main();
    arrays_and_for_loops::main();
}
