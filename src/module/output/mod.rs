use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::sync::Mutex;

mod spell;
mod tables;
mod fgu_module;

pub use fgu_module::FGUModule;

struct XmlAttribute {
    prop_name: String,
    prop_value: String,
}

impl XmlAttribute {
    fn custom<S>(name: S, value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            prop_name: name.into(),
            prop_value: value.into(),
        }
    }

    fn string() -> Self {
        Self {
            prop_name: "type".into(),
            prop_value: "string".into(),
        }
    }

    fn number() -> Self {
        Self {
            prop_name: "type".into(),
            prop_value: "number".into(),
        }
    }

    fn r#type<S>(value: S) -> Self where S: ToString {
        Self {
            prop_name: "type".into(),
            prop_value: value.to_string()
        }
    }
}

pub struct XmlBuilder {
    pub writer: Mutex<Writer<Vec<u8>>>,
}

impl XmlBuilder {
    fn new() -> Self {
        let buffer = Vec::new();
        Self {
            writer: Mutex::new(Writer::new_with_indent(buffer, b'\t', 1)),
        }
    }

    fn to_number(input: bool) -> u8 {
        if input {
            1
        } else {
            0
        }
    }

    fn child<F>(
        &self,
        name: &str,
        attributes: Vec<XmlAttribute>,
        fun: F,
    ) -> Result<(), anyhow::Error>
    where
        F: Fn(&XmlBuilder) -> Result<(), anyhow::Error>,
    {
        let mut elem = BytesStart::owned_name(name);

        for attribute in attributes {
            let key: &'_ str = &attribute.prop_name;
            let value: &'_ str = &attribute.prop_value;
            elem.push_attribute((key, value));
        }

        {
            let mut real_writer = self.writer.lock().unwrap();
            real_writer.write_event(Event::Start(elem))?;
        }

        fun(&self)?;

        {
            let mut real_writer = self.writer.lock().unwrap();
            real_writer.write_event(Event::End(BytesEnd::borrowed(name.as_ref())))?;
        }

        Ok(())
    }

    fn empty_node(
        &self,
        element_name: &str,
        attributes: Vec<XmlAttribute>,
    ) -> Result<(), anyhow::Error> {
        let mut elem = BytesStart::owned_name(element_name);
        for attribute in attributes {
            let key: &'_ str = &attribute.prop_name;
            let value: &'_ str = &attribute.prop_value;
            elem.push_attribute((key, value));
        }
        let mut real_writer = self.writer.lock().unwrap();
        real_writer.write_event(Event::Empty(elem))?;
        Ok(())
    }

    fn write_string<S>(
        &self,
        element_name: &str,
        attributes: Vec<XmlAttribute>,
        content: S,
    ) -> Result<(), anyhow::Error>
    where
        S: ToString,
    {
        let mut elem = BytesStart::owned_name(element_name);
        for attribute in attributes {
            let key: &'_ str = &attribute.prop_name;
            let value: &'_ str = &attribute.prop_value;
            elem.push_attribute((key, value));
        }
        let mut real_writer = self.writer.lock().unwrap();
        real_writer.write_event(Event::Start(elem))?;

        real_writer.write_event(Event::Text(BytesText::from_plain(
            content.to_string().as_ref(),
        )))?;

        real_writer.write_event(Event::End(BytesEnd::borrowed(element_name.as_ref())))?;
        Ok(())
    }

    fn write_raw<S>(
        &self,
        element_name: &str,
        attributes: Vec<XmlAttribute>,
        content: S,
    ) -> Result<(), anyhow::Error>
    where
        S: ToString,
    {
        self.child(element_name, attributes, |builder| {
            let mut real_writer = builder.writer.lock().unwrap();
            real_writer.write(b"\n")?;
            real_writer.write(content.to_string().as_bytes())?;
            real_writer.write(b"\n")?;

            Ok(())
        })
    }

    fn to_string(self) -> Result<String, anyhow::Error> {
        let real_writer = self.writer.into_inner().unwrap();
        Ok(String::from_utf8(real_writer.into_inner())?)
    }
}