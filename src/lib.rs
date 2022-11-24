pub mod error;
use error::XMLResult;

/// An object which can be read from and written as xml.
pub trait XMLObject: Sized {
    /// Returns an instance of this object from xml.
    fn from_xml(xml: &[u8]) -> XMLResult<Self>;
    /// Returns xml describing this object.
    fn to_xml(&self) -> String;
}
