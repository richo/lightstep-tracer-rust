pub struct Span;

impl Span {
    pub fn begin(&self) {
        println!("Span begin: {}:{}", file!(), line!());
    }

    pub fn end(self) {
        println!("Span end: {}:{}", file!(), line!());
    }
}
