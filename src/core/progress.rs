pub struct Progress {
    pub total: i32,
    pub cur: i32,
}

impl Progress {
    pub fn new(total:i32) -> Progress {
        Progress {
            total: total,
            cur: 0
        }
    }

    pub fn inc(&mut self)  {
        self.cur += 1;
    }
}