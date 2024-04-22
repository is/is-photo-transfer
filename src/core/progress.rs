pub struct Progress {
    pub total: usize,
    pub cur: usize,
}

impl Progress {
    pub fn new(total:usize) -> Progress {
        Progress {
            total: total,
            cur: 0
        }
    }

    pub fn inc(&mut self)  {
        self.cur += 1;
    }
}