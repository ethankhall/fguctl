use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpellCastDuration {
    Instant,
    Reaction,
    BonusAction { count: u8 },
    Action { count: u8 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellEffectDuration {
    pub count: u8,
    pub unit: TimeUnit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SpellLevel {
    Cantrip,
    Level { number: u8 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpellRange {
    Melee,
    Ranged,
    Touch,
    #[serde(rename = "self")]
    IsSelf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "stat")]
pub enum SpellStat {
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "dex")]
    Dextarity,
    #[serde(rename = "con")]
    Constitution,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "wis")]
    Wisdom,
    #[serde(rename = "cha")]
    Charisma,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dice {
    pub dice_type: String,
    pub count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellDamage {
    #[serde(flatten)]
    pub stat: SpellStat,
    pub damage_type: String,
    pub dice: Vec<Dice>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellEffect {
    pub effect: String,
    pub duration: SpellEffectDuration,
    pub targets_self: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SpellAction {
    Cast { cast: SpellRange },
    Damage { damage: Vec<SpellDamage> },
    Effect(SpellEffect),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellDefinition {
    #[serde(skip, default = "create_spell_id")]
    pub id: u8,
    pub name: String,
    pub short_description: Option<String>,
    pub description: String,
    pub duration: Option<String>,
    pub casting_time: SpellCastDuration,
    pub school: String,
    pub spell_level: SpellLevel,
    pub needs_preperation: bool,
    pub range: SpellRange,
    pub is_ritual: bool,
    pub group: String,
    pub actions: Vec<SpellAction>,
}

fn create_spell_id() -> u8 {
    super::SPELL_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleCategory {
    #[serde(rename = "Source Book")]
    SourceBook,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TableRange {
    pub from: u8,
    pub until: u8,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TableDefinition {
    #[serde(skip, default = "create_table_id")]
    pub id: u8,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_text: Option<String>,
    pub ranges: Vec<TableRange>,
}

fn create_table_id() -> u8 {
    super::TABLE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuleSet {
    #[serde(alias = "5e")]
    FifthEdition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModuleDefinition {
    pub name: String,
    pub spell_files: Vec<String>,
    pub table_files: Vec<String>,
    pub source: String,
    pub category: ModuleCategory,
    pub author: String,
    pub ruleset: RuleSet,
}
