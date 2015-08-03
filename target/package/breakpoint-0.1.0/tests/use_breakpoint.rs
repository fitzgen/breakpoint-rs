#![feature(asm)]
#[macro_use] extern crate breakpoint;

#[test]
fn test_that_we_can_use_the_breakpoint_macro_from_another_crate() {
    use breakpoint::tests;
    tests::reset();
    breakpoint!();
    assert_eq!(tests::hit_breakpoint(), true);
}
