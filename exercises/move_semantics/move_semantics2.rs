// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new(); // change to mut here, because we will be passing it into fill_vec as a mutable reference to change in place yet retain ownership

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    // modifying in place and not returning anything
    // let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec.to_vec()
}
