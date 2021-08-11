use super::inputs::*;
use crate::CommandExec;
use async_trait::async_trait;
use clap::{AppSettings, Clap};
use std::fs::File;
use std::io::Write;
use tracing::info;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub enum ModuleSubCommand {
    /// Take a module definition and generate a module file
    #[clap(name = "build")]
    BuildModule(BuildModuleArgs),

    /// Create a new spell file, fully populated
    #[clap(name = "create-spell")]
    CreateSpell(CreateSpellArgs),

    /// Create a new table file, fully populated
    #[clap(name = "create-table")]
    CreateTable(CreateTableArgs),
}

#[async_trait]
impl CommandExec for ModuleSubCommand {
    async fn exec(&self) -> Result<(), anyhow::Error> {
        match self {
            ModuleSubCommand::BuildModule(args) => args.exec().await,
            ModuleSubCommand::CreateSpell(args) => args.exec().await,
            ModuleSubCommand::CreateTable(args) => args.exec().await,
        }
    }
}

#[derive(Clap, Debug)]
pub struct CreateTableArgs {
    /// Path to the root module deinition
    #[clap(long = "output", short = 'o')]
    pub output_file: String,

    /// Path to the root module deinition
    #[clap(long = "name")]
    pub name: String,
}

#[async_trait]
impl CommandExec for CreateTableArgs {
    async fn exec(&self) -> Result<(), anyhow::Error> {
        let table_def = TableDefinition {
            id: 1,
            name: self.name.clone(),
            ranges: vec![
                TableRange {
                    from: 1,
                    until: 10,
                    description: "low".to_owned(),
                },
                TableRange {
                    from: 11,
                    until: 100,
                    description: "high".to_owned(),
                },
            ],
            description: "A simple table".to_owned(),
            formatted_text: None,
        };

        let text = serde_yaml::to_string(&table_def)?;
        let mut f = File::create(&self.output_file)?;
        f.write_all(text.as_bytes())?;

        info!(
            "Wrote file {}. You will need to add it to your module definition.",
            self.output_file
        );

        Ok(())
    }
}

#[derive(Clap, Debug)]
pub struct CreateSpellArgs {
    /// Path to the root module deinition
    #[clap(long = "output", short = 'o')]
    pub output_file: String,

    /// Path to the root module deinition
    #[clap(long = "name")]
    pub name: String,
}

#[async_trait]
impl CommandExec for CreateSpellArgs {
    async fn exec(&self) -> Result<(), anyhow::Error> {
        let spell_def: SpellDefinition = SpellDefinition {
            id: 1,
            name: self.name.clone(),
            short_description: Option::from("Something simple".to_owned()),
            duration: Option::from("1 minuite".to_owned()),
            description: "
# Simple Description

- List item 1
- List item 2
"
            .to_owned(),
            casting_time: SpellCastDuration::Instant,
            school: "Maaagic!".to_owned(),
            spell_level: SpellLevel::Cantrip,
            needs_preperation: false,
            is_ritual: false,
            group: "A group".to_owned(),
            actions: SpellActions {
                attacks: vec![SpellRange {
                    range: AttackRange::Melee,
                    save: ToSave::DC
                },
                SpellRange {
                    range: AttackRange::Ranged,
                    save: ToSave::DC
                }],
                saves: vec![
                    SpellSave {
                            is_magic: false,
                            stat: SpellStat::AbilityScore {
                                ability: AbilityScore::Charisma,
                            },
                            save: ToSave::DC,
                        },
                    SpellSave {
                            is_magic: false,
                            stat: SpellStat::AbilityScore {
                                ability: AbilityScore::Charisma,
                            },
                            save: ToSave::Ability(CustomSpellSave {
                                bonus: 1,
                                stat: SpellStat::AbilityScore {
                                    ability: AbilityScore::Charisma,
                                },
                                is_proficient: true,
                            }),
                        },
                ],
                damages: vec![
                    ActionDamage {
                        damage: vec![SpellDamage {
                            modifier: DamageModifier::AbilityScore { ability: AbilityScore::Constitution },
                            damage_type: "slashing".to_owned(),
                            dice: vec![Dice {
                                dice_type: "d4".to_owned(),
                                count: 1,
                            }],
                        }],
                    }
                ],
                effects: vec![

                SpellEffect {
                    effect: "DMG: 4d4".to_owned(),
                    duration: SpellEffectDuration {
                        count: 1,
                        unit: TimeUnit::Minute,
                    },
                    targets_self: true,
                },
                SpellEffect {
                    effect: "DMG: 4d4".to_owned(),
                    duration: SpellEffectDuration {
                        count: 1,
                        unit: TimeUnit::Round,
                    },
                    targets_self: false,
                },
                SpellEffect {
                    effect: "DMG: 4d4".to_owned(),
                    duration: SpellEffectDuration {
                        count: 1,
                        unit: TimeUnit::Hour,
                    },
                    targets_self: true,
                },
                SpellEffect {
                    effect: "DMG: 4d4".to_owned(),
                    duration: SpellEffectDuration {
                        count: 1,
                        unit: TimeUnit::Minute,
                    },
                    targets_self: false,
                },
                ]
            }
        };

        let text = serde_yaml::to_string(&spell_def)?;
        let mut f = File::create(&self.output_file)?;
        f.write_all(text.as_bytes())?;

        info!(
            "Wrote file {}. You will need to add it to your module definition.",
            self.output_file
        );
        Ok(())
    }
}

#[derive(Clap, Debug)]
pub struct BuildModuleArgs {
    /// Path to the root module deinition
    #[clap(long = "module-definition", short = 'm')]
    pub module_definition: String,

    /// Where to write the module file to
    #[clap(long = "output", short = 'o')]
    pub output: String,
}

#[async_trait]
impl CommandExec for BuildModuleArgs {
    async fn exec(&self) -> Result<(), anyhow::Error> {
        use std::path::PathBuf;

        let mut spells: Vec<SpellDefinition> = Vec::new();
        let mut tables: Vec<TableDefinition> = Vec::new();

        let mut root_dir = PathBuf::from(&self.module_definition);
        root_dir.pop();

        let module_def: ModuleDefinition =
            serde_yaml::from_str(&std::fs::read_to_string(&self.module_definition)?)?;

        info!("Processing {} spells...", module_def.spell_files.len());
        for spell_file in &module_def.spell_files {
            let spell_file = root_dir.join(spell_file);
            let spell_file = spell_file.to_str().unwrap();
            info!("Processing {}", spell_file);
            let spell: SpellDefinition =
                serde_yaml::from_str(&std::fs::read_to_string(&spell_file)?)?;
            spells.push(spell);
        }

        info!("Processing {} tables...", module_def.table_files.len());
        for table_file in &module_def.table_files {
            let table_file = root_dir.join(table_file);
            let table_file = table_file.to_str().unwrap();
            info!("Processing {}", table_file);
            let table: TableDefinition =
                serde_yaml::from_str(&std::fs::read_to_string(&table_file)?)?;
            tables.push(table);
        }

        let fgu_module = super::output::FGUModule {
            module: module_def,
            spells,
            tables,
        };

        fgu_module.process(&self.output)?;

        Ok(())
    }
}
