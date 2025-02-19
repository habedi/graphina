use template_rust_project::cli::run;

fn main() {
    if let Err(code) = run(std::env::args_os()) {
        std::process::exit(code);
    }
}
