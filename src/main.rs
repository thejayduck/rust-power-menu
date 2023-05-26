use anyhow::{anyhow, Ok};
use args::Arguments;
use clap::Parser;
use std::process::Command;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use system_shutdown::{hibernate, logout, reboot, shutdown, sleep};
mod args;

fn main() {
    let args = Arguments::parse();

    let entries: Vec<_> = Action::iter()
        .map(|action| {
            format!(
                "{} {}",
                if !args.no_icon { action.icon() } else { "" },
                action
            )
        })
        .collect();

    match rofi::Rofi::new(&entries)
        .prompt("Power Menu")
        .pango()
        .run_index()
    {
        std::result::Result::Ok(choice) => {
            get_action(Action::iter().nth(choice).unwrap(), args.dry_run).unwrap();
        }
        Err(rofi::Error::Interrupted) => println!("Interrupted"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString)]
enum Action {
    #[strum(serialize = "Shut Down")]
    ShutDown,
    Reboot,
    Suspend,
    Hibernate,
    #[strum(serialize = "Log Out")]
    Logout,
    #[strum(serialize = "Lock Screen")]
    LockScreen,
}

impl Action {
    fn icon(self) -> &'static str {
        match self {
            Action::ShutDown => "",
            Action::Reboot => "",
            Action::Suspend => "",
            Action::Hibernate => "󰤄",
            Action::Logout => "󰍃",
            Action::LockScreen => "",
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Action::ShutDown => "Shut Down",
            Action::Reboot => "Reboot",
            Action::Suspend => "Suspend",
            Action::Hibernate => "Hibernate",
            Action::Logout => "Log Out",
            Action::LockScreen => "Lock Screen",
        })
    }
}

fn get_action(choice: Action, dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        println!("Called '{}' using dry-run", choice);
        return Ok(());
    }

    match choice {
        Action::ShutDown => shutdown()?,
        Action::Reboot => reboot()?,
        Action::Suspend => sleep()?,
        Action::Hibernate => hibernate()?,
        Action::Logout => logout()?,
        Action::LockScreen => {
            let command = Command::new("loginctl")
                .args(["lock-session", env!("XDG_SESSION_ID")])
                .output()?;

            if !command.status.success() {
                Err(anyhow!(String::from_utf8(command.stderr).unwrap()))?;
            }
        }
    };

    Ok(())
}
