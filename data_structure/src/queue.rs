pub struct Queue<T> {
    data:Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        let d:Vec<T> = Vec::new();
        let queue_ = Queue {data : d};
        return queue_;
    }

    pub fn en_queue(&mut self, d:T) {
        self.data.insert(0, d);
    }

    pub fn de_queue(&mut self) -> T {
        let t = self.data.pop().unwrap();
        return t;
    }

    pub fn is_empty(&self) -> bool {
        if self.data.len() == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

}

impl Queue<i32> {
    pub fn summary(&self) {
        println!("[Summary] empty: {}, len: {}, data: {:?}", self.is_empty(), self.len(), self.data);
    }
}