use crate::config::Config;

use eyre::{ensure, Result};
use std::fs;

pub struct Lut {
    pub table: Vec<[u8; 3]>,
}

impl Lut {
    pub fn from_cfg(cfg: &Config) -> Result<Self> {
        let mut table = Vec::new();

        for line in fs::read_to_string(&cfg.lut)?.trim().lines() {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            ensure!(
                tokens.len() == 3,
                "invalid LUT: colors must be formatted in trios"
            );

            #[rustfmt::skip]
            table.push([
                tokens[0].parse()?,
                tokens[1].parse()?,
                tokens[2].parse()?,
            ]);
        }

        Ok(Self { table })
    }
}
