use crate::module::inputs::*;
use super::*;
use tracing::trace;

impl SpellDefinition {
    pub fn process(&self, module: &ModuleDefinition, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let id = format!("id-{:05}", self.id.get_id());

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
                XmlBuilder::to_number(self.is_ritual),
            )?;
            // builder.write_string("range", vec![XmlAttribute::string()], &self.range.to_xml())?;
            builder.write_string("school", vec![XmlAttribute::string()], &self.school)?;
            builder.write_string(
                "prepared",
                vec![XmlAttribute::number()],
                XmlBuilder::to_number(self.needs_preperation),
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
            SpellCastDuration::Forever => "forever".to_owned(),
        };

        w.write_string("castingtime", vec![XmlAttribute::string()], casting_time)
    }

    fn description(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        let description = markdown::to_html(&self.description);
        w.write_raw(
            "description",
            vec![XmlAttribute::r#type("formattedtext")],
            description,
        )
    }

    fn actions(&self, w: &XmlBuilder) -> Result<(), anyhow::Error> {
        trace!("Actions: {:?}", self.actions);
        w.child("actions", vec![], |builder| {
            let mut count = 1;
            for action in &self.actions.attacks {
                count += 1;
                SpellDefinition::process_action(count, || process_attack(action, w), builder)?;
            }
            for action in &self.actions.saves {
                count += 1;
                SpellDefinition::process_action(count, || process_save(action, w), builder)?;
            }
            for action in &self.actions.damages {
                count += 1;
                SpellDefinition::process_action(count, || process_damage(action, w), builder)?;
            }
            for action in &self.actions.effects {
                count += 1;
                SpellDefinition::process_action(count, || process_effect(action, w), builder)?;
            }
            Ok(())
        })?;

        Ok(())
    }

    fn process_action<F>(
        index: u8,
        action: F,
        w: &XmlBuilder,
    ) -> Result<(), anyhow::Error> where F: Fn() -> Result<(), anyhow::Error> {
        let id = format!("id-{:05}", index);
        w.child(&id, vec![], |builder| {
            builder.write_string("order", vec![XmlAttribute::number()], index)?;
            action()
        })?;

        Ok(())
    }
}

fn process_save(cast: &SpellSave, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    match &cast.save {
        ToSave::DC => {
            w.write_string("savedcmod", vec![XmlAttribute::number()], 0)?;
            w.write_string("savedcprof", vec![XmlAttribute::number()], 1)?;
        }
        ToSave::Fixed{ value: _ } => {}
        ToSave::Ability(custom) => {
            w.write_string("savedcbase", vec![XmlAttribute::string()], "ability")?;
            w.write_string("savedcmod", vec![XmlAttribute::number()], custom.bonus)?;
            w.write_string(
                "savedcprof",
                vec![XmlAttribute::number()],
                XmlBuilder::to_number(custom.is_proficient),
            )?;
            w.write_string(
                "savedcstat",
                vec![XmlAttribute::string()],
                match &custom.stat {
                    SpellStat::AbilityScore { ability } => ability.to_long_name(),
                },
            )?;
        }
    }
    w.write_string(
        "savemagic",
        vec![XmlAttribute::number()],
        XmlBuilder::to_number(cast.is_magic),
    )?;
    w.write_string(
        "savetype",
        vec![XmlAttribute::string()],
        match &cast.stat {
            SpellStat::AbilityScore { ability } => ability.to_long_name(),
        },
    )?;
    w.write_string(
        "tknbutton",
        vec![XmlAttribute::r#type("tknbutton")],
        "",
    )?;
    w.write_string(
        "tknimg",
        vec![XmlAttribute::r#type("tknbutton")],
        "",
    )?;
    w.write_string("type", vec![XmlAttribute::string()], "cast")?;
    Ok(())
}

fn process_attack(cast: &SpellRange, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    w.write_string("atktype", vec![XmlAttribute::string()], cast.range.to_xml())?;
    w.write_string("type", vec![XmlAttribute::string()], "cast")?;
    w.write_string(
        "tknbutton",
        vec![XmlAttribute::r#type("tknbutton")],
        "",
    )?;
    w.write_string(
        "tknimg",
        vec![XmlAttribute::r#type("tknbutton")],
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

    if let SpellEffectDuration::Finite(duration) = &effect.duration {
        let duration_unit = match duration.unit {
            TimeUnit::Minute => "minute".to_owned(),
            TimeUnit::Hour => "hour".to_owned(),
            TimeUnit::Round => "round".to_owned(),
        };

        w.write_string("durunit", vec![XmlAttribute::string()], duration_unit)?;
        w.write_string(
            "durmod",
            vec![XmlAttribute::number()],
            duration.count,
        )?;
    }
    Ok(())
}

fn process_damage(action_damage: &ActionDamage, w: &XmlBuilder) -> Result<(), anyhow::Error> {
    w.write_string("type", vec![XmlAttribute::string()], "damage")?;
    w.child("damagelist", vec![], |builder| {
        let mut dmg_count = 0;
        for dmg_element in action_damage.damage.iter() {
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
                    vec![XmlAttribute::r#type("dice")],
                    dice_list,
                )?;

                let spell_stat = match &dmg_element.modifier {
                    DamageModifier::AbilityScore { ability } => Some(ability.to_long_name()),
                    DamageModifier::None => None,
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

impl AttackRange {
    fn to_xml(&self) -> String {
        match self {
            AttackRange::Melee => "melee".to_owned(),
            AttackRange::Ranged => "ranged".to_owned(),
        }
    }
}
