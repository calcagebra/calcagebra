mod ast;
mod compiler;
mod data;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod standardlibrary;
mod token;

use std::{
    ffi::OsStr,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    process::{exit, Command},
    time::Instant,
};

use clap::Parser as ClapParser;
use compiler::Compiler;
use lexer::Lexer;

use crate::{interpreter::Interpreter, parser::Parser, repl::repl};

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Compile the code
    #[clap(long, value_parser)]
    compile: bool,

    /// Output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    /// Only run the lexer and parser
    #[clap(long, value_parser)]
    dry_run: bool,

    /// Emit llvm ir
    #[clap(long, value_parser)]
    emit_ir: bool,

    /// Print the time elapsed while executing code
    #[clap(short, long, value_parser)]
    time: bool,

    /// Print the characters used as globals
    #[clap(short, long, value_parser)]
    globals: bool,

    /// The path of file which is to be executed
    #[clap()]
    input: Option<String>,
}

macro_rules! os {
    ($val:expr) => {{
        OsStr::new($val)
    }};
}

fn main() {
    let args = Args::parse();
    let main = Instant::now();

    if args.input.is_none() {
        if args.globals {
            println!("calcagebra v{}\n", version());
            let _ = Interpreter::new()
                .init_globals()
                .variables
                .iter()
                .map(|(a, b)| println!("{a} {b}"))
                .collect::<Vec<_>>();
            return;
        }
        repl();
    }

    let contents = read_to_string(args.input.clone().unwrap()).unwrap();

    let tokens = Lexer::new(&contents).tokens();

    if args.debug {
        let duration = main.elapsed();
        println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
    }

    let ast = Parser::new(tokens).ast();

    if args.debug {
        let duration = main.elapsed();
        println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
    }

    if args.dry_run {
        return;
    }

    if !args.compile {
        Interpreter::new().run(ast);
        if args.debug || args.time {
            let duration = main.elapsed();
            println!("\nTIME: {duration:?}");
        }
        return;
    }

    let name = args.input.unwrap();
    let module_name = Path::new(&name)
        .file_name()
        .expect("no filename")
        .to_str()
        .expect("invalid filename")
        .strip_suffix(".cal")
        .unwrap_or("<unknown>");

    let llvm_context = inkwell::context::Context::create();
    let mut codegen = Compiler::new(&llvm_context, module_name);

    codegen.declare_functions(&ast);
    codegen.emit_program(&ast).unwrap();

    if args.emit_ir {
        codegen.dump_to_stderr();
        return;
    }

    codegen.verify().unwrap();

    let target_machine = Compiler::create_target_machine().unwrap();

    codegen.optimize(&target_machine).unwrap();

    let target_triple = target_machine.get_triple();
    let target_triple = target_triple
        .as_str()
        .to_str()
        .expect("invalid utf8 in target triple");

    let on_windows = target_triple.contains("windows");

    let object_file_ext = if on_windows { "obj" } else { "o" };
    let object_file_name = format!("{module_name}.{object_file_ext}");
    let object_file = Path::new(&object_file_name);

    codegen.compile(&target_machine, object_file).unwrap();

    let output_file = PathBuf::from(module_name.to_owned());

    let mut link_params = vec![
        object_file.as_os_str(),
        os!("-target"),
        target_triple.as_ref(),
        os!("-o"),
        output_file.as_ref(),
    ];

    if on_windows {
        // See https://learn.microsoft.com/en-us/cpp/porting/visual-cpp-change-history-2003-2015.unwrap()view=msvc-170#stdio_and_conio
        link_params.push(os!("-llegacy_stdio_definitions"));
    }

    let mut child = Command::new("clang").args(&link_params).spawn().unwrap();

    let status = child.wait().unwrap();

    if !status.success() {
        eprintln!("Link failed");
        exit(1);
    }

    let _ = fs::remove_file(object_file);

    if args.debug || args.time {
        let duration = main.elapsed();
        println!("\nTIME: {duration:?}");
    }
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
