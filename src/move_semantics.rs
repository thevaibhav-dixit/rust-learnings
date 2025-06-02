use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Default)]
#[allow(dead_code)]
pub struct AddrTracker(Option<usize>);

impl AddrTracker {
    // If we haven't checked the addr of self yet, store the current
    // address. If we have, confirm that the current address is the same
    // as it was last time, or else panic.
    #[allow(dead_code)]
    fn check_for_move(&mut self) {
        let current_addr = self as *mut Self as usize;
        match self.0 {
            None => self.0 = Some(current_addr),
            Some(prev_addr) => assert_ne!(prev_addr, current_addr),
        }
    }
}

#[allow(dead_code)]
struct BoxType<'a> {
    data: Box<&'a str>,
}

#[derive(Default)]
#[allow(dead_code)]
struct AddrTrackerPinned {
    prev_addr: Option<usize>,
    // remove auto-implemented `Unpin` bound to mark this type as having some
    // address-sensitive state. This is essential for our expected pinning
    // guarantees to work, and is discussed more below.
    _pin: PhantomPinned,
}

impl AddrTrackerPinned {
    #[allow(dead_code)]
    fn check_for_move(self: Pin<&mut Self>) {
        let current_addr = &*self as *const Self as usize;
        match self.prev_addr {
            None => {
                // SAFETY: we do not move out of self
                // Getting the mut ref to Self because we need to fill in the value for prev_addr
                let self_data_mut = unsafe { self.get_unchecked_mut() };
                self_data_mut.prev_addr = Some(current_addr);
            }
            // current_addr is always same as the previous address because the Self is pinned.
            // Pinning means that the value can not be moved.
            // Moving a value means changing its location on the physical memory.
            // Reference to Self is always the same because we can't move it(Self ie pinned value)
            // to a new location in memory.
            Some(prev_addr) => assert_eq!(prev_addr, current_addr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn addr_tracker() {
        let mut tracker = AddrTracker::default();
        tracker.check_for_move();

        // Here we shadow the variable. This carries a semantic move, and may therefore also
        // come with a mechanical memory *move*
        let mut tracker = tracker;
        // we move tracker to new location in memory, thus changing its address

        tracker.check_for_move();
    }

    #[test]
    fn box_type() {
        let a = "hello".to_string();
        let box_type = BoxType { data: Box::new(&a) };
        let _new_box_type = box_type;
        // can't use box_type here, as it has been moved
        // let data = box_type.data;
        //
        std::mem::drop(a);
        // can't access data since the a is dropped and the ref &a become invalid
        // let val = _new_box_type.data;
    }

    #[test]
    fn addr_tracker_pinned() {
        use std::pin::pin;
        let tracker = AddrTrackerPinned::default();

        // 2. Pin the value by putting it behind a pinning pointer, thus putting
        // it into an address-sensitive state
        let mut ptr_to_pinned_tracker: Pin<&mut AddrTrackerPinned> = pin!(tracker);
        ptr_to_pinned_tracker.as_mut().check_for_move();

        // Trying to access `tracker` or pass `ptr_to_pinned_tracker` to anything that requires
        // mutable access to a non-pinned version of it will no longer compile

        // 3. We can now assume that the tracker value will never be moved, thus
        // this will never panic!
        ptr_to_pinned_tracker.as_mut().check_for_move();

        // The below operation can not be allowed because when we pin tracker we made sure that the
        // ref that points to tracker never changes. (&mut Ref) never changes. Note this is
        // different from the previous test where this operation was allowed. This is the feature
        // that Pin provides.
        // let new_tracker = tracker;
    }

    #[test]
    fn box_moves() {
        #[derive(Debug, Default)]
        struct Data<'a> {
            value: String,
            value_str: Option<&'a str>,
        }
        let value = "hello".to_string();
        let mut data = Data {
            value,
            value_str: None,
        };
        data.value_str = Some(&data.value);

        let boxed_ref = Box::new(&data);
        assert_eq!(boxed_ref.value_str, Some("hello"));

        let new_boxed_ref = boxed_ref;
        assert_eq!(new_boxed_ref.value_str, Some("hello"));

        // can not borrow data as mutable since it is also borrowed as immutable
        // std::mem::swap(&mut data, &mut Data::default());
    }

    #[test]
    fn pinned_self_referential_struct() {
        // use std::marker::PhantomPinned;
        use std::pin::Pin;
        use std::ptr::NonNull;
        /// This is a self-referential struct because `self.slice` points into `self.data`.
        struct Unmovable {
            /// Backing buffer.
            data: [u8; 64],
            /// Points at `self.data` which we know is itself non-null. Raw pointer because we can't do
            /// this with a normal reference.
            slice: NonNull<[u8]>,
            // _pin: PhantomPinned,
        }

        impl Unmovable {
            /// Creates a new `Unmovable`.
            ///
            /// To ensure the data doesn't move we place it on the heap behind a pinning Box.
            /// Note that the data is pinned, but the `Pin<Box<Self>>` which is pinning it can
            /// itself still be moved. This is important because it means we can return the pinning
            /// pointer from the function, which is itself a kind of move!
            fn new(i: u8) -> Pin<Box<Self>> {
                let res = Unmovable {
                    data: [i; 64],
                    // We only create the pointer once the data is in place
                    // otherwise it will have already moved before we even started.
                    slice: NonNull::from(&[]),
                    // _pin: PhantomPinned,
                };
                // First we put the data in a box, which will be its final resting place
                let mut boxed = Box::new(res);

                // Then we make the slice field point to the proper part of that boxed data.
                // From now on we need to make sure we don't move the boxed data.
                boxed.slice = NonNull::from(&boxed.data);

                // To do that, we pin the data in place by pointing to it with a pinning
                // (`Pin`-wrapped) pointer.
                //
                // `Box::into_pin` makes existing `Box` pin the data in-place without moving it,
                // so we can safely do this now *after* inserting the slice pointer above, but we have
                // to take care that we haven't performed any other semantic moves of `res` in between.
                let pin = Box::into_pin(boxed);

                // Now we can return the pinned (through a pinning Box) data
                pin
            }
        }

        let mut unmovable: Pin<Box<Unmovable>> = Unmovable::new(1);

        // The inner pointee `Unmovable` struct will now never be allowed to move.
        // Meanwhile, we are free to move the pointer around.
        // let mut still_unmoved = unmovable;
        // assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

        let mut new_unmovable = Unmovable::new(2);
        let prev_slice = new_unmovable.slice;

        std::mem::swap(&mut *new_unmovable, &mut *unmovable);
        let current_slice = new_unmovable.slice;

        // without PhantomPinned, this would panic because the address of `slice` would change
        // and this is how pinned is useful if the struct was PhantomPinned it would be !Unpin and
        // hence we could not have deref_mut which changes the inner values
        assert_eq!(prev_slice, current_slice);
    }
}
