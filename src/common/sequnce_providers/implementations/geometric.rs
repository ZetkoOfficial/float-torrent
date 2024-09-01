use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self}, 
    sequnce_providers::FunctionSequence
};
pub struct Sequence {}
impl FunctionSequence for Sequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "geometric".to_owned(),
            description: "Geometrijsko zaporedje g(n) = g0 * q^n. Parametri: [g0, q]".to_owned(),
            parameters: 2,
            sequences: 0
        }
    }

    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error(self.get_info(),"Potrebna sta parametra `a0` in `q`."))
        } else {
            Ok(parameters[0] * parameters[1].powf(n as f64))
        }
    }
}