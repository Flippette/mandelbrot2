use crate::{complex::Complex64, config::Config, lut::Lut};

use std::time::Instant;

use eyre::Result;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[repr(u8)]
pub enum ExitTrace {
    Early(u8),
    Late(u8),
}

#[derive(Debug)]
pub enum RenderError {
    LutParsingFailure,
    MalformedBuffer { expected: usize, received: usize },
}

pub fn render(cfg: &Config) -> Result<Vec<u8>, RenderError> {
    let Ok(lut) = Lut::from_cfg(cfg) else { return Err(RenderError::LutParsingFailure); };

    // isolated to make separating between `parallel` and normal easier
    fn row_iter<'a>(y: u32, cfg: &'a Config, lut: &'a Lut) -> impl Iterator<Item = [u8; 3]> + 'a {
        let h_step = cfg.viewport_size.0 / f64::from(cfg.image_size.0);
        let v_step = cfg.viewport_size.1 / f64::from(cfg.image_size.1);

        (0..cfg.image_size.0).map(move |x| {
            // we know x, y and cfg.image_size are u32s,
            // thus, abs(diff) <= u32::MAX as i64,
            // and so it's fine to cast the difference to f64 (32 < 52)

            #[allow(clippy::cast_precision_loss)]
            let x = (i64::from(x) - i64::from(cfg.image_size.0) / 2) as f64 * h_step
                + cfg.viewport_displacement.0;

            #[allow(clippy::cast_precision_loss)]
            let y = (i64::from(y) - i64::from(cfg.image_size.1) / 2) as f64 * v_step
                + cfg.viewport_displacement.1;

            match trace(x, y, cfg) {
                ExitTrace::Early(steps) => lut.table[steps as usize % lut.table.len()],
                ExitTrace::Late(steps) => [steps; 3],
            }
        })
    }

    let col_iter = 0..cfg.image_size.1 / 2;

    let timer = Instant::now();
    println!("[info] starting rendering...");

    // render top half

    #[cfg(feature = "parallel")]
    let mut render_buf = col_iter
        .into_par_iter()
        .flat_map_iter(|y| row_iter(y, cfg, &lut))
        .flatten_iter()
        .collect::<Vec<_>>();

    #[cfg(not(feature = "parallel"))]
    #[rustfmt::skip] // oh the horror of long lines
    let mut render_buf = col_iter
        .into_iter()
        .flat_map(|y| row_iter(y, cfg, &lut))
        .flatten()
        .collect::<Vec<_>>();

    render_buf.reserve(render_buf.len()); // reserve space for bottom half to avoid allocation
    debug_assert!(render_buf.capacity() >= 2 * render_buf.len());

    // copy & reverse buffer into bottom half
    for i in (0..cfg.image_size.1 / 2).rev() {
        let start = i as usize * cfg.image_size.0 as usize * 3;
        let end = start + cfg.image_size.0 as usize * 3;

        render_buf.extend_from_within(start..end);
    }

    let expected_buf_size = cfg.image_size.0 as usize * cfg.image_size.1 as usize * 3;
    if render_buf.len() == expected_buf_size {
        println!(
            "[info] rendering finished in {}ms.",
            timer.elapsed().as_millis()
        );
        Ok(render_buf)
    } else {
        Err(RenderError::MalformedBuffer {
            expected: expected_buf_size,
            received: render_buf.len(),
        })
    }
}

#[must_use]
pub fn trace(x: f64, y: f64, cfg: &Config) -> ExitTrace {
    let c = Complex64::new(x, y);
    let mut z = Complex64::new(0, 0);

    #[allow(unused_variables)]
    let mut last_z;

    for step in (0..cfg.max_steps).step_by(2) {
        last_z = z.square() + c;
        z = z.square() + c;

        if is_oob(&z, cfg) {
            let mut steps = cfg.max_steps - step;

            if is_oob(&last_z, cfg) {
                steps -= 1;
            }

            return ExitTrace::Early(steps);
        }
    }

    ExitTrace::Late(0) // black is a special case
}

#[inline]
fn is_oob(c: &Complex64, cfg: &Config) -> bool {
    c.re.abs() + cfg.viewport_displacement.0 > cfg.viewport_size.0
        || c.re.abs() + cfg.viewport_displacement.1 > cfg.viewport_size.1
}
