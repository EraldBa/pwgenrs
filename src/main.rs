use clap::Parser;

mod pwgen;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct PwOpts {
    #[arg(
        long,
        help = "Exclude lowercase characters",
        default_value_t = false
    )]
    pub x_lower: bool,

    #[arg(
        long,
        help = "Exclude uppercase characters",
        default_value_t = false
    )]
    pub x_upper: bool,

    #[arg(
        long,
        help = "Exclude numerics",
        default_value_t = false
    )]
    pub x_numbers: bool,

    #[arg(
        long,
        help = "Exclude default symbols '!@#$%^&*()<>,./'",
        default_value_t = false
    )]
    pub x_symbols: bool,

    #[arg(
        short = 'l',
        long,
        help = "Password length",
        default_value_t = 15
    )]
    pub pwlen: usize,

    #[arg(
        short,
        long,
        help = "Amount of passwords to generate",
        default_value_t = 12
    )]
    repeat: usize,
}

fn main() {
    let pwopts = PwOpts::parse();

    if pwopts.x_lower && pwopts.x_upper && pwopts.x_numbers && pwopts.x_symbols {
        panic!("All possible seed characters are excluded, cannot generate password!");
    }

    let generator = pwgen::Generator::new(&pwopts);

    let divisor: usize = match pwopts.pwlen {
        0..=25 => 4,
        ..=40 => 3,
        ..=60 => 2,
        _ => 1

    };

    for i in 1..=pwopts.repeat {
        print!("{}  ", generator.generate());
        
        if i % divisor == 0 {
            println!();
        }
    }
}
