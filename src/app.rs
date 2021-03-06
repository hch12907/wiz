use clap::{ App, Arg, ArgMatches, SubCommand };

pub fn run<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Wiz is a package manager.")
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs package.")
                .arg(
                    Arg::with_name("package_name")
                    .help("Specifies package to install")
                )

                .arg(
                    Arg::with_name("force")
                    .short("f")
                    .long("force")
                    .help("Forces the installation of the package to commence")
                )
        )
    .get_matches()
}