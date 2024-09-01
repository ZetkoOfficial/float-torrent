use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self}, 
    sequnce_providers::OperationSequence
};

pub struct Sequence {}
impl OperationSequence for Sequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "round".to_owned(),
            description: "Zaokroženo zaporedje na p decimalk(lahko tudi negativno). Zaporedje f(n) = 10^(-p) * round((10^p) * a(n)) Parametri: [p], Zaporedja: [a]".to_owned(),
            parameters: 1,
            sequences: 1
        }
    }

    fn apply(&self, parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        if sequences.len() != 1 || parameters.len() != 1 {
            Err(Error::sequence_arithmetic_error("Številu parametrov ali zaporedij je nepravilno."))
        } else {
            let factor = (10 as f64).powf(parameters[0]);
            Ok((factor * sequences[0]).round()/factor)
        }
    }
}