use anyhow::Context as _;

mod actions;
mod commands;
mod sock;

fn main() {
    let matches = clap::App::new("rbw")
        .about("unofficial bitwarden cli")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .subcommand(
            clap::SubCommand::with_name("config")
                .subcommand(clap::SubCommand::with_name("show"))
                .subcommand(
                    clap::SubCommand::with_name("set")
                        .arg(clap::Arg::with_name("key").required(true))
                        .arg(clap::Arg::with_name("value").required(true)),
                ),
        )
        .subcommand(clap::SubCommand::with_name("login"))
        .subcommand(clap::SubCommand::with_name("unlock"))
        .subcommand(clap::SubCommand::with_name("sync"))
        .subcommand(clap::SubCommand::with_name("list"))
        .subcommand(
            clap::SubCommand::with_name("get")
                .arg(clap::Arg::with_name("name").required(true))
                .arg(clap::Arg::with_name("user")),
        )
        .subcommand(
            clap::SubCommand::with_name("add")
                .arg(clap::Arg::with_name("name").required(true))
                .arg(clap::Arg::with_name("user").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("generate")
                .arg(clap::Arg::with_name("len").required(true))
                .arg(clap::Arg::with_name("name"))
                .arg(clap::Arg::with_name("user"))
                .arg(clap::Arg::with_name("no-symbols").long("no-symbols"))
                .arg(
                    clap::Arg::with_name("only-numbers").long("only-numbers"),
                )
                .arg(
                    clap::Arg::with_name("nonconfusables")
                        .long("nonconfusables"),
                )
                .arg(clap::Arg::with_name("diceware").long("diceware"))
                .group(clap::ArgGroup::with_name("password-type").args(&[
                    "no-symbols",
                    "only-numbers",
                    "nonconfusables",
                    "diceware",
                ])),
        )
        .subcommand(clap::SubCommand::with_name("edit"))
        .subcommand(clap::SubCommand::with_name("remove"))
        .subcommand(clap::SubCommand::with_name("lock"))
        .subcommand(clap::SubCommand::with_name("purge"))
        .subcommand(clap::SubCommand::with_name("stop-agent"))
        .get_matches();

    let res = match matches.subcommand() {
        ("config", Some(smatches)) => match smatches.subcommand() {
            ("show", Some(_)) => {
                commands::config_show().context("config show")
            }
            // these unwraps are fine because key and value are both marked
            // .required(true)
            ("set", Some(ssmatches)) => commands::config_set(
                ssmatches.value_of("key").unwrap(),
                ssmatches.value_of("value").unwrap(),
            )
            .context("config set"),
            _ => {
                eprintln!("{}", smatches.usage());
                std::process::exit(1);
            }
        },
        ("login", Some(_)) => commands::login().context("login"),
        ("unlock", Some(_)) => commands::unlock().context("unlock"),
        ("sync", Some(_)) => commands::sync().context("sync"),
        ("list", Some(_)) => commands::list().context("list"),
        // this unwrap is safe because name is marked .required(true)
        ("get", Some(smatches)) => commands::get(
            smatches.value_of("name").unwrap(),
            smatches.value_of("user"),
        )
        .context("get"),
        // this unwrap is safe because name is marked .required(true)
        ("add", Some(smatches)) => commands::add(
            smatches.value_of("name").unwrap(),
            smatches.value_of("user"),
        )
        .context("add"),
        ("generate", Some(smatches)) => {
            let ty = if smatches.is_present("no-symbols") {
                rbw::pwgen::Type::NoSymbols
            } else if smatches.is_present("only-numbers") {
                rbw::pwgen::Type::Numbers
            } else if smatches.is_present("nonconfusables") {
                rbw::pwgen::Type::NonConfusables
            } else if smatches.is_present("diceware") {
                rbw::pwgen::Type::Diceware
            } else {
                rbw::pwgen::Type::AllChars
            };
            // this unwrap is fine because len is marked as .required(true)
            let len = smatches.value_of("len").unwrap();
            match len.parse() {
                Ok(len) => commands::generate(
                    smatches.value_of("name"),
                    smatches.value_of("user"),
                    len,
                    ty,
                )
                .context("generate"),
                Err(e) => Err(e.into()),
            }
        }
        ("edit", Some(_)) => commands::edit().context("edit"),
        ("remove", Some(_)) => commands::remove().context("remove"),
        ("lock", Some(_)) => commands::lock().context("lock"),
        ("purge", Some(_)) => commands::purge().context("purge"),
        ("stop-agent", Some(_)) => {
            commands::stop_agent().context("stop-agent")
        }
        _ => {
            eprintln!("{}", matches.usage());
            std::process::exit(1);
        }
    }
    .context("rbw");

    if let Err(e) = res {
        eprintln!("{:#}", e);
        std::process::exit(1);
    }
}
