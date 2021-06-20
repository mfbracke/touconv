use crate::parsers::ElementEventHandler;

use super::common::{Event, ParseState};

pub mod ooxml;

pub trait Subparser {
    fn handle_xml_event(
        &self,
        event: &Event,
        parse_state: &ParseState,
        element_event_handler: &mut dyn ElementEventHandler,
    );
}
