mod xml_vec;

#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::fmt::Debug;
use std::io::Read;

use yaserde::__xml::reader::XmlEvent;
use yaserde::de::{Deserializer, from_str};
use yaserde::YaDeserialize;
use crate::xml_vec::XmlVec;

#[derive(Debug, Default, YaDeserialize)]
struct W {
  #[yaserde(child)]
  c: VecC,
}

#[derive(Debug, Default)]
pub struct VecC(Vec<C>);

impl XmlVec<C> for VecC {
  fn push(&mut self, element: C) {
    self.0.push(element);
  }
}

impl YaDeserialize for VecC {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    VecC::read_inner_value(reader)
  }
}

#[derive(Debug, YaDeserialize)]
enum C {
  C1,
  C2(String),
  C3 {
    #[yaserde(attribute)]
    t: i32,
    #[yaserde(text)]
    b: String
  }
}

impl Default for C {
  fn default() -> Self {
    Self::C1
  }
}

fn main() {
  // let string = "<w><c><C2>Hallo</C2><C1>Hey</C1><C1>Fish</C1><C1>and Fun</C1></c></w>";
  let string = "<C2>Hallo</C2>";
  let w: C = from_str(string).expect("Wrong Xml Format");
  println!("{:?}", w);
}