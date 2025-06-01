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
}
