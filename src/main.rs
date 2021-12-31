use structopt::StructOpt;
use text_colorizer::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "Polyndrome", about = "Tool that tests input string to be a polyndrome")]
struct Opt {
    /// Input string to test as polyndrome
    #[structopt(short = "i", long = "input")]
    input: String
}

fn invalid_index(input: &str) -> i64 {
    let len = input.len();
    for i in 0 .. (len + 1) / 2 {
        let current = input.chars().nth(i);
        let opposite = input.chars().nth(len - i - 1);
        if current != opposite && i != (len + 1) / 2 {
            return i as i64;
        }
    }

    return -1;
}

#[test]
fn test_short_validation() {
    assert_eq!(invalid_index("ab"), 0);
}

#[test]
fn test_shortest_possible_validation() {
    assert_eq!(invalid_index("axa"), -1);
}

#[test]
fn test_shortest_possible_failing_validation() {
    assert_eq!(invalid_index("abc"), 0);
}

#[test]
fn test_failing_validation() {
    assert_eq!(invalid_index("deadbeeaxfeebdaed"), 7);
}

#[test]
fn test_success_validation() {
    assert_eq!(invalid_index("deadbeefxfeebdaed"), -1);
}

fn check_polyndrome(input: &str) -> Result<bool, String> {
    let len = input.len();
    if len < 3 {
        let msg = format!("'{:?}' is too short for a polyndrome", input);
        return Err(msg);
    }

    if len % 2 == 0 {
        let msg = format!("Even length string '{:?}' can't be a polyndrome", input);
        return Err(msg);
    }

    let index = invalid_index(input);
    if index != -1 {
        let msg = format!("Character at position {} does not have a symmetric opposite character", index);
        return Err(msg);
    }

    return Ok(true);
}

#[test]
fn test_short_input() {
    assert_eq!(check_polyndrome("ab").is_err(), true);
}

#[test]
fn test_even_input() {
    assert_eq!(check_polyndrome("ababab").is_err(), true);
}

#[test]
fn test_valid_input() {
    assert_eq!(check_polyndrome("deadbeefxfeebdaed").is_ok(), true);
}

#[test]
fn test_shortest_valid_input() {
    assert_eq!(check_polyndrome("axa").is_ok(), true);
}

fn main() {
    let opt = Opt::from_args();

    match check_polyndrome(&opt.input) {
        Ok(_) => {
            let msg = format!("'{}' seems to be a valid polyndrome", opt.input);
            eprintln!("{}", msg.green().bold());
        }

        Err(msg) => eprintln!("{}", msg.red().bold())
    }
}
