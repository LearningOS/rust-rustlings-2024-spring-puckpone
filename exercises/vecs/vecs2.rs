// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2; //v.iter_mut() 返回一个可变引用的迭代器，对每个元素进行操作,所以修改时要解引用
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}
/*
iter().map(...) 是对迭代器的方法调用，使用 map 方法对迭代器产生的每个元素进行转换。
|element| { ... } 是一个闭包（lambda 函数），表示对每个元素的操作。闭包接收一个参数 element，代表迭代器中的每个元素。闭包体 { ... } 中是对 element 执行的操作。

map(...).collect() 是将 map 方法生成的结果收集起来，形成一个新的 Vec<i32>。
collect() 是一个用于迭代器收集结果的方法，将迭代器产生的元素收集到一个新的容器中，这里是收集到一个新的 Vec<i32> 中。
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
