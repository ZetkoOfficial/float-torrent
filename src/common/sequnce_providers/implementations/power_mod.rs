use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self, SequenceInfo}, 
    sequnce_providers::SequenceProvider
};

pub struct Provider {}
impl SequenceProvider for Provider {

    fn get_info(&self) -> SequenceInfo {
        SequenceInfo {
            name: "power_mod".to_owned(),
            description: "f(n) = (a^(p^n) mod M), Parametri [a >= 0, p >= 0, M > 0] so truncirani u32.". to_owned(),
            parameters: 3,
            sequences: 0
        }
    }

    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], _sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        let mut result = vec![];
        if parameters[0] < 0. || parameters[1] < 0. || parameters[2] <= 0.  { return Err(Error::sequence_arithmetic_error("Neveljavni parametri")); }        
        let (mut a, p, m): (u32, u32, u32) = (parameters[0].trunc() as u32, parameters[1].trunc() as u32, parameters[2].trunc() as u32);

        let mut i = 0; 
        while i < range.from {
            a = a.wrapping_pow(p) % m;
            i += 1;
        }

        let p = p.pow(range.step.try_into()?);
        while i < range.to {
            result.push(a);
            a = a.wrapping_pow(p) % m;
            i += range.step;
        }

        Ok(result.iter().map(|v| *v as f64).collect())
    }
}