use super::Command;
use crate::error::CliError;
use crate::git;
use crate::input;
use log::info;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct TagCommand {
    #[structopt(short, long, help = "Provide a tag name")]
    name: Option<String>,

    #[structopt(short = "y", long, help = "Want to create a new version (y/N)")]
    validate: bool,

    #[structopt(flatten)]
    tag_options: git::TagGeneratorOptions,
}

impl Command for TagCommand {
    fn execute(&self) -> Result<(), CliError> {
        if git::has_staged_changes()? {
            return Err(CliError::StagedChanges);
        }

        if let Some(name) = &self.name {
            info!("Tag {} created successfully!", name);
        } else {
            let validate = if !self.validate {
                input::ask_want_create_new_tag()?
            } else {
                true
            };
            if !validate {
                info!("Abort");
                return Ok(());
            }
            let version_manager = git::TagGenerator::new(self.tag_options.clone());
            version_manager.run()?;
        }

        Ok(())
    }
}
