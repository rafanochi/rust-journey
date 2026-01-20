use adder::add;

#[test]
fn it_adds() {
    let result = add(18, 18);
    assert_eq!(result, 36);
}
