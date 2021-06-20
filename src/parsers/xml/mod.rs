use self::common::*;
use self::subparsers::ooxml;
use self::subparsers::Subparser;
use crate::constants::xml::namespaces::*;
use crate::errors::ParseError;
use crate::parsers::ElementEventHandler;
use crate::parsers::Parser as Parse;
use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event as QxEvent;
use quick_xml::Error as QxError;
use quick_xml::Reader as QxReader;
use std::collections::HashMap;
use std::io::BufRead;
use std::str;

mod common;
mod subparsers;

/// Looks at the namespace of each tag and lets the relevant subparser handle it.
pub struct Parser<B: BufRead> {
    qx_reader: QxReader<B>,
    subparser_registry: SubparserRegistry,
    parse_state: ParseState,
}

impl<B: BufRead> Parser<B> {
    pub fn for_bufread(bufread: B) -> Parser<B> {
        Parser {
            qx_reader: QxReader::from_reader(bufread),
            subparser_registry: SubparserRegistry::new(),
            parse_state: ParseState::new(),
        }
    }

    fn to_tag(&self, namespace: &[u8], qx_event: &BytesStart) -> Result<Box<Tag>, QxError> {
        let decoder = self.qx_reader.decoder();
        let namespace = decoder.decode(namespace)?;
        let name = decoder.decode(qx_event.local_name())?;
        Ok(Box::new(Tag::new(namespace.to_string(), name.to_string())))
    }

    fn handle_opening_tag(
        &mut self,
        namespace: &[u8],
        start_bytes: &BytesStart,
        event_handler: &mut dyn ElementEventHandler,
    ) {
        match self.to_tag(namespace, start_bytes) {
            Ok(tag) => {
                if let Some(subparser) = self.subparser_registry.subparser_for(tag.namespace()) {
                    let event = Event::TagAboutToOpen(tag.as_ref());
                    subparser.handle_xml_event(&event, &self.parse_state, event_handler)
                }
                self.parse_state.open_tag(tag);
            }
            Err(error) => event_handler.handle(Err(ParseError::from(error))),
        }
    }

    fn handle_closing_tag(
        &mut self,
        end_bytes: &BytesEnd,
        event_handler: &mut dyn ElementEventHandler,
    ) {
        if let Some(tag) = self.parse_state.close_tag() {
            if let Some(subparser) = self.subparser_registry.subparser_for(tag.namespace()) {
                let event = Event::TagClosed(tag.as_ref());
                subparser.handle_xml_event(&event, &self.parse_state, event_handler)
            }
        } else {
            event_handler.handle(Err(ParseError::InvalidInput {
                explanation: format!(
                    "Found closing tag when no tags were open: {:?}",
                    self.qx_reader.decoder().decode(end_bytes)
                ),
            }))
        }
    }

    fn handle_text(&mut self, text_bytes: &BytesText, event_handler: &mut dyn ElementEventHandler) {
        match self.qx_reader.decoder().decode(text_bytes) {
            Ok(text) => {
                let subparser_registry = &mut self.subparser_registry;
                if let Some(subparser) = self
                    .parse_state
                    .last_opened_tag()
                    .and_then(|tag| subparser_registry.subparser_for(tag.namespace()))
                {
                    let event = Event::Text(text);
                    subparser.handle_xml_event(&event, &self.parse_state, event_handler)
                }
            }
            Err(err) => event_handler.handle(Err(ParseError::from(err))),
        }
    }
}

impl<B: BufRead> Parse for Parser<B> {
    fn parse(mut self: Box<Self>, event_handler: &mut dyn ElementEventHandler) {
        let mut name_buffer = Vec::new();
        let mut namespace_buffer = Vec::new();

        loop {
            match self
                .qx_reader
                .read_namespaced_event(&mut name_buffer, &mut namespace_buffer)
            {
                Ok((Some(namespace), QxEvent::Start(ref start_bytes))) => {
                    self.handle_opening_tag(namespace, start_bytes, event_handler)
                }
                Ok((_, QxEvent::End(ref end_bytes))) => {
                    self.handle_closing_tag(end_bytes, event_handler)
                }
                Ok((_, QxEvent::Text(ref text_bytes))) => {
                    self.handle_text(text_bytes, event_handler)
                }
                Ok((_, QxEvent::Eof)) => break,
                _ => (),
            }
        }
    }
}

struct SubparserRegistry {
    subparsers: HashMap<String, Box<dyn Subparser>>,
}

impl SubparserRegistry {
    fn new() -> SubparserRegistry {
        SubparserRegistry {
            subparsers: HashMap::new(),
        }
    }

    fn subparser_for(&mut self, namespace: &str) -> Option<&mut (dyn Subparser + 'static)> {
        if !self.subparsers.contains_key(namespace) {
            match self.new_subparser_for(namespace) {
                Some(parser) => {
                    self.subparsers.insert(String::from(namespace), parser);
                }
                None => (),
            }
        }
        self.subparsers.get_mut(namespace).map(|b| b.as_mut())
    }

    fn new_subparser_for(&self, namespace: &str) -> Option<Box<dyn Subparser>> {
        match namespace {
            WORD_2006 => Some(Box::new(ooxml::Parser::new())),
            _ => None,
        }
    }
}
