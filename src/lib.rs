pub struct Fibonacci {
    current: u64,
    next: u64
}
impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1
        }
    }
}
impl Iterator for Fibonacci {
    type Item=u64;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        self.current.into()
    }
}
