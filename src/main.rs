use touconv::Converter;
use touconv::ConverterConf;
use touconv::Error;

fn main() {
    let conf = ConverterConf::from_args();
    match Converter::new(&conf) {
        Ok(converter) => converter.convert(),
        Err(err) => eprintln!("{}", Error::Args(err)),
    }
}
