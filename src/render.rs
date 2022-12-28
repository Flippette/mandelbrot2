use crate::{complex::Complex64, config::Config, lut::Lut};

use eyre::Result;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub enum ExitTrace {
    Early(u8),
    Late(u8),
}

pub fn render(cfg: &Config) -> Result<Vec<u8>> {
    let lut = Lut::from_cfg(cfg)?;

    // isolated to make separating between `parallel` and normal easier
    fn row_iter<'a>(y: u64, cfg: &'a Config, lut: &'a Lut) -> impl Iterator<Item = [u8; 3]> + 'a {
        (0..cfg.image_size.0).map(move |x| {
            let x = (x as i64 - cfg.image_size.0 as i64 / 2) as f64 * cfg.viewport_size.0
                / cfg.image_size.0 as f64
                + cfg.viewport_displacement.0;
            let y = (y as i64 - cfg.image_size.1 as i64 / 2) as f64 * cfg.viewport_size.1
                / cfg.image_size.1 as f64
                + cfg.viewport_displacement.1;

            match trace(x, y, cfg) {
                ExitTrace::Early(steps) => lut.table[steps as usize % lut.table.len()],
                ExitTrace::Late(steps) => [steps; 3],
            }
        })
    }

    let col_iter = 0..cfg.image_size.1;

    #[cfg(feature = "parallel")]
    return Ok(col_iter
        .into_par_iter()
        .flat_map_iter(|y| row_iter(y, cfg, &lut))
        .flatten_iter()
        .collect());

    #[cfg(not(feature = "parallel"))]
    #[rustfmt::skip]
    return Ok(line_iter
        .into_iter()
        .flat_map(|y| row_iter(y, cfg, &lut))
        .flatten()
        .collect());
}

pub fn trace(x: f64, y: f64, cfg: &Config) -> ExitTrace {
    let c = Complex64::new(x, y);
    let mut z = Complex64::new(0, 0);

    for step in 0..cfg.max_steps {
        z = z.square() + c;

        if z.re.abs() + cfg.viewport_displacement.0 > cfg.viewport_size.0
            || z.re.abs() + cfg.viewport_displacement.1 > cfg.viewport_size.1
        {
            return ExitTrace::Early(cfg.max_steps - step);
        }
    }

    ExitTrace::Late(0) // black is a special case
}
