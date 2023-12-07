use std::env;

fn get_wholenums_in_range(time: f32, dist: f32) -> i32 {
    let disc_sroot = ((-time).powf(2.) - 4. * dist).sqrt();
    let left_root = (time - disc_sroot) / 2.;
    let right_root = (time + disc_sroot) / 2.;

    let wholenums_in_range = (right_root - (left_root + 1.).floor()).ceil() as i32;

    wholenums_in_range
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 3 {
        println!("Please provide two arguments in the terminal.");
        return Err("Less than two arguments in the terminal.0".into());
    } else if argv.len() % 2 == 0 {
        // Argv with always be > 1.  2 args will have arc of 3.
        println!("Please provide arguments in pairs.");
        return Err("Arguments are not in pairs".into());
    }

    let mut argf: Vec<f32> = vec![];

    for arg in argv.iter().skip(1) {
        match arg.parse::<f32>() {
            Ok(parsed) => argf.push(parsed),
            Err(_) => {
                println!("Failed to parse argument: {}", arg);
                return Err("Failed to parse argument".into());
            }
        }
    }

    let mut total = 1;
    for pair in argf.chunks(2) {
        if let [first, second] = pair {
            let wholenums = get_wholenums_in_range(*first, *second);
            total *= wholenums;
            println!("Pair: ({}, {}) which gets: {}", first, second, wholenums);
        }
    }

    if argf.len() > 2 {
        println!("Giving a product of {}.", total);
    }

    Ok(())
}
