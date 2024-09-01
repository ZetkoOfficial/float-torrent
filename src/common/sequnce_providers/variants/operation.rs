use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self}, 
    sequnce_providers::SequenceProvider
};

/// Zaporedje, ki ga lahko predstavimo kot neka operacija nad istoležečimi členi drugih zaporedih
pub trait OperationSequence : Sync + Send {
    fn apply(&self, parameters: &[f64], sequences: &[f64]) -> Result<f64>;
    fn get_info(&self) -> sequence_provide::SequenceInfo;
}

pub struct OperationSequenceProvider {
    base: Box<dyn OperationSequence>
}

impl OperationSequenceProvider {
    fn combine(&self, length: usize, parameters: &[f64], sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        if !sequences.iter().all(|s| s.len() == length) { 
            Err(Error::sequence_arithmetic_error("Pridobljene dolžine zaporedij se ne ujemajo"))
        } else {
            let mut result = vec![];

            for i in 0..length {
                let collect: Vec<f64> = sequences.iter().map(|s| s[i]).collect();
                result.push(self.base.apply(parameters, &collect)?);
            }
            
            Ok(result)
        }

    }

    pub fn new(base: Box<dyn OperationSequence>) -> Self {
        OperationSequenceProvider {
            base
        }
    } 
}

impl SequenceProvider for OperationSequenceProvider {
    fn get_info(&self) -> sequence_provide::SequenceInfo { self.base.get_info().clone() }
    fn generate(&self, _range:sequence_provide::Range, parameters: &[f64],sequences: &[Vec<f64>]) -> Result<Vec<f64> > {
        Ok(self.combine(sequences[0].len(), parameters, sequences)?)
    }
}