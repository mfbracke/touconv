use crate::constants::xml::namespaces::WORD_2006;
use crate::events::ElementEvent::*;
use crate::parsers::xml::common::Tag;
use crate::parsers::xml::Event as XmlEvent;
use crate::parsers::xml::ParseState;
use crate::parsers::ElementEventHandler;
use crate::ElementStart;
use crate::ElementStartVariant::*;

use super::Subparser;

// Tags we process and for which we will thus need to send an End event when they close.
const PROCESSED_TAGS: [(&str, &str); 1] = [(WORD_2006, "p")];

/// A parser for the Open Office XML format (used by docx).
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    fn handle_text(
        &self,
        text: &str,
        parse_state: &ParseState,
        element_event_handler: &mut dyn ElementEventHandler,
    ) {
        if let Some(tag) = parse_state.last_opened_tag() {
            if tag.matches(WORD_2006, "t") {
                element_event_handler.handle(Ok(Text(text)))
            }
        }
    }

    fn handle_opening_tag(&self, tag: &Tag, element_event_handler: &mut dyn ElementEventHandler) {
        if tag.matches(WORD_2006, "p") {
            element_event_handler.handle(Ok(Start(ElementStart::simple(Paragraph))))
        }
    }

    fn handle_closing_tag(&self, tag: &Tag, element_event_handler: &mut dyn ElementEventHandler) {
        if PROCESSED_TAGS.contains(&(tag.namespace(), tag.name())) {
            element_event_handler.handle(Ok(End))
        }
    }
}

impl Subparser for Parser {
    fn handle_xml_event(
        &self,
        event: &XmlEvent,
        parse_state: &ParseState,
        element_event_handler: &mut dyn ElementEventHandler,
    ) {
        match event {
            XmlEvent::Text(text) => self.handle_text(text, parse_state, element_event_handler),
            XmlEvent::TagAboutToOpen(tag) => self.handle_opening_tag(tag, element_event_handler),
            XmlEvent::TagClosed(tag) => self.handle_closing_tag(tag, element_event_handler),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::MockElementEventHandler;
    use crate::ElementEvent;

    #[test]
    fn parser_sends_events() {
        assert_parser_sends_event(
            XmlEvent::Text("test"),
            ParseState::with_open_tags(vec![Tag::from_refs(WORD_2006, "t")]),
            vec![Text("test")],
        );
        assert_parser_sends_event(
            XmlEvent::Text("test"),
            ParseState::with_open_tags(vec![Tag::from_refs(WORD_2006, "p")]),
            vec![],
        );
        assert_parser_sends_event(
            XmlEvent::TagAboutToOpen(&Tag::from_refs(WORD_2006, "p")),
            ParseState::with_open_tags(vec![]),
            vec![Start(ElementStart::simple(Paragraph))],
        );
        assert_parser_sends_event(
            XmlEvent::TagAboutToOpen(&Tag::from_refs("unknown", "p")),
            ParseState::with_open_tags(vec![]),
            vec![],
        );
        assert_parser_sends_event(
            XmlEvent::TagAboutToOpen(&Tag::from_refs(WORD_2006, "unknown")),
            ParseState::with_open_tags(vec![]),
            vec![],
        );
        assert_parser_sends_event(
            XmlEvent::TagClosed(&Tag::from_refs(WORD_2006, "p")),
            ParseState::with_open_tags(vec![]),
            vec![End],
        );
        assert_parser_sends_event(
            XmlEvent::TagClosed(&Tag::from_refs("unknown", "p")),
            ParseState::with_open_tags(vec![]),
            vec![],
        );
        assert_parser_sends_event(
            XmlEvent::TagClosed(&Tag::from_refs(WORD_2006, "unknown")),
            ParseState::with_open_tags(vec![]),
            vec![],
        )
    }

    fn assert_parser_sends_event(
        xml_event: XmlEvent,
        parse_state: ParseState,
        expected_events: Vec<ElementEvent<'static>>,
    ) {
        let parser = Parser::new();
        let mut event_handler = MockElementEventHandler::new();

        for expected_event in expected_events {
            event_handler
                .expect_handle()
                .withf(move |event_result| {
                    if let Ok(event) = event_result {
                        event == &expected_event
                    } else {
                        false
                    }
                })
                .times(1)
                .return_const(());
        }

        parser.handle_xml_event(&xml_event, &parse_state, &mut event_handler)
    }
}
