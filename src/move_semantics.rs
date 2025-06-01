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
}
