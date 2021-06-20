use crate::errors::ParseError;
use crate::events::ElementEvent;
use mockall::*;

pub mod xml;

#[automock]
pub trait ElementEventHandler {
    fn handle<'a>(&mut self, event: Result<ElementEvent<'a>, ParseError>);
}

pub trait Parser {
    /// A parser should only be used once, so it's moved into the parse method.
    fn parse(self: Box<Self>, event_handler: &mut dyn ElementEventHandler);
}
