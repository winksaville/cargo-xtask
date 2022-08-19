fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("add: 1 + 2 = {}", add(1,2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(add(1, 1), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(add(1, 1), 1);
    }
}
