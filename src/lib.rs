//! Set breakpoints from within the comfort of your editor.
//!
//! [YOU MAY WANT TO USE THIS INSTEAD](http://doc.rust-lang.org/std/intrinsics/fn.breakpoint.html)
//!
//! ## Usage
//!
//! Import the macros from the `breakpoint` crate. Unfortunately, the
//! `breakpoint!()` macro relies on the `asm!()` macro, so we need
//! `#![feature(asm)]` as well.
//!
//! ```rust
//! #![feature(asm)]
//! #[macro_use] extern crate breakpoint;
//! # fn main() { }
//! ```
//!
//! Then, anywhere you would like to pause at in your debugger, add the
//! following line:
//!
//! ```rust
//! # #![feature(asm)]
//! # #[macro_use] extern crate breakpoint;
//! # fn main() {
//! # if false {
//! // Always pause.
//! breakpoint!();
//! # }
//!
//! // Only pause if `condition` is true.
//! # let condition = false;
//! breakpoint!(condition);
//! # }
//! ```
//!
//! ## Example
//!
//! Imagine we have this really tricky function that needs to be debugged,
//! because we are seeing integer underflows:
//!
//! ```rust
//! fn tricky_function_must_be_debugged(val: usize) -> usize {
//!     val - 1
//! }
//! ```
//!
//! We can set a breakpoint in the program from the comfort of our editor,
//! rebuild, and run under a debugger to see where things are going wrong:
//!
//! ```rust
//! # #![feature(asm)]
//! # #[macro_use] extern crate breakpoint;
//! # fn main() { }
//!
//! fn tricky_function_must_be_debugged(val: usize) -> usize {
//!     // Set a breakpoint before the underflow, so we can debug!
//!     breakpoint!();
//!     val - 1
//! }
//! ```
//!
//! If the  problematic function is  only called  a handful of  times, congrats!
//! You've uncovered the root cause of the bug by now!
//!
//! If, however, the tricky function is called many times, and the bug only
//! manifests after many calls, it is useful to break only if some condition
//! evaluates to true:
//!
//! ```rust
//! # #![feature(asm)]
//! # #[macro_use] extern crate breakpoint;
//! # fn main() { }
//!
//! fn tricky_function_must_be_debugged(val: usize) -> usize {
//!     // Only break if we are going to underflow!
//!     breakpoint!(val == 0);
//!     val - 1
//! }
//! ```
//!
//! ## Why?
//!
//! It can be convenient. Especially when you're already in your editor, you
//! can't remember your debugger's incantation for conditional breakpoints
//! (often made worse by poor support for parsing Rust expressions in current
//! versions of debuggers), and/or your crate isn't super big so rebuilding is
//! fast.
//!
//! In particular, I got annoyed that panics from failing tests didn't
//! automatically pause my debugger, and could never remember the incantation
//! for breaking on panic off the top of my head. I find this easier than that,
//! most of the time.
//!
//! Admittedly, `breakpoint!()` is far from perfect. These things tend to work
//! better in dynamic languages where re-evaluating a function is super easy and
//! doesn't need a full recompilation.

#![deny(missing_docs)]
#![feature(asm)]

extern crate libc;

#[macro_export]

/// Set a breakpoint.
///
/// See the [module-level documentation](./index.html).
macro_rules! breakpoint {
    () => {
        unsafe {
            asm!("int3"::);
            // For whatever reason, debuggers will show the paused line as the
            // next one after the int3, so we have this dummy line here for
            // ergonomics.
            let _ = 1;
        }
    };
    ($e:expr) => {
        if $e {
            breakpoint!();
        }
    };
}

#[doc(hidden)]
pub mod tests {
    use std::cell::Cell;
    use libc::types::os::arch::c95::c_int;

    thread_local!(static HIT_BREAKPOINT : Cell<bool> = Cell::new(false));

    #[repr(C)]
    type sig_t = extern "C" fn(c_int);

    extern "C" {
        fn signal(sig: c_int, func: sig_t) -> sig_t;
    }

    static SIGTRAP : c_int = 5;

    extern "C" fn sigtrap_handler(sig: c_int) {
        assert_eq!(sig, SIGTRAP);
        HIT_BREAKPOINT.with(|v| v.set(true));
    }

    pub fn reset() {
        HIT_BREAKPOINT.with(|v| v.set(false));
        unsafe {
            signal(SIGTRAP, sigtrap_handler);
        }
    }

    pub fn hit_breakpoint() -> bool {
        HIT_BREAKPOINT.with(|v| v.get())
    }

    #[test]
    fn test_that_we_can_use_the_breakpoint_macro() {
        reset();
        breakpoint!();
        assert_eq!(hit_breakpoint(), true);
    }

    #[test]
    fn test_condition() {
        reset();

        let x = 0;

        breakpoint!(x == 1);
        assert_eq!(hit_breakpoint(), false);

        breakpoint!(x == 0);
        assert_eq!(hit_breakpoint(), true);
    }
}
