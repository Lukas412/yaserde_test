use std::fmt::Debug;
use std::io::Read;
use yaserde::__xml::reader::XmlEvent;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

pub fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}

pub trait XmlVec<E: YaDeserialize + Default + Debug>: Sized + Default {
  fn push(&mut self, element: E);

  fn push_deserialized_element<R: Read>(&mut self, reader: &mut Deserializer<R>) -> Result<(), String> {
    let element: E = from_deserializer(reader)?;
    self.push(element);
    Ok(())
  }

  fn read_inner_value<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    if let XmlEvent::StartElement { name: owned_name, .. } = reader.next_event()? {
      let mut result = Self::default();
      loop {
        println!("event: {:?}", reader.peek());
        match reader.peek()? {
          XmlEvent::StartElement { .. } => {
            result.push_deserialized_element(reader)?;
          }
          XmlEvent::EndElement { name } => {
            if &owned_name == name {
              return Ok(result);
            }
          }
          _ => {}
        };
        println!("done: {:?}", reader.peek());
        reader.next_event()?;
      }
    } else {
      Err("Expected start event".to_string())
    }
  }
}