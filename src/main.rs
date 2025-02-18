use std::env;

#[derive(Debug)]
enum Measure {
    Bytes,
    KB,
    MB,
    GB
}

impl Measure {
    fn from_str(val: &str) -> Option<Measure> {
        match val.to_lowercase().as_str() {
            "bytes" | "b" => Some(Measure::Bytes),
            "kb" => Some(Measure::KB),
            "mb" => Some(Measure::MB),
            "gb" => Some(Measure::GB),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: i64
}

impl Sizes {
    fn convert_to_bytes(value: i64, measurement: Measure) -> Sizes{
        let bytes = match measurement {
            Measure::Bytes => value,
            Measure::KB => value * 1024,
            Measure::MB => value * 1024 * 1024,
            Measure::GB => value * 1024 * 1024 * 1024,
        };
        Sizes { bytes }
    }

    fn display_sizes(&self) {
        let kb = self.bytes / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;

        println!(
            "{} b, {} kb, {} mb, {} gb", self.bytes, kb, mb, gb
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split_whitespace().collect();
    let value: i64 = match input[0].parse(){
        Ok(num) => num,
        Err(_) => panic!("Hoba")
    };
    let unit = match Measure::from_str(input[1]){
        Some(m) => m,
        None => panic!("Hoba2")
    };
    
    let size: Sizes = Sizes::convert_to_bytes(value, unit);

    size.display_sizes();
    
}