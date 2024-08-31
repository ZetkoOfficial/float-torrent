use std::vec;
use crate::{error::error::{Error, Result}, parse::sequence_provide::{self}};

use super::SequenceProvider;

/// Zaporedje, ki ga lahko predstavimo kot neka operacija nad istoležečimi členi drugih zaporedih
pub trait OperationSequence : Sync + Send {
    fn apply(&self, parameters: &[f64]) -> Result<f64>;
    fn get_info(&self) -> sequence_provide::SequenceInfo;
}

pub struct OperationSequenceProvider {
    base: Box<dyn OperationSequence>
}

impl OperationSequenceProvider {
    fn combine(&self, length: usize, sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        if !sequences.iter().all(|s| s.len() == length) { 
            Err(Error::sequence_arithmetic_error("Pridobljene dolžine zaporedij se ne ujemajo"))
        } else {
            let mut result = vec![];

            for i in 0..length {
                let collect: Vec<f64> = sequences.iter().map(|s| s[i]).collect();
                result.push(self.base.apply(&collect)?);
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
    fn generate(&self, _range:sequence_provide::Range, _parameters: &[f64],sequences: &[Vec<f64>]) -> Result<Vec<f64> > {
        Ok(self.combine(sequences[0].len(), sequences)?)
    }
}

// ---------- implementacije nekaj posebnih primerov ----------

pub struct SumSequence {}
impl OperationSequence for SumSequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "sum".to_owned(),
            description: "Zaporedje f(n) = a(n) + b(n). Zaporedja: [a, b]".to_owned(),
            parameters: 0,
            sequences: 2
        }
    }

    fn apply(&self, parameters: &[f64]) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error("Napčano število parametrov za funkcijo nad zaporedji"))
        } else {
            Ok(parameters[0] + parameters[1])
        }
    }
}

pub struct ProductSequence {}
impl OperationSequence for ProductSequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "prod".to_owned(),
            description: "Zaporedje f(n) = a(n) * b(n). Zaporedja: [a, b]".to_owned(),
            parameters: 0,
            sequences: 2
        }
    }

    fn apply(&self, parameters: &[f64]) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error("Napčano število parametrov za funkcijo nad zaporedji"))
        } else {
            Ok(parameters[0] * parameters[1])
        }
    }
}
