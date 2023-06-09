use clap::{ArgAction::SetTrue, Parser};

#[derive(Parser, Debug)]
#[command(name = "factor", author = "Ben Raz <ben.raz2008@gmail.com>")]
struct Args {
    #[arg(help = "An unsigned integer")]
    number: u64,
    #[arg(short, long)]
    #[arg(help="toggles human-readable output", action=SetTrue)]
    fancy: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let user_number: u64 = args.number;

    let fancy: bool = match args.fancy {
        Some(value) => value,
        None => false,
    };

    let mut list_of_used_multiples: Vec<u64> = vec![];

    for number in 1..user_number {
        let is_divisible_by_current_number: bool = user_number % number == 0;
        let already_divided = list_of_used_multiples.contains(&(user_number / number));

        if !is_divisible_by_current_number || already_divided {
            continue;
        }

        if !fancy {
            println!("({}, {})", number, user_number / number);
        } else {
            println!(
                "{} is divisible by {} and {}.",
                user_number,
                number,
                user_number / number
            );
        }
        list_of_used_multiples.push(number);
    }
}
