use clap::*;

fn main() {
    let arguments = Command::new("factor")
        .author("Ben Raz <ben.raz2008@gmail.com>")
        .arg(
            Arg::new("number")
                .required(true)
                .help("An integer")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("fancy")
                .long("fancy")
                .short('f')
                .required(false)
                .help("toggles whether you want human-readable output.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let user_number: u64 = *arguments
        .get_one("number")
        .expect("failed to retrieve number!");

    let fancy: bool = *arguments
        .get_one("fancy")
        .expect("could not get argument fancy");

    let mut list_of_used_multiples: Vec<u64> = vec![];

    for number in 1..user_number {
        if !(user_number % number == 0)
            || (list_of_used_multiples.contains(&(user_number / number)))
        {
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
