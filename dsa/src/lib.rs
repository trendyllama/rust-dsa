
pub mod node;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_subtract() {
        let result = subtract(2, 2);

        assert_eq!(result, 0)
    }
}
