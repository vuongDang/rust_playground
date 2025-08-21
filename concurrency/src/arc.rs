//! Arc implementation
//! Inspired from Rust Atomics and Locks

#![allow(dead_code)]

use std::{
    cell::UnsafeCell,
    mem::ManuallyDrop,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{fence, AtomicU32, Ordering},
};

#[derive(Debug)]
struct Arc<T> {
    // Using raw pointer because ArcData is not owned
    // by anything which is outside of Rust reference model
    data: Weak<T>,
}

// Reference to arc data which is not accounted when dropping
// the inner data. This is useful for cyclic data.
#[derive(Debug)]
struct Weak<T> {
    data: NonNull<ArcData<T>>,
}

#[derive(Debug)]
struct ArcData<T> {
    // Value stored in the Arc
    value: ManuallyDrop<UnsafeCell<T>>,
    // Counts the total number of ref (strong + weak) to this data
    // When reaching 0, the `ArcData` is dropped
    // Small trick is to count all "strong" ref as a single to avoid
    // incrementing twice (already counted in the `strong_ref_counter`
    // field)
    total_ref_counter: AtomicU32,
    // Counts the number of strong ref to this data
    // When reaching 0, the field `value` is deallocated
    strong_ref_counter: AtomicU32,
}

impl<T> Arc<T> {
    fn new(value: T) -> Self {
        let data = ArcData {
            value: ManuallyDrop::new(UnsafeCell::new(value)),
            // This counter starts at 2 because this represents:
            // - the counter that represents all `Arc` to this data
            // - the weak ref that is contained in this `Arc`
            total_ref_counter: AtomicU32::new(2),
            strong_ref_counter: AtomicU32::new(1),
        };
        Arc {
            data: Weak {
                data: NonNull::from(Box::leak(Box::new(data))),
            },
        }
    }

    // We need to make sure that only a single arc is left (including weak ref)
    // to allow a mutable reference to the inner value
    fn get_mut(&mut self) -> Option<&mut T> {
        // We know that there is at least this instance of Arc and the weak ref that is contained in this `Arc`
        if self.data().total_ref_counter.load(Ordering::Relaxed) == 2 {
            // The fence is used to make sure that any cross-variable
            // modifications that were made in other arc/weak
            // are taken into account in this thread
            fence(Ordering::Acquire);
            // Safety: this is guaranteed to be non-null and initialized
            // because the instance of Arc calling this function still
            // exists and that only one instance of arc exists so it's
            // an exclusive ref
            unsafe { Some(&mut *self.data().value.get()) }
        } else {
            None
        }
    }

    fn data(&self) -> &ArcData<T> {
        // Safety: this is guaranteed to be non-null
        // because the instance of Arc calling this function still
        // exists
        self.data.data()
    }

    // Downgrade an Arc to a Weak ref
    fn downgrade(&self) -> Weak<T> {
        let mut n = self.data().total_ref_counter.load(Ordering::Relaxed);
        loop {
            assert!(n < u32::MAX / 2);
            if let Err(new_n) = self.data().total_ref_counter.compare_exchange_weak(
                n,
                n + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                n = new_n;
                continue;
            }
            return Weak {
                data: self.data.data,
            };
        }
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        // Safety: this is guaranteed to be non-null because
        // the instance of Weak calling this function still exists
        unsafe { self.data.as_ref() }
    }

