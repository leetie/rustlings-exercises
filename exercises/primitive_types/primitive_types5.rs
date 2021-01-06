// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // destructuring cat tuple here

    println!("{} is {} years old.", name, age);
    println!("cat is {}", cat.0); // does not take ownership of cat
}
