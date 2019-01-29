
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn when_int_is_16_then_result_should_be_17() {
    assert_eq!(17, super::add_one(16));
}
