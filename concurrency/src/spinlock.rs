//! Spinlock implementation
//! Inspired from Rust Atomics and Locks

#![allow(dead_code)]
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

struct SpinLock<T> {
    is_locked: AtomicBool,
    content: UnsafeCell<T>,
}

struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// Spinlock should be send but not sync
// T has to be Send to be accessible in other threads
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    fn new(content: T) -> Self {
        SpinLock {
            is_locked: AtomicBool::new(false),
            content: UnsafeCell::new(content),
        }
    }

    fn lock<'a>(&'a self) -> SpinGuard<'a, T> {
        // Acquire ordering to establish happen-before relationship with the drop
        while self.is_locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        SpinGuard { lock: self }
    }
}

impl<'a, T> Drop for SpinGuard<'a, T> {
    fn drop(&mut self) {
        // Acquire ordering to establish happen-before relationship with the lock
        self.lock.is_locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: Existence of the spinguard is guaranteed to be unique
        unsafe { &*self.lock.content.get() }
    }
}

impl<'a, T> DerefMut for SpinGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.content.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn spinlock_test() {
        let size = 1000;
        let part1 = Vec::from_iter(0..size);
        let part2 = Vec::from_iter(size..size + size);
        let mut expected1 = Vec::new();
        expected1.extend(part2.clone());
        expected1.extend(part1.clone());
        let mut expected2 = Vec::new();
        expected2.extend(part1.clone());
        expected2.extend(part2.clone());

        for _ in 0..10 {
            let lock = SpinLock::new(Vec::new());
            thread::scope(|s| {
                s.spawn(|| {
                    let mut guard = lock.lock();
                    for i in 0..size {
                        guard.push(i);
                    }
                });
                s.spawn(|| {
                    let mut guard = lock.lock();
                    for i in size..size + size {
                        guard.push(i);
                    }
                });
            });
            let value = lock.lock();

            assert!(*value == expected1 || *value == expected2);
        }
    }
}
