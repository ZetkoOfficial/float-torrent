use crate::{
    error::Result, 
    parse::sequence_provide::{self}, 
    sequnce_providers::OperationSequence
};


pub struct Sequence {}
impl OperationSequence for Sequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "prod".to_owned(),
            description: "Produkt zaporedij. Zaporedje f(n) = a(n) * b(n). Zaporedja: [a, b]".to_owned(),
            parameters: 0,
            sequences: 2
        }
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        Ok(sequences[0] * sequences[1])
    }
}