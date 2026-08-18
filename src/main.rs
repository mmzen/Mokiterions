use std::env;
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;

use mokiterions::execute;

fn main() -> ExitCode {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut stderr = BufWriter::new(stderr.lock());

    let mut code = execute(env::args().skip(1), &mut stdout, &mut stderr);
    if stdout.flush().is_err() {
        code = 1;
    }
    let _ = stderr.flush();
    ExitCode::from(code)
}
