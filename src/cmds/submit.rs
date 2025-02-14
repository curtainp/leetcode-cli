//! Submit command
use super::Command;
use crate::Error;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

/// Abstract Submit Command
///
/// ```sh
/// leetcode-submit
/// Submit solution
///
/// USAGE:
///     leetcode submit <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct SubmitCommand;

#[async_trait]
impl Command for SubmitCommand {
    /// `submit` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("submit")
            .about("Submit solution")
            .visible_alias("s")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
    }

    /// `submit` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = *m.get_one::<i32>("id").ok_or(Error::NoneError)?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None).await?;

        println!("{}", res);
        Ok(())
    }
}
