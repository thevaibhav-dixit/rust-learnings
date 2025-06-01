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
}
