use crate::module::inputs::*;
use super::*;

pub struct FGUModule {
    pub module: ModuleDefinition,
    pub spells: Vec<SpellDefinition>,
    pub tables: Vec<TableDefinition>,
}

impl FGUModule {
    pub fn process(&self, destination: &str) -> Result<(), anyhow::Error> {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;
        use zip::ZipWriter;

        let client_file = self.client_file()?;
        let definition_file = self.definition_file()?;

        let path = Path::new(&destination);
        let file = File::create(&path)?;

        let mut zip = ZipWriter::new(file);

        zip.start_file("client.xml", Default::default())?;
        zip.write_all(client_file.as_bytes())?;

        zip.start_file("definition.xml", Default::default())?;
        zip.write_all(definition_file.as_bytes())?;

        zip.finish()?;

        Ok(())
    }

    fn definition_file(&self) -> Result<String, anyhow::Error> {
        let xml_builder = XmlBuilder::new();
        xml_builder.child(
            "root",
            vec![
                XmlAttribute::custom("version", "4.1"),
                XmlAttribute::custom("dataversion", "20210302"),
                XmlAttribute::custom("release", "8.1|CoreRPG:4.1"),
            ],
            |builder| {
                builder.write_string("name", vec![XmlAttribute::string()], &self.module.name)?;
                builder.write_string("category", vec![XmlAttribute::string()], "Source Book")?;
                builder.write_string(
                    "author",
                    vec![XmlAttribute::string()],
                    &self.module.author,
                )?;
                builder.write_string("ruleset", vec![XmlAttribute::string()], "5E")?;
                Ok(())
            },
        )?;

        xml_builder.to_string()
    }

    fn client_file(&self) -> Result<String, anyhow::Error> {
        let xml_builder = XmlBuilder::new();
        let lower_module = self.module.name.replace(" ", "").to_lowercase();

        let has_spells = !self.spells.is_empty();
        let has_tables = !self.tables.is_empty();

        xml_builder.child(
            "root",
            vec![
                XmlAttribute::custom("version", "4.1"),
                XmlAttribute::custom("dataversion", "20210302"),
                XmlAttribute::custom("release", "8.1|CoreRPG:4.1"),
            ],
            |_| {
                xml_builder.child("library", vec![], |_| {
                    xml_builder.child(
                        &lower_module,
                        vec![XmlAttribute::custom("static", "true")],
                        |_| {
                            xml_builder.write_string(
                                "categoryname",
                                vec![XmlAttribute::string()],
                                "Source Book",
                            )?;
                            xml_builder.write_string(
                                "name",
                                vec![XmlAttribute::string()],
                                &lower_module,
                            )?;
                            xml_builder.child("entries", vec![], |_| {
                                if has_spells {
                                    xml_builder.child(
                                        "spell",
                                        vec![XmlAttribute::custom("static", "true")],
                                        |_| {
                                            xml_builder.child(
                                                "librarylink",
                                                vec![XmlAttribute::custom(
                                                    "type",
                                                    "windowreference",
                                                )],
                                                |_| {
                                                    xml_builder.write_string(
                                                        "class",
                                                        vec![],
                                                        "reference_list",
                                                    )?;
                                                    xml_builder.write_string(
                                                        "recordname",
                                                        vec![],
                                                        "..",
                                                    )?;
                                                    Ok(())
                                                },
                                            )?;

                                            xml_builder.write_string(
                                                "name",
                                                vec![XmlAttribute::string()],
                                                "Spells",
                                            )?;
                                            xml_builder.write_string(
                                                "recordtype",
                                                vec![XmlAttribute::string()],
                                                "spell",
                                            )?;
                                            Ok(())
                                        },
                                    )?;
                                }
                                if has_tables {
                                    xml_builder.child(
                                        "tables",
                                        vec![XmlAttribute::custom("static", "true")],
                                        |_| {
                                            xml_builder.child(
                                                "librarylink",
                                                vec![XmlAttribute::custom(
                                                    "type",
                                                    "windowreference",
                                                )],
                                                |_| {
                                                    xml_builder.write_string(
                                                        "class",
                                                        vec![],
                                                        "reference_list",
                                                    )?;
                                                    xml_builder.write_string(
                                                        "recordname",
                                                        vec![],
                                                        "..",
                                                    )?;
                                                    Ok(())
                                                },
                                            )?;

                                            xml_builder.write_string(
                                                "name",
                                                vec![XmlAttribute::string()],
                                                "Tables",
                                            )?;
                                            xml_builder.write_string(
                                                "recordtype",
                                                vec![XmlAttribute::string()],
                                                "tables",
                                            )?;
                                            Ok(())
                                        },
                                    )?;
                                }
                                Ok(())
                            })
                        },
                    )
                })?;

                if has_spells {
                    xml_builder.child("spell", vec![], |_| {
                        for spell in &self.spells {
                            spell.process(&self.module, &xml_builder)?;
                        }
                        Ok(())
                    })?;
                }

                if has_tables {
                    xml_builder.child("tables", vec![], |_| {
                        for table in &self.tables {
                            table.process(&xml_builder)?;
                        }
                        Ok(())
                    })?;
                }

                Ok(())
            },
        )?;

        xml_builder.to_string()
    }
}