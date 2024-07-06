use fastlib::{MessageFactory, Value};

pub struct NullMessageFactory {
}

impl NullMessageFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl MessageFactory for NullMessageFactory {
    fn start_template(&mut self, _id: u32, _name: &str) {}
    fn stop_template(&mut self) {}
    fn set_value(&mut self, _id: u32, _name: &str, _value: Option<Value>) {}
    fn start_sequence(&mut self, _id: u32, _name: &str, _length: u32) {}
    fn start_sequence_item(&mut self, _index: u32) {}
    fn stop_sequence_item(&mut self) {}
    fn stop_sequence(&mut self) {}
    fn start_group(&mut self, _name: &str) {}
    fn stop_group(&mut self) {}
    fn start_template_ref(&mut self, _name: &str, _dynamic: bool) {}
    fn stop_template_ref(&mut self) {}
}
