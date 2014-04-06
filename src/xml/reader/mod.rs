//! Contains high-level interface for a pull-based XML parser.
//!
//! The most important type in this module is `EventReader`, which provides an iterator
//! view for events in XML document.

use std::io::Buffer;
use std::io::{MemReader, BufReader};

use events;
use events::XmlEvent;
use self::parser::PullParser;

pub use self::config::ParserConfig;

pub mod lexer;
pub mod parser;
pub mod config;

/// Simple wrapper around an `std::io::Buffer` which provides pull-based XML parsing.
pub struct EventReader<B> {
    priv source: B,
    priv parser: PullParser
}

impl<B: Buffer> EventReader<B> {
    /// Creates a new parser, consuming given `Buffer`.
    #[inline]
    pub fn new(source: B) -> EventReader<B> {
        EventReader::new_with_config(source, ParserConfig::new())
    }

    /// Creates a new parser with the provded configuration, consuming given `Buffer`.
    #[inline]
    pub fn new_with_config(source: B, config: ParserConfig) -> EventReader<B> {
        EventReader { source: source, parser: parser::new(config) }
    }

    /// Pulls and returns next XML event from the stream.
    ///
    /// If returned event is `xml::event::Error` or `xml::event::EndDocument`, then 
    /// further calls to this method will return this event again.
    #[inline]
    pub fn next(&mut self) -> XmlEvent { 
        self.parser.next(&mut self.source)
    }

    /// Returns an iterator over XML events.
    ///
    /// When the next event is `xml::event::Error` or `xml::event::EndDocument`, then
    /// it will be returned by the iterator once, and then it will stop producing events.
    #[inline]
    pub fn events<'a>(&'a mut self) -> Events<'a, B> {
        Events { reader: self, finished: false }
    }
}

/// XML events iterator, created by `events()` method on `Parser`.
pub struct Events<'a, B> {
    priv reader: &'a mut EventReader<B>,
    priv finished: bool
}

impl<'a, B: Buffer> Iterator<XmlEvent> for Events<'a, B> {
    #[inline]
    fn next(&mut self) -> Option<XmlEvent> {
        if self.finished { None } 
        else {
            let ev = self.reader.next();
            match ev {
                events::EndDocument | events::Error(_) => self.finished = true,
                _ => {}
            }
            Some(ev)
        }
    }
}

impl EventReader<MemReader> {
    /// Convenience method to create a parser from an owned string.
    #[inline]
    pub fn new_from_str(source: ~str) -> EventReader<MemReader> {
        EventReader::new_from_bytes(source.into_bytes())
    }

    /// Convenience method to create a parser from an owned vector of bytes.
    #[inline]
    pub fn new_from_bytes(source: ~[u8]) -> EventReader<MemReader> {
        EventReader::new(MemReader::new(source))
    }

}

impl<'r> EventReader<BufReader<'r>> {
    /// Convenience method to create a parser from a string slice.
    #[inline]
    pub fn new_from_str_slice(source: &'r str) -> EventReader<BufReader<'r>> {
        EventReader::new_from_bytes_slice(source.as_bytes())
    }

    /// Convenience method to create a parser from a slice of bytes.
    #[inline]
    pub fn new_from_bytes_slice(source: &'r [u8]) -> EventReader<BufReader<'r>> {
        EventReader::new(BufReader::new(source))
    }
}

#[cfg(test)]
mod tests {
    use std::io::File;
    use std::io::BufferedReader;

    use super::{EventReader, ParserConfig};

    fn test_sample(path: &str) {
        let file = File::open(&Path::new(path));
        let reader = BufferedReader::new(file);

        let mut eventreader = EventReader::new_with_config(
            reader, 
            ParserConfig::new()
                .ignore_comments(true)
                .whitespace_to_characters(true)
                .cdata_to_characters(true)
                .trim_whitespace(true)
                .coalesce_characters(true)
        );

        for e in eventreader.events() {
            println!("{}", e);
        }
    }

    #[test]
    fn sample_1_test() {
        test_sample("data/sample_1.xml");
    }

    #[test]
    fn sample_2_test() {
        test_sample("data/sample_2.xml");
    }

    #[test]
    fn sample_3_test() {
        test_sample("data/sample_3.xml");
    }

    #[test]
    fn sample_4_test() {
        test_sample("data/sample_4.xml");
    }
}