use rand::{rngs::OsRng, Rng};

use crate::PwOpts;

pub struct Generator {
    seed: Vec<char>,
    pwlen: usize,
}

impl Generator {
    pub fn new(pwopts: &PwOpts) -> Generator {
        let mut seed: Vec<char> = vec![];

        if !pwopts.x_lower {
            seed.extend('a'..='z');
        }

        if !pwopts.x_upper {
            seed.extend('A'..='Z');
        }

        if !pwopts.x_numbers {
            seed.extend('0'..='9');
        }

        if !pwopts.x_symbols {
            seed.extend("!@#$%^&*()<>,./".chars());
        }

        return Generator {
            pwlen: pwopts.pwlen,
            seed,
        };
    }

    pub fn generate(&self) -> String {
        let mut rng = OsRng::default();
        let use_unique_chars = self.pwlen < self.seed.len();

        if !use_unique_chars {
            return (0..self.pwlen)
                .map(|_| {
                    let index = rng.gen_range(0..self.seed.len());
                    self.seed[index]
                })
                .collect();
        }

        let mut password = String::with_capacity(self.pwlen);

        for _ in 0..self.pwlen {
            let mut ch: char;

            loop {
                let index = rng.gen_range(0..self.seed.len());
                ch = self.seed[index];

                if !password.contains(ch) {
                    break;
                }
            }

            password.push(ch);
        }

        password
    }
}
