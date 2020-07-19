use super::*;

#[test]
fn can_make_cb() {
    let x = 42;
    let _ = CallbackWithReturn::make(&x, |x| x + 1);
}

#[test]
fn can_call_cb() {
    let x = 42;
    let cb = CallbackWithReturn::make(&x, |x| x + 1);
    assert_eq!(cb.call(), 42);
}
