mod cli;
mod url_handler;

fn main() {
    cli::get_matches_and_run_command().unwrap();
}
