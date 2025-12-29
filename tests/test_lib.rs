use rust_test::{add, subtract};

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3, "Add 1 + 2. Ans=3");
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(1, 2), -1, "Substract 2 from 1. Ans=-1");
}