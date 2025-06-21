use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Actions {
    #[clap(alias = "s")]
    Setup,

    #[clap(alias = "c")]
    Create,

    #[clap(alias = "r")]
    Rename,
}

impl From<&str> for Actions {
    fn from(action: &str) -> Self {
        match action {
            "setup" => Actions::Setup,
            "create" => Actions::Create,
            "rename" => Actions::Rename,
            _ => unreachable!(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(propagate_version = true)]
#[command(version, about, long_about = None)]
#[command(about = "create python virtualenv with optional name prompt!")]
#[command(after_help = r#"Examples:

$ envpy setup             # creates "cd" function in profile file (activates venv as you cd to it)

$ envpy create            # create .venv in curr dir
$ envpy create -d xyz     # create virtual env in "xyz" dir (automatic venv activation won't work!)
$ envpy create -n envpy   # create .venv but 'envpy' will be shown on prompt
$ envpy create -d x -n y  # create "x" venv and once loaded "y" will be shown on prompt.

$ envpy rename -n y       # rename existing prompt from ".venv"  to "y"
$ envpy renmae -d x -n y  # rename existing prompt from "x" to "y"
"#)]
pub struct Cli {
    #[arg(value_enum, help = "create/setup/rename virtual env")]
    pub actions: Actions,

    #[arg(short, long, default_value = ".venv", help = "venv dir name")]
    pub dir_name: String,

    #[arg(
        short,
        long,
        default_value = "",
        help = "optional name for prompt instead of venv dir"
    )]
    pub name: String,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "backup existing loader file (e.g. activate)"
    )]
    pub backup: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Tests that actions are parsed correctly whether given the full name or
    /// abbreviation.
    #[test]
    fn test_parse_action() {
        let actions: [[&str; 2]; 3] = [["setup", "s"], ["create", "c"], ["rename", "r"]];

        for [full, abbr] in actions.iter() {
            let expected: Actions = (*full).into();

            let cli_full = Cli::parse_from(["envpy", full]);

            assert_eq!(cli_full.actions, expected);

            let cli_abbr = Cli::parse_from(["envpy", abbr]);
            assert_eq!(cli_abbr.actions, expected);
        }
    }

    #[test]
    fn test_create_with_no_args() {
        let cli = Cli::parse_from(["envpy", "create"]);
        assert_eq!(cli.actions, Actions::Create);
        assert_eq!(cli.dir_name, ".venv");
        assert_eq!(cli.name, "");
        assert!(!cli.backup);
    }

    #[test]
    fn test_create_with_dir_name() {
        let cli = Cli::parse_from(["envpy", "create", "-d", "x"]);
        assert_eq!(cli.actions, Actions::Create);
        assert_eq!(cli.dir_name, "x");
        assert_eq!(cli.name, "");
        assert!(!cli.backup);
    }

    #[test]
    fn test_create_with_name() {
        let cli = Cli::parse_from(["envpy", "create", "-n", "x"]);
        assert_eq!(cli.actions, Actions::Create);
        assert_eq!(cli.dir_name, ".venv");
        assert_eq!(cli.name, "x");
        assert!(!cli.backup);
    }

    #[test]
    fn test_create_with_dir_name_and_name() {
        let cli = Cli::parse_from(["envpy", "create", "-d", "x", "-n", "y"]);
        assert_eq!(cli.actions, Actions::Create);
        assert_eq!(cli.dir_name, "x");
        assert_eq!(cli.name, "y");
        assert!(!cli.backup);
    }

    #[test]
    fn test_rename_with_no_args() {
        let cli = Cli::parse_from(["envpy", "rename"]);
        assert_eq!(cli.actions, Actions::Rename);
        assert_eq!(cli.dir_name, ".venv");
        assert_eq!(cli.name, "");
        assert!(!cli.backup);
    }

    #[test]
    fn test_rename_with_args() {
        let cli = Cli::parse_from(["envpy", "rename", "-d", "x", "-n", "y"]);
        assert_eq!(cli.actions, Actions::Rename);
        assert_eq!(cli.dir_name, "x");
        assert_eq!(cli.name, "y");
        assert!(!cli.backup);
    }

    #[test]
    fn test_rename_with_backup() {
        let cli = Cli::parse_from(["envpy", "rename", "-n", "y", "--backup"]);
        assert_eq!(cli.actions, Actions::Rename);
        assert_eq!(cli.dir_name, ".venv");
        assert_eq!(cli.name, "y");
        assert!(cli.backup);
    }
}
