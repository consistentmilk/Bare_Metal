pub struct MyCalendar {
    buffer: std::collections::BTreeMap<i32, i32>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Self {
            buffer: std::collections::BTreeMap::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, last_end)) = self.buffer.range(..end).last() {
            if start < *last_end {
                return false;
            }
        }

        self.buffer.insert(start, end);

        true
    }
}
