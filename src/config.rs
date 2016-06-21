use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use serde::{Deserialize, Deserializer};


#[derive(Deserialize, Debug)]
pub struct TwitterConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
    pub screen_name: String,
}

pub type AliasTable = ::toml::Table;

trait MyDeserialize : Sized {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer;
}

impl MyDeserialize for AliasTable {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let deserialized : ::toml::Table = try!(Deserialize::deserialize(de));
        Ok( deserialized )
    }
}

#[derive(Deserialize, Debug)]
pub struct TwatterConfig {
    pub twitter: TwitterConfig,
    #[serde(deserialize_with="MyDeserialize::deserialize")]
    pub aliases: AliasTable,
}

quick_error! {
    #[derive(Debug)]
    pub enum TwatterConfigError {
        Io(err: ::std::io::Error) {
            from()
            //XXX add context
            display( "twatter config io error [{}]", err )
        }
        Decode(err: ::toml::DecodeError) {
            from()
            display( "twatter config parse error [{}]", err )
            cause( err )
        }
        Parse(err: String) {
            from()
            description( err )
            display( "{}", err )
        }
    }
}

impl TwatterConfig {
    pub fn parse(path: &str) -> Result<Self, TwatterConfigError> {
        let mut config_toml = String::new();

        let mut file = try!( File::open(path) );

        try!( file.read_to_string(&mut config_toml) );

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            let mut errors = "".to_string();
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                errors.push_str(
                    format!(
                        "{}:{}:{}-{}:{} error: {}\n",
                        path, loline, locol, hiline, hicol, err.desc
                    ).as_str()
                );
            }
        }

        let mut config = Decoder::new( Value::Table( toml.unwrap() ) );

        Ok(try!(Deserialize::deserialize( &mut config )))
    }
}

