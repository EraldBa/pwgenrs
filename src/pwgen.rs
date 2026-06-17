use rand::{rngs::OsRng, Rng};

use crate::PwOpts;

pub struct Generator {
    seed: Vec<char>,
    pwlen: usize,
}

impl Generator {
    pub fn new(pwopts: &PwOpts) -> Self {
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

        Self {
            pwlen: pwopts.pwlen,
            seed,
        }
    }

    pub fn generate(&self) -> String {
        let mut rng = OsRng::default();

        return (0..self.pwlen)
            .map(|_| {
                let index = rng.gen_range(0..self.seed.len());
                self.seed[index]
            })
            .collect::<String>();
    }
}
