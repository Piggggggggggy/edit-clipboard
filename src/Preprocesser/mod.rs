pub trait Preprocesser {
    fn process(&self, text: &mut String) {}
}

pub struct Processor(Vec<Box<dyn Preprocesser>>);
// define module basetrait
impl Processor {
    pub fn new() -> Self {
        Processor(Vec::new())
    }
    pub fn apply(self, text: &mut String) {
        for item in self.0 {
            item.process(text);
        }
    }

    // Adds operation to processor stack
    pub fn add_op(&mut self, op: Box<dyn Preprocesser>) {
        self.0.push(op);
    }
    // todo
    // fn order()
}
