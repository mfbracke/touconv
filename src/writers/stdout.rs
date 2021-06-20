use super::Writer as Write;
use crate::events::ElementEvent;

/// A writer that just prints the events to the standard output.
pub struct Writer {}

impl Writer {
    pub fn new() -> Writer {
        Writer {}
    }
}

impl Write for Writer {
    fn handle(&mut self, event: ElementEvent) {
        println!("{:?}", event);
    }
}
