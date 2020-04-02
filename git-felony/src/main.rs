use regex::Regex;
use std::io::stdin;
use std::mem::swap;
use std::process::exit;

enum State {
    Ok,
    Skip,
}

fn parse_args() {
    for var in std::env::args().skip(1) {
        match var.as_str() {
            "-v" | "--version" => {
                println!(env!("CARGO_PKG_VERSION"));
                // git-felony::skip-next
                exit(0);
            }
            "-h" | "--help" => {
                println!(
                    r#"Usage: git-felony [-v] [-h]
Pipe git diff output to git-felony which will print felonies to standard error and return the number of offenders found.
For example: `git diff --cached | git-felony`
You can skip lines by adding `git-felony::skip-next` to the previous line
For example:
```
// git-felony::skip-next
exit(0)
```

Arguments:
{:arg$}: Prints this help message
{:arg$}: Prints the version number
"#,
                    "-h, --help",
                    "-v, --version",
                    arg = 15
                );
                // git-felony::skip-next
                exit(0);
            }
            _ => {
                eprintln!("Warning: unrecognised command line argument: {}", var);
            }
        }
    }
}

fn main() {
    parse_args();

    let mut file = String::new();
    let mut chunk = String::new();
    let mut input = String::with_capacity(256);
    // begins with a separator followed by a single letter then a single ':' then a path separator
    let win_path_re = Regex::new(r#"("|'|\s|,)([a-z]|[A-Z]){1}:(/|\\|\\\\)"#).unwrap();
    // TODO: create Exit state to be able to parse multi-line exit statements
    let exit_re = Regex::new(r#"exit\s*\("#).unwrap();
    let mut felonies = 0;
    let mut state = State::Ok;
    'mainloop: loop {
        input.clear();
        match stdin().read_line(&mut input) {
            Ok(0) => break,
            Err(e) => eprintln!("failed to read line {:?}", e),
            _ => {
                if input.starts_with("+++ b/") {
                    state = State::Ok;
                    swap(&mut file, &mut input);
                } else if input.starts_with("@@") {
                    state = State::Ok;
                    swap(&mut chunk, &mut input);
                } else if input.starts_with("+") {
                    if let State::Skip = state {
                        state = State::Ok;
                        continue 'mainloop;
                    }
                    if win_path_re.is_match(&input) || exit_re.is_match(&input) {
                        felonies += 1;
                        eprintln!(
                            "Looks like you're about to commit a felony here:\n{:width$}| {}{:width$}| {}{:width$}| {}",
                            "", &file[6..],"", &chunk,"", &input[1..], width=4
                        );
                    }
                    if input.contains("git-felony::skip-next") {
                        state = State::Skip // skip the next line.
                    }
                }
            }
        }
    }
    eprintln!("Found {} felonies", felonies);
    // git-felony::skip-next
    exit(felonies)
}
