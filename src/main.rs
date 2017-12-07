// Copyright (C) 2017 Christopher R. Field.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// TODO: Add module-level documentation

extern crate ansi_term;
extern crate atty;
extern crate cargo_wix;
#[macro_use] extern crate clap;
extern crate loggerv;

use ansi_term::Colour;
use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::io::Write;

const SUBCOMMAND_NAME: &str = "wix";
const ERROR_COLOR: Colour = Colour::Fixed(9); // Bright red

fn main() {
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first. The ansi_term crate provides a function for enabling ANSI
    // support in Windows, but it is conditionally compiled and only exists for Windows builds. To
    // avoid build errors on non-windows platforms, a cfg guard should be put in place.
    if atty::is(atty::Stream::Stdout) {
        #[cfg(windows)] ansi_term::enable_ansi_support().expect("Enable ANSI support on Windows");
    }
    let matches = App::new(crate_name!())
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name(SUBCOMMAND_NAME)
                .version(crate_version!())
                .about(crate_description!())
                .author(crate_authors!())
                .arg(Arg::with_name("binary-name")
                     .help("Overrides the 'name' field of the bin section of the package's manifest (Cargo.toml) as the name of the executable within the installer.")
                     .long("binary-name")
                     .short("b")
                     .takes_value(true))
                .arg(Arg::with_name("description")
                     .help("Overrides the 'description' field of the package's manifest (Cargo.toml) as the description within the installer.")
                     .long("description")
                     .short("d")
                     .takes_value(true))
                .arg(Arg::with_name("force")
                     .help("Overwrites any existing WiX Source files when using the '--init' flag. Use with caution.")
                     .long("force")
                     .requires("init"))
                .arg(Arg::with_name("init")
                     .help("Initializes the package to be used with this subcommand. This creates a 'wix` sub-folder within the root folder of the package and creates a 'main.wxs' WiX Source (wxs) file within the 'wix' sub-folder from the embedded template. The 'wix\\main.wxs' file that is created can immediately be used with this subcommand without modification to create an installer for the project.")
                     .long("init"))
                .arg(Arg::with_name("manufacturer")
                     .help("Overrides the first author in the 'authors' field of the package's manifest (Cargo.toml) as the manufacturer within the installer.")
                     .long("manufacturer")
                     .short("m")
                     .takes_value(true))
                .arg(Arg::with_name("no-capture")
                     .help("By default, this subcommand captures, or hides, all output from the builder, compiler, linker, and signer for the binary and Windows installer, respectively. Use this flag to show the output.")
                     .long("nocapture"))
                .arg(Arg::with_name("print-template")
                     .help("Prints a template WiX Source (wxs) file to use with this subcommand to stdout. The template provided with this subcommand uses xml preprocessor varaibles to set values based on fields in the rust project's manifest file (Cargo.toml). Only the '{{replace-with-a-guid}}' placeholders within the template need to be modified with unique GUIDs by hand. Redirection can be used to save the contents to 'main.wxs' and then placed in the 'wix' subfolder.")
                     .long("print-template"))
                .arg(Arg::with_name("product-name")
                     .help("Overrides the 'name' field of the package's manifest (Cargo.toml) as the product name within the installer.")
                     .long("product-name")
                     .short("p")
                     .takes_value(true))
                .arg(Arg::with_name("sign")
                     .help("The Windows installer (msi) will be signed using the SignTool application available in the Windows 10 SDK. The signtool is invoked with the '/a' flag to automatically obtain an appropriate certificate from the Windows certificate manager. The default is to also use the Comodo timestamp server with the '/t' flag.")
                     .short("s")
                     .long("sign"))
                .arg(Arg::with_name("timestamp")
                     .help("The URL for the timestamp server used with the 'signtool' to sign the installer. This can only be used with the '-s,--sign' flag.")
                     .short("t")
                     .long("timestamp")
                     .takes_value(true)
                     .requires("sign"))
                .arg(Arg::with_name("verbose")
                     .help("Sets the level of verbosity. The higher the level of verbosity, the more information that is printed and logged when the application is executed. This flag can be specified multiple times, where each occurrance increases the level and/or details written for each statement.")
                     .long("verbose")
                     .short("v")
                     .multiple(true))
                .arg(Arg::with_name("INPUT")
                     .help("A WiX Source (wxs) file. The default is to use the 'wix\\main.wxs' file.")
                     .index(1))
        ).get_matches();
    let matches = matches.subcommand_matches(SUBCOMMAND_NAME).unwrap();
    let verbosity = matches.occurrences_of("verbose");
    if verbosity > 3 {
        loggerv::Logger::new()
            .line_numbers(true)
            .module_path(true)
    } else {
        loggerv::Logger::new()
            .module_path(false)
    }.verbosity(verbosity)
    .level(true)
    .init()
    .expect("logger to initiate");
    let result = if matches.is_present("init") {
        cargo_wix::init(matches.is_present("force"))
    } else if matches.is_present("print-template") {
        cargo_wix::print_template()
    } else {
        cargo_wix::Wix::new()
            .binary_name(matches.value_of("binary-name"))
            .capture_output(!matches.is_present("no-capture"))
            .description(matches.value_of("description"))
            .input(matches.value_of("INPUT"))
            .manufacturer(matches.value_of("manufacturer"))
            .product_name(matches.value_of("product-name"))
            .sign(matches.is_present("sign"))
            .timestamp(matches.value_of("timestamp"))
            .run()
    };
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            let mut tag = format!("Error[{}] ({})", e.code(), e.description());
            if atty::is(atty::Stream::Stderr) {
                tag = ERROR_COLOR.paint(tag).to_string()
            }
            writeln!(&mut std::io::stderr(), "{}: {}", tag, e)
                .expect("Writing to stderr");
            std::process::exit(e.code());
        }
    }
}

