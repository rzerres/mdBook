extern crate mdbook;
#[macro_use]
extern crate clap;
extern crate log;
extern crate env_logger;
extern crate open;

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{App, ArgMatches, SubCommand, AppSettings};

pub mod build;
#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "watch")]
pub mod watch;

use mdbook::MDBook;

const NAME: &'static str = "mdbook";

fn main() {
    env_logger::init().unwrap();

    // Create a list of valid arguments and sub-commands
    let app = App::new(NAME)
                    .about("Create a book in form of a static website from markdown files")
                    .author("Mathieu David <mathieudavid@mathieudavid.org>")
                    // Get the version from our Cargo.toml using clap's crate_version!() macro
                    .version(concat!("v",crate_version!()))
                    .setting(AppSettings::SubcommandRequired)
                    .after_help("For more information about a specific command, try `mdbook <command> --help`\nSource code for mdbook available at: https://github.com/azerupi/mdBook")
                    .subcommand(SubCommand::with_name("init")
                        .about("Create boilerplate structure and files in the directory")
                        // the {n} denotes a newline which will properly aligned in all help messages
                        .arg_from_usage("[dir] 'A directory for your book{n}(Defaults to Current Directory when omitted)'")
                        .arg_from_usage("--theme 'Copies the default theme into your source folder'")
                        .arg_from_usage("--force 'skip confirmation prompts'"))
                    .subcommand(build::make_subcommand())
                    .subcommand(SubCommand::with_name("test")
                        .about("Test that code samples compile"));

    #[cfg(feature = "watch")]
    let app = app.subcommand(watch::make_subcommand());
    #[cfg(feature = "serve")]
    let app = app.subcommand(serve::make_subcommand());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        ("init", Some(sub_matches)) => init(sub_matches),
        ("build", Some(sub_matches)) => build::build(sub_matches),
        #[cfg(feature = "watch")]
        ("watch", Some(sub_matches)) => watch::watch(sub_matches),
        #[cfg(feature = "serve")]
        ("serve", Some(sub_matches)) => serve::serve(sub_matches),
        ("test", Some(sub_matches)) => test(sub_matches),
        (_, _) => unreachable!(),
    };

    if let Err(e) = res {
        writeln!(&mut io::stderr(), "An error occured:\n{}", e).ok();
        ::std::process::exit(101);
    }
}


// Simple function that user comfirmation
fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    match &*s.trim() {
        "Y" | "y" | "yes" | "Yes" => true,
        _ => false,
    }
}


// Init command implementation
fn init(args: &ArgMatches) -> Result<(), Box<Error>> {

    let book_dir = get_book_dir(args);
    let mut book = MDBook::new(&book_dir);

    // Call the function that does the initialization
    book.init()?;

    // If flag `--theme` is present, copy theme to src
    if args.is_present("theme") {

        // Skip this if `--force` is present
        if !args.is_present("force") {
            // Print warning
            print!("\nCopying the default theme to {:?}", book.get_source());
            println!("could potentially overwrite files already present in that directory.");
            print!("\nAre you sure you want to continue? (y/n) ");

            // Read answer from user and exit if it's not 'yes'
            if !confirm() {
                println!("\nSkipping...\n");
                println!("All done, no errors...");
                ::std::process::exit(0);
            }
        }

        // Call the function that copies the theme
        book.copy_theme()?;
        println!("\nTheme copied.");

    }

    // Because of `src/book/mdbook.rs#L37-L39`, `dest` will always start with `root`
    let is_dest_inside_root = book.get_destination()
                                  .map(|p| p.starts_with(book.get_root()))
                                  .unwrap_or(false);

    if !args.is_present("force") && is_dest_inside_root {
        println!("\nDo you want a .gitignore to be created? (y/n)");

        if confirm() {
            book.create_gitignore();
            println!("\n.gitignore created.");
        }
    }

    println!("\nAll done, no errors...");

    Ok(())
}

fn test(args: &ArgMatches) -> Result<(), Box<Error>> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::new(&book_dir).read_config()?;

    book.test()?;

    Ok(())
}


fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().unwrap()
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    if let Err(e) = open::that(path) {
        println!("Error opening web browser: {}", e);
    }
}
