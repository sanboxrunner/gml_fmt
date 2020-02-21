mod config;
mod expressions;
mod lang_config;
mod lex_token;
mod parser;
mod printer;
mod scanner;
mod statements;

use anyhow::Result as AnyResult;
use parser::Parser;
use printer::Printer;
use std::fs;

pub use config::{Config, PrintFlags};
pub use lang_config::LangConfig;

pub fn run_config(config: &Config, lang_config: &LangConfig) -> AnyResult<()> {
    let log = config.print_flags.contains(PrintFlags::LOGS);
    // let log_scan = config.print_flags.contains(PrintFlags::SCANNER_LOGS);
    let overwrite = config.print_flags.contains(PrintFlags::OVERWRITE);

    for this_file in &config.files {
        let contents = fs::read_to_string(this_file)?;

        if contents.contains("// @gml_fmt ignore") {
            continue;
        }

        if log {
            println!("=========INPUT=========");
            println!("{}", contents);
        }

        let res = run(&contents, lang_config, log);
        if let Some(err) = res.0 {
            println!("Could not parse file {:?}", this_file);
            println!("{}", err);
        } else {
            let output = res.1.unwrap();
            if log {
                println!("=========OUTPUT=========");
                println!("{}", output);
                println!("==========AST===========");
                println!("{}", res.2.unwrap());
            }

            if overwrite {
                fs::write(this_file, output)?;
            }
        }
    }

    Ok(())
}

pub fn run_test(input: &str) -> String {
    let res = run(input, &LangConfig::default(), false);
    if let Some(err) = res.0 {
        println!("{}", err);
        return input.to_owned();
    }
    return res.1.unwrap();
}

fn run(source: &str, lang_config: &LangConfig, print_ast: bool) -> (Option<String>, Option<String>, Option<String>) {
    let source_size = source.len();
    let mut parser = Parser::new(source);
    parser.build_ast();
    if let Some(err) = parser.failure {
        return (Some(err), None, None);
    }

    let printer = Printer::new(source_size / 2, lang_config).autoformat(&parser.ast[..]);

    (
        None,
        Some(Printer::get_output(&printer.output, source_size)),
        if print_ast {
            Some(format!("{:#?}", parser.ast))
        } else {
            None
        },
    )
}
