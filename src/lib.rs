//! # References
//!
//! - The tutorial for the pin mechanism: <https://rust-lang.github.io/async-book/04_pinning/01_chapter.html>

use std::{marker::PhantomPinned, pin::Pin, ptr::null};

/// Playing with self-references and pin
#[derive(Debug)]
pub struct SelfRefer {
    /// a pointer to `v`
    ptr: *const usize,
    /// the value to be checked against to make sure `ptr` is properly set
    v: usize,
    /// make sure `v` won't be moved and thus `ptr` always valid
    _pin: PhantomPinned,
}
impl SelfRefer {
    /// Return a new instance without self-referencing yet
    pub fn new(v: usize) -> Self {
        Self {
            v,
            ptr: null(),
            _pin: PhantomPinned,
        }
    }

    /// Only build up a self reference after being pinned to make sure the reference points to the correct memory slot
    pub fn refer_self(self: Pin<&mut Self>) {
        // SAFETY: this `self` is not going to be swapped out of the current memory slot
        let this = unsafe { self.get_unchecked_mut() };

        this.ptr = &this.v as _;
    }

    /// Should be pinned before use to make sure the self reference is correct
    pub fn referred(self: Pin<&Self>) -> Option<usize> {
        unsafe { self.ptr.as_ref() }.copied()
    }

    pub fn set(&mut self, v: usize) {
        self.v = v;
    }

    /// Being pinned does not mean you can't change the value in-place.
    /// It is just that you can't move the whole memory slot elsewhere.
    pub fn pinned_set(self: Pin<&mut Self>, v: usize) {
        let this = unsafe { self.get_unchecked_mut() };

        this.v = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin() {
        let v = 42;
        let mut sr = SelfRefer::new(v);

        // Force `sr` to be moved
        let v_ptr = (&mut sr.v) as *mut usize;
        let mut sr = std::mem::replace(&mut sr, SelfRefer::new(420));
        assert_eq!(unsafe { v_ptr.read() }, 420);

        // SAFETY: `p` drops with `sr` and before that, `sr` never gets unpinned
        let mut p = unsafe { Pin::new_unchecked(&mut sr) };

        assert_eq!(p.as_ref().referred(), None);

        p.as_mut().refer_self();
        assert_eq!(p.as_ref().referred(), Some(v));
    }
}
