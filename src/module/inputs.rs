use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

#[derive(Debug, Serialize, Deserialize)]

pub enum AbilityScore {
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "dex")]
    Dexterity,
    #[serde(rename = "con")]
    Constitution,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "wis")]
    Wisdom,
    #[serde(rename = "cha")]
    Charisma,
}

impl AbilityScore {
    pub fn to_long_name(&self) -> String {
        match self {
            AbilityScore::Strength => "strength",
            AbilityScore::Dexterity => "dexterity",
            AbilityScore::Constitution => "constitution",
            AbilityScore::Intelligence => "intelligence",
            AbilityScore::Wisdom => "wisdom",
            AbilityScore::Charisma => "charisma",
        }
        .to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SpellCastDuration {
    Instant,
    Reaction,
    BonusAction { count: u8 },
    Action { count: u8 },
    Forever
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellEffectDurationFinite {
    pub count: u8,
    pub unit: TimeUnit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "time-scale", rename_all = "lowercase")]
pub enum SpellEffectDuration {
    Finite(SpellEffectDurationFinite),
    Indefinite
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SpellLevel {
    Cantrip,
    Level { number: u8 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttackRange {
    Melee,
    Ranged,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellRange {
    pub range: AttackRange,
    pub save: ToSave
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellSave {
    pub is_magic: bool,
    pub stat: SpellStat,
    pub save: ToSave,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SpellStat {
    #[serde(rename = "ability-score")]
    AbilityScore { ability: AbilityScore },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CustomSpellSave {
    pub stat: SpellStat,
    pub is_proficient: bool,
    #[serde(skip, default = "Default::default")]
    pub bonus: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToSave {
    Fixed { value: u8 },
    DC,
    Ability(CustomSpellSave),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "damage-mod")]
pub enum DamageModifier {
    #[serde(rename = "ability-score")]
    AbilityScore { ability: AbilityScore },
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
    pub modifier: DamageModifier,
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
pub struct ActionDamage {
    pub damage: Vec<SpellDamage>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellActions {
    #[serde(default = "Default::default")]
    pub attacks: Vec<SpellRange>,
    #[serde(default = "Default::default")]
    pub saves: Vec<SpellSave>,
    #[serde(default = "Default::default")]
    pub damages: Vec<ActionDamage>,
    #[serde(default = "Default::default")]
    pub effects: Vec<SpellEffect>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpellDefinition {
    #[serde(skip, default = "Default::default")]
    pub id: SpellId,
    pub name: String,
    pub short_description: Option<String>,
    pub description: String,
    pub duration: Option<String>,
    pub casting_time: SpellCastDuration,
    pub school: String,
    pub spell_level: SpellLevel,
    pub needs_preperation: bool,
    pub is_ritual: bool,
    pub group: String,
    pub actions: SpellActions
}

#[derive(Debug)]
pub struct SpellId {
    value: u8
}

impl SpellId {
    pub fn get_id(&self) -> u8 {
        self.value
    }
}

impl Default for SpellId {
    fn default() -> Self {
        Self {
            value: super::SPELL_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
        }
    }
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
