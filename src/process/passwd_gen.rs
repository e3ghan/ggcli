use rand::prelude::*;
use zxcvbn::zxcvbn;

use crate::PassGenOpts;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!\"#$%&'()*+,-./";

pub fn process_passgen(passgen_opts: &PassGenOpts) -> anyhow::Result<String> {
    let mut rng = rand::rng();

    let mut character_pool: Vec<u8> = Vec::new();
    let mut password_bytes: Vec<u8> = Vec::new();

    if !passgen_opts.no_lowercase {
        character_pool.extend_from_slice(LOWERCASE);
        password_bytes.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("lowercase character set is never empty"),
        );
    }

    if !passgen_opts.no_uppercase {
        character_pool.extend_from_slice(UPPERCASE);
        password_bytes.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("uppercase character set is never empty"),
        );
    }

    if !passgen_opts.no_numbers {
        character_pool.extend_from_slice(NUMBER);
        password_bytes.push(
            *NUMBER
                .choose(&mut rng)
                .expect("number character set is never empty"),
        );
    }

    if !passgen_opts.no_symbols {
        character_pool.extend_from_slice(SYMBOL);
        password_bytes.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("symbol character set is never empty"),
        );
    }

    if character_pool.is_empty() {
        anyhow::bail!("at least one character class must be enabled");
    }

    let required_character_count = password_bytes.len();
    let requested_length = passgen_opts.length as usize;

    if requested_length < required_character_count {
        anyhow::bail!(
            "password length ({requested_length}) is shorter than the number of required character classes ({required_character_count})"
        );
    }

    let remaining_character_count = requested_length - required_character_count;
    for _ in 0..remaining_character_count {
        password_bytes.push(
            *character_pool
                .choose(&mut rng)
                .expect("character pool is non-empty"),
        );
    }

    password_bytes.shuffle(&mut rng);

    let password = String::from_utf8(password_bytes)?;

    let password_strength = zxcvbn(&password, &[]);
    eprintln!(
        "the password strength score is: {} (0=weak, 4=strong)",
        password_strength.score()
    );

    Ok(password)
}
