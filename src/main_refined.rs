use std::env;

#[derive(Debug)]
enum Measure {
    Bytes,
    KB,
    MB,
    GB,
}

impl Measure {
    fn from_str(unit: &str) -> Option<Measure> {
        match unit.to_lowercase().as_str() {
            "b" | "bytes" => Some(Measure::Bytes),
            "kb" => Some(Measure::KB),
            "mb" => Some(Measure::MB),
            "gb" => Some(Measure::GB),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: i64,
}

impl Sizes {
    fn count_all_sizes(val: i64, measurement: Measure) -> Sizes {
        let bytes = match measurement {
            Measure::Bytes => val,
            Measure::KB => val * 1024,
            Measure::MB => val * 1024 * 1024,
            Measure::GB => val * 1024 * 1024 * 1024,
        };
        Sizes { bytes }
    }

    fn display_sizes(&self) {
        let kb = self.bytes / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;

        println!(
            "Size: {} B, {} KB, {} MB, {} GB",
            format_number(self.bytes),
            format_number(kb),
            format_number(mb),
            format_number(gb)
        );
    }
}

// Helper function to format numbers with spaces for readability
fn format_number(num: i64) -> String {
    let num_str = num.to_string();
    let mut formatted = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            formatted.insert(0, ' ');
        }
        formatted.insert(0, c);
        count += 1;
    }
    formatted
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run \"<number> <unit>\"");
        std::process::exit(1);
    }

    let parts: Vec<&str> = args[1].split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Error: Please provide input in the format \"<number> <unit>\", e.g., \"12 mb\"");
        std::process::exit(1);
    }

    let number: i64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Could not parse number from '{}'", parts[0]);
            std::process::exit(1);
        }
    };

    let unit = parts[1];

    let measure = match Measure::from_str(unit) {
        Some(m) => m,
        None => {
            eprintln!("Error: Unknown unit '{}'. Supported units: b, kb, mb, gb", unit);
            std::process::exit(1);
        }
    };

    let size: Sizes = Sizes::count_all_sizes(number, measure);

    size.display_sizes();
}
