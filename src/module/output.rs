use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::sync::Mutex;

use super::inputs::*;

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
}

struct XmlBuilder {
    pub writer: Mutex<Writer<Vec<u8>>>,
}

impl XmlBuilder {
    fn new() -> Self {
        let buffer = Vec::new();
        Self {
            writer: Mutex::new(Writer::new_with_indent(buffer, b'\t', 1)),
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

pub struct FGUModule {
    pub module: ModuleDefinition,
    pub spells: Vec<SpellDefinition>,
    pub tables: Vec<TableDefinition>,
}

impl FGUModule {
    pub fn process(&self, destination: &str) -> Result<(), anyhow::Error> {
        use std::path::Path;
        use std::fs::File;
        use zip::ZipWriter;
        use std::io::prelude::*;

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
                builder.write_string(
                    "name",
                    vec![XmlAttribute::string()],
                    &self.module.name,
                )?;
                builder.write_string(
                    "category",
                    vec![XmlAttribute::string()],
                    "Source Book",
                )?;
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
                                        "table",
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
                                                "table",
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
                    xml_builder.child("table", vec![], |_| {
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

impl TableDefinition {
    fn process(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
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
                        vec![XmlAttribute::number()],
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

impl SpellDefinition {
    fn process(&self, module: &ModuleDefinition, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let id = format!("id-{:05}", self.id);

        w.child(&id, vec![], |builder| {
            self.casting_time(&builder)?;
            self.description(&builder)?;
            self.duration(&builder)?;
            self.spell_level(&builder)?;
            builder.write_string("locked", vec![XmlAttribute::number()], "1")?;
            builder.write_string("name", vec![XmlAttribute::string()], &self.name)?;
            builder.write_string(
                "ritual",
                vec![XmlAttribute::number()],
                if self.is_ritual { 1 } else { 0 },
            )?;
            builder.write_string("range", vec![XmlAttribute::string()], &self.range.to_xml())?;
            builder.write_string("school", vec![XmlAttribute::string()], &self.school)?;
            builder.write_string(
                "prepared",
                vec![XmlAttribute::number()],
                if self.needs_preperation { 1 } else { 0 },
            )?;
            if let Some(short_description) = &self.short_description {
                builder.write_string(
                    "shortdescription",
                    vec![XmlAttribute::string()],
                    &short_description,
                )?;
            }
            builder.write_string("group", vec![XmlAttribute::string()], &self.group.clone())?;
            builder.write_string("source", vec![XmlAttribute::string()], &module.name.clone())?;
            self.actions(&builder)?;

            Ok(())
        })?;
        Ok(())
    }

    fn spell_level(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let spell_level = match self.spell_level {
            SpellLevel::Cantrip => 0,
            SpellLevel::Level { number } => number,
        };

        w.write_string("level", vec![XmlAttribute::number()], spell_level)
    }

    fn duration(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        if let Some(duration) = &self.duration {
            w.write_string("duration", vec![XmlAttribute::string()], &duration)?;
        }

        Ok(())
    }

    fn casting_time(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let casting_time = match self.casting_time {
            SpellCastDuration::Reaction => "1 reaction".to_owned(),
            SpellCastDuration::BonusAction { count } => format!("{} bonus action", count),
            SpellCastDuration::Action { count } => format!("{} action", count),
            SpellCastDuration::Instant => "instant".to_owned(),
        };

        w.write_string("castingtime", vec![XmlAttribute::string()], casting_time)
    }

    fn description(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let description = markdown::to_html(&self.description);
        w.write_raw(
            "description",
            vec![XmlAttribute::custom("type", "formattedtext")],
            description,
        )
    }

    fn actions(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        w.child("actions", vec![], |builder| {
            let mut count = 1;
            for action in &self.actions {
                self.process_action(count, &action, builder)?;
                count += 1;
            }
            Ok(())
        })?;

        Ok(())
    }

    fn process_action(
        &self,
        index: u8,
        action: &SpellAction,
        w: &XmlBuilder,
    ) -> Result<(), anyhow::Error> {
        let id = format!("id-{:05}", index);
        w.child(&id, vec![], |builder| {
            builder.write_string("order", vec![XmlAttribute::number()], index)?;
            match action {
                SpellAction::Cast { cast } => process_cast(cast, w)?,
                SpellAction::Damage { damage } => process_damage(damage, w)?,
                SpellAction::Effect(effect) => process_effect(effect, w)?,
            }
            Ok(())
        })?;

        Ok(())
    }
}

fn process_cast(cast: &SpellRange, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    w.write_string("atktype", vec![XmlAttribute::string()], cast.to_xml())?;
    w.write_string("type", vec![XmlAttribute::string()], "cast")?;
    w.write_string(
        "tknbutton",
        vec![XmlAttribute::custom("type", "tknbutton")],
        "",
    )?;
    w.write_string(
        "tknimg",
        vec![XmlAttribute::custom("type", "tknbutton")],
        "",
    )?;

    Ok(())
}

fn process_effect(effect: &SpellEffect, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    if effect.targets_self {
        w.write_string("targeting", vec![XmlAttribute::string()], "self")?;
    }
    w.write_string("type", vec![XmlAttribute::string()], "effect")?;
    w.write_string("label", vec![XmlAttribute::string()], &effect.effect)?;

    let duration_unit = match effect.duration.unit {
        TimeUnit::Minute => "minute".to_owned(),
        TimeUnit::Hour => "hour".to_owned(),
        TimeUnit::Round => "round".to_owned(),
    };

    w.write_string("durunit", vec![XmlAttribute::string()], duration_unit)?;
    w.write_string(
        "durmod",
        vec![XmlAttribute::number()],
        effect.duration.count,
    )?;
    Ok(())
}

fn process_damage(damage: &Vec<SpellDamage>, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    w.write_string("type", vec![XmlAttribute::string()], "damage")?;
    w.child("damagelist", vec![], |builder| {
        let mut dmg_count = 0;
        for dmg_element in damage.iter() {
            dmg_count += 1;

            let dmg_id = format!("id-{:05}", dmg_count);
            builder.child(&dmg_id, vec![], |builder| {
                let mut dice_list = Vec::new();
                for dice in &dmg_element.dice {
                    dice_list.push(format!("{}{}", dice.count, dice.dice_type));
                }
                let dice_list = dice_list.join(",");

                builder.write_string(
                    "type",
                    vec![XmlAttribute::string()],
                    dmg_element.damage_type.to_owned(),
                )?;
                builder.write_string(
                    "dice",
                    vec![XmlAttribute::custom("type", "dice")],
                    dice_list,
                )?;

                let spell_stat = match dmg_element.stat {
                    SpellStat::Strength => Some("strength"),
                    SpellStat::Dextarity => Some("dextarity"),
                    SpellStat::Constitution => Some("constitution"),
                    SpellStat::Intelligence => Some("intelligence"),
                    SpellStat::Wisdom => Some("wisdom"),
                    SpellStat::Charisma => Some("charisma"),
                    SpellStat::None => None,
                };
                if let Some(stat) = spell_stat {
                    builder.write_string("stat", vec![XmlAttribute::string()], stat)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    })?;

    Ok(())
}

impl SpellRange {
    fn to_xml(&self) -> String {
        match self {
            SpellRange::Touch => "touch".to_owned(),
            SpellRange::IsSelf => "[SELF]".to_owned(),
            SpellRange::Melee => "melee".to_owned(),
            SpellRange::Ranged => "ranged".to_owned(),
        }
    }
}
