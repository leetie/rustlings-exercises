// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

trait AppendBar {
    fn append_bar(&mut self) -> Self;
}

// kind of a hacky solution here; chose to change the function signature to allow for modifying self in place, function simply pushes String "Bar" onto self and returns self.
// NEVERMIND, the $ rustlings hint traits2 suggests 'mutating the incoming string vector' so I think I'm good...
impl AppendBar for Vec<String> {
    fn append_bar(&mut self) -> Self {
        self.push(String::from("Bar"));
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
