use scientific::{Precision, Scientific};
use tracing::debug;
use tracing_subscriber::field::display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Scientific error: {0}")]
    Scientific(#[from] scientific::Error),
    #[error("Conversion error: {0}")]
    ConvertError(#[from] scientific::ConversionError),
}

pub fn trapazoidal_rule(
    func: impl Fn(f64) -> f64,
    bottom_a: f64,
    top_b: f64,
    number: u16,
    precision: Precision,
) -> Result<Scientific, Error> {
    let func = |x: &Scientific| {
        let fin: f64 = x.into();
        let func_output = func(fin);
        Scientific::try_from(func_output)
    };
    let top_b = Scientific::try_from(top_b)?;
    let bottom_a = Scientific::try_from(bottom_a)?;

    let width = (&top_b - &bottom_a).div_rpsp(&Scientific::from(number), precision)?;

    let ends = &func(&bottom_a)? + &func(&top_b)?;
    let middle: Scientific = {
        let middle_num = number - 2;
        let multiples = 1..number - 1;
				assert_eq!(multiples.len(), middle_num as usize, "Multiples length is not equal to middle_num");


        let mut sum = Scientific!(0);
        for m in multiples {
            debug!(%m, "Computing");
            let m = Scientific::from(m);
            let x = &bottom_a + &(&m * &width.div_rpsp(&Scientific::from(number), precision)?);
            sum = &sum + &func(&x)?;
        }

        debug!(%sum, "Finished sum");
        sum
    };

    Ok(
        &(width.div_rpsp(&Scientific!(2.0), precision))?
            * &(&ends + &(&Scientific!(2.0) * &middle)),
    )
}