    // Upgrade a weak ref to a full arc when the data
    // has not been dropped yet, else returns None
    fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().strong_ref_counter.load(Ordering::Relaxed);
        loop {
            // We use a compare_exchange loop in the case where the `strong_ref_counter`
            // has been modified between the load and now
            if n == 0 {
                // There are no more strong ref to the data
                // Data has been dropped already
                return None;
            } else {
                assert!(n < u32::MAX / 2);
                if let Err(new_n) = self.data().strong_ref_counter.compare_exchange_weak(
                    n,
                    n + 1,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    n = new_n;
                    continue;
                } else {
                    return Some(Arc { data: self.clone() });
                }
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}
unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().strong_ref_counter.load(Ordering::Relaxed) > u32::MAX / 2 {
            panic!("Arc has too many clones");
        }
        self.data()
            .strong_ref_counter
            .fetch_add(1, Ordering::Relaxed);
        Arc {
            data: self.data.clone(),
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().total_ref_counter.load(Ordering::Relaxed) > u32::MAX / 2 {
            panic!("Weak has too many clones");
        }

        self.data()
            .total_ref_counter
            .fetch_add(1, Ordering::Relaxed);
        Self { data: self.data }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // If the `strong_ref_counter` reaches 0 we drop the inner arc_data
        if self
            .data()
            .strong_ref_counter
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            // We only need the acquire-release relationship when dropping
            // to make sure that no other instance of arc exists
            fence(Ordering::Acquire);

            // Drop the inner value
            // Safety: We know this is the last Arc, we can drop the inner value
            unsafe {
                let manual_drop = &mut self.data.data.as_mut().value;
                ManuallyDrop::drop(manual_drop);
            }
            // Remove the counter value representing the last `Arc` in the total counter
            print!(
                "{:?}",
                self.data().total_ref_counter.load(Ordering::Acquire)
            );
            self.data()
                .total_ref_counter
                .fetch_sub(1, Ordering::Relaxed);

            print!(
                "{:?}",
                self.data().total_ref_counter.load(Ordering::Acquire)
            );
        }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        // If the `total_ref_counter` reaches 0 we drop the `ArcData`
        if self
            .data()
            .total_ref_counter
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            // We only need the acquire-release relationship when dropping
            // to make sure that no other instance of arc exists
            fence(Ordering::Acquire);

            // Box::from_raw does not allocate new memory just take ownership of existing ones
            // rather than Box::new
            // Safety: `ArcData` existence is guaranteed by this instance of `Weak`
            unsafe {
                let data = Box::from_raw(self.data.as_ptr());
                drop(data);
            }
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: the raw pointer can be deref as it is guaranteed
        // to be non-null as long as the instance of Arc exists
        unsafe { &*self.data().value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::atomic::AtomicUsize, thread};

    #[test]
    fn test_arc_strong_ref() {
        let value: Vec<u8> = vec![1, 2, 3];
        let mut shared = Arc::new(value);
        let shared1 = shared.clone();
        let shared2 = shared.clone();
        thread::scope(|s| {
            s.spawn(|| {
                assert_eq!((*shared1).len(), 3);
            });
            s.spawn(|| {
                assert_eq!((*shared2).len(), 3);
            });
        });
        drop(shared1);
        drop(shared2);
        shared.get_mut().unwrap().push(4);
        let shared1 = shared.clone();
        let shared2 = shared.clone();
        thread::scope(|s| {
            s.spawn(|| {
                assert_eq!((*shared1).len(), 4);
            });
            s.spawn(|| {
                assert_eq!((*shared2).len(), 4);
            });
        });
    }

    #[test]
    fn test_arc_get_mut_only_happens_with_one_ref() {
        let value: Vec<u8> = Vec::new();
        let mut shared = Arc::new(value);
        assert!(shared.get_mut().is_some());
        let mut shared1 = shared.clone();
        assert!(shared1.get_mut().is_none());
        drop(shared);
        let weak1 = shared1.downgrade();
        assert!(shared1.get_mut().is_none());
        drop(weak1);
        assert!(shared1.get_mut().is_some());
    }

    #[test]
    fn test_arc_is_correctly_dropped() {
        static NB_DROP: AtomicUsize = AtomicUsize::new(0);
        struct DetectDrop {}
        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NB_DROP.fetch_add(1, Ordering::Release);
            }
        }

        let value = DetectDrop {};
        let shared = Arc::new(value);
        let shared1 = shared.clone();
        let weak1 = shared.downgrade();
        drop(shared);
        assert_eq!(NB_DROP.load(Ordering::Acquire), 0);
        let shared2 = weak1.upgrade();
        assert!(shared2.is_some());
        drop(shared1);
        assert_eq!(NB_DROP.load(Ordering::Acquire), 0);
        drop(shared2);
        assert_eq!(NB_DROP.load(Ordering::Acquire), 1);
        assert!(weak1.upgrade().is_none())
    }

    #[test]
    fn test_arc_weak_ref() {
        let value: Vec<u8> = vec![1, 2, 3];
        let mut shared = Arc::new(value);
        let weak1 = shared.downgrade();
        let weak2 = shared.downgrade();
        thread::scope(|s| {
            s.spawn(|| {
                let shared1 = weak1.upgrade().unwrap();
                assert_eq!((*shared1).len(), 3);
            });
            s.spawn(|| {
                let shared2 = weak2.upgrade().unwrap();
                assert_eq!((*shared2).len(), 3);
            });
        });
        drop(weak1);
        drop(weak2);
        shared.get_mut().unwrap().push(4);
        let weak1 = shared.downgrade();
        let weak2 = shared.downgrade();
        thread::scope(|s| {
            s.spawn(|| {
                let shared1 = weak1.upgrade().unwrap();
                assert_eq!((*shared1).len(), 4);
            });
            s.spawn(|| {
                let shared2 = weak2.upgrade().unwrap();
                assert_eq!((*shared2).len(), 4);
            });
        });
    }
}
