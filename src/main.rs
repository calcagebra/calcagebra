mod ast;
mod compiler;
mod lexer;
mod parser;
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

use crate::parser::Parser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
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
    codegen.emit_main(&ast).unwrap();

    if args.emit_ir {
        codegen.write_ir(format!("{module_name}.ll"));
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
