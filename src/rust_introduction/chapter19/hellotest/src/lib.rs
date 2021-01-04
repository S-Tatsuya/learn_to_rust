#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
        assert_eq!(10, add(0, 10));
        assert_eq!(10, add(10, 0));
        assert_eq!(20, add(10, 10));
    }

	#[test]
	fn test_add_zero() {
		assert_eq!(0, add(0, 0));
	}
	#[test]
	fn test_add_under_zero() {
		assert_eq!(0, add(-1, -1));
	}
}

fn add(x: i32, y: i32) -> i32 {
	let ans = x + y;
	if ans < 0 {
		0
	} else {
		ans
	}
}
