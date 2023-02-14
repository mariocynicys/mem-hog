use mem_hog::*;

/// Gets the input from stdin.
pub fn get_input() -> Result<u32, String> {
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(_) => {}
        Err(e) => eprintln!("Error reading the input: {e}"),
    }
    buf.trim()
        .parse()
        .map_err(|e| format!("Couldn't parse the input: {e:?}"))
}

/// Print the available commands.
fn print_commands() {
    use std::io::Write;
    print!(
        "
Available Commands:
    1) Accumulate (1st technique).
    2) Accumulate (2nd technique).
    3) Accumulate (3rd technique).
    4) Perform a `malloc_trim(0)`.
    5) Clear the accumulator.
    6) Reset the accumulator.
    7) Change the insertion amount.
    0) Exit.
Choice: "
    );
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut accumulator = HashMap::new();
    let mut accumulator_size = 0;
    let mut amount = 5_000_000;
    loop {
        print!("Accumulator Size = {accumulator_size}");
        print_commands();
        let input = get_input();
        let start_time = std::time::Instant::now();
        match input {
            Ok(x) if [1, 2, 3].contains(&x) => {
                match x {
                    1 => fill_map_light(&mut accumulator, amount),
                    2 => fill_map_iter(&mut accumulator, amount),
                    3 => fill_map(&mut accumulator, amount),
                    _ => unreachable!(),
                }
                accumulator_size += amount;
            }
            Ok(4) => unsafe {
                if libc::malloc_trim(0) == 1 {
                    println!("Memory released");
                } else {
                    println!("No memory released");
                }
            },
            Ok(5) => {
                accumulator.clear(); // Note that `clear` keeps the used memory allocated for future use.
                accumulator_size = 0;
            }
            Ok(6) => {
                accumulator = HashMap::new();
                accumulator_size = 0;
            }
            Ok(7) => match get_input() {
                Ok(x) => amount = x,
                Err(e) => eprintln!("{e}"),
            },
            Ok(0) => return,
            Ok(x) => eprintln!("Invalid input: {x}"),
            Err(e) => eprintln!("{e}"),
        }
        println!("Executed in {:?}\n", std::time::Instant::now() - start_time);
    }
}