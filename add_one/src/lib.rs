#[cfg(test)]
mod tests;

struct LinkedList<T> {
    next: Option<Box<Self>>,
    value: T,
    count: u32,
}

impl<T> LinkedList<T> {
    fn new(v: T) -> Self {
        Self {
            next: None,
            value: v,
            count: 1,
        }
    }

    fn push(&mut self, v: T) {
        let mut head: &Self = &self;
        while let Some(n) = &head.next {
            head = n;
        }

        let n = Box::new(Self::new(v));
        head.next = Some(n); // does not work


        self.count += 1;
    }

    fn recount_elements(&mut self) -> u32 {
        let mut count = 0;
        let mut head: &Self = &self;
        while let Some(n) = &head.next {
            count += 1;
            head = n;
        }

        self.count = count;
        self.count
    }
}
