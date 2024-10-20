pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
// ANCHOR_END: here
