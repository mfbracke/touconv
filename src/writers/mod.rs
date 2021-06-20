use crate::events::ElementEvent;

pub mod stdout;

pub trait Writer {
    fn handle(&mut self, event: ElementEvent);
}
