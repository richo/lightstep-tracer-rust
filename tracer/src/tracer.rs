use crate::Span;

pub struct Tracer;

// These are really just dumb stubs in order to see if the proc macros can be made to work.

impl Tracer {
    pub fn span() -> Span {
        Span
    }
}
