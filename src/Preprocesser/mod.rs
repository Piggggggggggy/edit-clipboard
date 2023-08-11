pub trait Preprocesser {
    pub fn process(self, text: &mut String) {}
}

pub struct Processor(Vec<Box<dyn Preprocesser>>);
// define module basetrait
impl Processor {
    pub fn apply(self, text: &mut String) {
        for item in self.0 {
            *item.process(text);
        }
    }

    // Adds operation to processor stack
    pub fn add_op(&mut self, op: dyn Preprocesser) {
        self.0.push(Box::new(op));
    }
    // todo
    // fn order()
}
