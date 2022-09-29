use clap::{Command, Arg, ArgAction, builder::NonEmptyStringValueParser};

fn main() {
    let app = Command::new("echor")
        .version("0.1.0")
        .author("Debojit Sinha")
        .about("Rusty echo app")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
                .value_parser(NonEmptyStringValueParser::new())
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("omit-newline")
                .help("Do not print newline")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let text = app.get_many::<String>("text").unwrap().map(|data| data.to_owned()).collect::<Vec<String>>().join(" ");
    let omit_newline = app.get_flag("omit_newline");

    let end = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.as_str(), end);
}

#[cfg(test)]
mod echor_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;
    #[test]
    fn panic_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
        Ok(())
    }

    #[test]
    fn run_ok() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.arg("Hello").assert().success();
        Ok(())
    }

    #[test]
    fn hello1() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.args(vec!["Hello there"]).assert().success().stdout("Hello there\n");
        Ok(())
    }

    #[test]
    fn hello2() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.args(vec!["Hello", "there"]).assert().success().stdout("Hello there\n");
        Ok(())
    }

    #[test]
    fn hello1n() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.args(vec!["-n", "Hello there"]).assert().success().stdout("Hello there");
        Ok(())
    }

    #[test]
    fn hello2n() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.args(vec!["-n", "Hello", "there"]).assert().success().stdout("Hello there");
        Ok(())
    }
}
