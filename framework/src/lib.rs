pub fn foo(a: i32) -> i32 {
    println!("Doing foo...");
    a + 2
}

#[test]
fn it_works() {
    //assert!(false); // test fails
    assert_eq!(4, foo(2));
}
