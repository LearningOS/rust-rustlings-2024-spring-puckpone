// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self{ //Rust 允许在 impl 中对 trait 方法进行实现时使用不同的 self 参数形式，只要它们符合 trait 中相应方法的要求。
        self.push(String::from("Bar"));
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar")); //pop弹出最后一个元素
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
