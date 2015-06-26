use time;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub static TIME_FORMAT:&'static str = "%F %Y at %T";

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct SerdeTime(pub time::Tm);

impl Serialize for SerdeTime {
  #[inline]
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
      where S: Serializer {
    format!("{}", self.0.strftime(TIME_FORMAT).unwrap()).serialize(serializer)
  }
}

impl Deserialize for SerdeTime {
  #[inline]
  fn deserialize<D>(deserializer: &mut D) -> Result<SerdeTime, D::Error>
      where D: Deserializer {
    let time_str = try!(String::deserialize(deserializer));
    Ok(SerdeTime(time::strptime(&time_str, TIME_FORMAT).unwrap()))
  }
}
