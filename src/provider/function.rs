use std::vec;
use crate::{error::error::{Error, Result}, parse::sequence_provide::{self}};

use super::SequenceProvider;

pub trait FunctionSequence : Sync + Send {
    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64>;
    fn get_info(&self) -> sequence_provide::SequenceInfo;
}

pub struct FunctionSequenceProvider {
    base: Box<dyn FunctionSequence>
}

impl FunctionSequenceProvider {
    pub fn new(base: Box<dyn FunctionSequence>) -> Self {
        FunctionSequenceProvider { base }
    }
}

impl SequenceProvider for FunctionSequenceProvider {
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        self.base.get_info()
    }

    fn generate(&self,range:sequence_provide::Range,parameters: &[f64], _: &[Vec<f64>]) -> Result<Vec<f64> > {
        let mut result = vec![];        

        let mut i = range.from; 
        while i < range.to {
            result.push(self.base.evaluate(parameters, i)?); i += range.step;
        }

        Ok(result)
    }
}

// ---------- implementacije nekaj posebnih primerov ----------

pub struct ArithmeticSequence {}
impl FunctionSequence for ArithmeticSequence {
    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error("Potrebna sta parametra `a0` in `d`."))
        } else {
            Ok(parameters[0] + parameters[1] * (n as f64))
        }
    }
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "arithmetic".to_owned(),
            description: "Aritmetično zaporedje a(n) = a(0) + d * n, kjer je `a(0)` prvi in `d` drugi parameter".to_owned(),
            parameters: 2,
            sequences: 0
        }
    }
}

pub struct GeometricSequence {}
impl FunctionSequence for GeometricSequence {
    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error("Potrebna sta parametra `a0` in `q`."))
        } else {
            Ok(parameters[0] * parameters[1].powf(n as f64))
        }
    }
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "geometric".to_owned(),
            description: "Geometrijsko zaporedje g(n) = g(0) * q^n, kjer je `g(0)` prvi in `q` drugi parameter".to_owned(),
            parameters: 2,
            sequences: 0
        }
    }
}