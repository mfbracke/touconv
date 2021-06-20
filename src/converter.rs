use crate::errors::ArgsError;
use crate::errors::Error;
use crate::errors::ParseError;
use crate::events::ElementEvent;
use crate::parsers::xml::Parser as XmlParser;
use crate::parsers::ElementEventHandler as HandleElementEvent;
use crate::parsers::Parser;
use crate::writers::stdout::Writer as StdoutWriter;
use crate::writers::Writer;
use std::boxed::Box;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

pub struct Converter {
    parser: Box<dyn Parser>,
    event_handler: ElementEventHandler,
}

impl Converter {
    pub fn new(conf: &Conf) -> Result<Converter, ArgsError> {
        let file = File::open(conf.source_file_path())?;
        let bufreader = BufReader::new(file);
        Ok(Converter {
            parser: Converter::parser_for(conf.input_format(), bufreader)?,
            event_handler: ElementEventHandler {
                writer: Converter::writer_for(conf.output_format())?,
            },
        })
    }

    pub fn convert(mut self) {
        self.parser.parse(&mut self.event_handler)
    }

    fn parser_for<B: BufRead + 'static>(
        format: &str,
        bufread: B,
    ) -> Result<Box<dyn Parser>, ArgsError> {
        match format {
            "xml" => Ok(Box::new(XmlParser::for_bufread(bufread))),
            _ => Err(ArgsError::NoSuchParser),
        }
    }

    fn writer_for(format: &str) -> Result<Box<dyn Writer>, ArgsError> {
        match format {
            "stdout" => Ok(Box::new(StdoutWriter::new())),
            _ => Err(ArgsError::NoSuchWriter),
        }
    }
}

struct ElementEventHandler {
    writer: Box<dyn Writer>,
}

impl HandleElementEvent for ElementEventHandler {
    fn handle<'a>(&mut self, event: Result<ElementEvent<'a>, ParseError>) {
        match event {
            Ok(event) => self.writer.handle(event),
            Err(err) => eprintln!("{}", Error::Parse(err)),
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "touconv", about = "A general event-based converter.")]
pub struct Conf {
    #[structopt(short = "f", long = "from")]
    input_format: String,
    #[structopt(short = "t", long = "to")]
    output_format: String,
    #[structopt(parse(from_os_str))]
    source_file_path: PathBuf,
}

impl Conf {
    /// Read the configuration from the command line arguments.
    pub fn from_args() -> Conf {
        StructOpt::from_args()
    }

    pub fn input_format(&self) -> &str {
        &self.input_format
    }

    pub fn output_format(&self) -> &str {
        &self.output_format
    }

    pub fn source_file_path(&self) -> &PathBuf {
        &self.source_file_path
    }
}
