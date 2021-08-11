mod module;

use async_trait::async_trait;
use clap::{AppSettings, Clap};
use tracing::{error, Level};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

#[async_trait]
trait CommandExec {
    async fn exec(&self) -> Result<(), anyhow::Error>;
}

#[derive(Clap, Debug)]
#[clap(author, about, version)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_command: RootSubCommand,
}

#[derive(Clap, Debug)]
pub enum RootSubCommand {
    /// Actions about FGU Module's
    Module(module::ModuleSubCommand),
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opts::parse();

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let results = match opt.sub_command {
        RootSubCommand::Module(args) => args.exec().await,
    };

    if let Err(e) = &results {
        error!("Error: {:?}", e);
        std::process::exit(1);
    }

    Ok(())
}
