use super::*;

#[test]
fn test_new() {
    let s = LinkedList::new(5);
    assert_eq!(5, s.value);
    assert_eq!(1, s.count);
}

#[test]
fn test_push() {
    let mut s = LinkedList::new(5);
    s.push(6);
    assert_eq!(2, s.count);

    s.push(7);
    assert_eq!(2, s.count);
}

#[test]
fn test_recount_elements() {
    let mut s = LinkedList::new(5);
    s.count  = 0;
    assert_eq!(1, s.recount_elements());
}