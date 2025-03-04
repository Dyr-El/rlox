use std::{env, fs, io::{self, Write}, process::ExitCode};

mod chunk;
mod compiler;
mod disassembler;
mod scanner;
mod values;
mod virtual_machine;

fn repl(vm: &mut virtual_machine::VM) {
    let mut line = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().expect("Fluch fail");
        line.clear();
        if let Err(_) = stdin.read_line(&mut line) {
            println!("");
            break;
        }
        if line == "" {
            break;
        }
        if let Err(e) = vm.interpret(&line) {
            match e {
                virtual_machine::InterpretError::CompileError => eprintln!("Compilation error!"),
                virtual_machine::InterpretError::RuntimeError => eprintln!("Runtime error!"),
            }
        }
    }
}

fn run_file(vm: &mut virtual_machine::VM, path: &str) -> Result<(), ExitCode> {
    let contents = fs::read_to_string(path);
    match contents {
        Err(_) => {
            Err(ExitCode::from(74))
        },
        Ok(content) => {
            let result = vm.interpret(&content);
            if let Err(_) = result {
                return Err(ExitCode::FAILURE);
            }
            Ok(())
        },
    }
}

pub fn run() -> ExitCode {
    let mut vm = virtual_machine::VM::new();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        if let Err(ec) = run_file(&mut vm, &args[1]) {
            return ec;
        }
    } else {
        eprintln!("Usage: {} <cmd> <filename>", args[0]);
        eprintln!("          <cmd> is one of tokenize or parse");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
