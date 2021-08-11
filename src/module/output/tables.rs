use crate::module::inputs::*;
use super::*;

impl TableDefinition {
    pub fn process(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let note_text = self
            .formatted_text
            .as_ref()
            .map(|x| x.clone())
            .unwrap_or_default();
        let note_text = markdown::to_html(&note_text);
        let id = format!("id-{:05}", self.id);

        w.child(&id, vec![], |builder| {
            builder.write_string(
                "description",
                vec![XmlAttribute::string()],
                &self.description,
            )?;
            builder.write_string("dice", vec![XmlAttribute::custom("type", "dice")], "")?;
            builder.write_string("enabled", vec![XmlAttribute::number()], 0)?;
            builder.write_string("hiddenenabled", vec![XmlAttribute::string()], "Disabled")?;
            builder.write_string("hiderollresults", vec![XmlAttribute::number()], 0)?;
            builder.write_string("labelcol1", vec![XmlAttribute::string()], "Effect")?;
            builder.write_string("locked", vec![XmlAttribute::number()], 1)?;
            builder.write_string("mode", vec![XmlAttribute::number()], 0)?;
            builder.write_string("name", vec![XmlAttribute::string()], &self.name)?;
            w.write_raw(
                "notes",
                vec![XmlAttribute::custom("type", "formattedtext")],
                &note_text,
            )?;
            builder.write_string("resultscols", vec![XmlAttribute::number()], 1)?;
            builder.write_string("table_positionoffset", vec![XmlAttribute::number()], 0)?;
            builder.child("tablerows", vec![], |builder| {
                let mut counter = 0;
                for range in &self.ranges {
                    counter += 1;
                    TableDefinition::process_table_row(range, counter, builder)?;
                }

                Ok(())
            })?;
            Ok(())
        })?;
        Ok(())
    }

    fn process_table_row(
        range: &TableRange,
        index: u8,
        w: &XmlBuilder,
    ) -> Result<(), anyhow::Error> {
        let id = format!("id-{:05}", index);
        w.child(&id, vec![], |builder| {
            builder.write_string("fromrange", vec![XmlAttribute::number()], range.from)?;
            builder.write_string("torange", vec![XmlAttribute::number()], range.until)?;
            builder.child("results", vec![], |builder| {
                builder.child("id-00001", vec![], |builder| {
                    //yolo, idk when you would need more than 1
                    builder.write_string(
                        "result",
                        vec![XmlAttribute::string()],
                        &range.description,
                    )?;
                    builder.child(
                        "resultlink",
                        vec![XmlAttribute::custom("type", "windowreference")],
                        |builder| {
                            builder.empty_node("class", vec![])?;
                            builder.empty_node("recordname", vec![])
                        },
                    )
                })
            })
        })
    }
}
