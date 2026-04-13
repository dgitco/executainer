use executainer::cli::run_app;

fn main() {
    if let Err(err) = run_app() {
        eprintln!("{err}");
        std::process::exit(err.exit_code());
    }
}
