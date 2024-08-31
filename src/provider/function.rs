use std::vec;
use crate::{error::error::{Error, Result}, parse::sequence_provide::{self, SequenceInfo}};

use super::SequenceProvider;

/// Zaporedje, ki ga lahko predstavimo kot funkcija nekih parametrov in indeksa
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

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "arithmetic".to_owned(),
            description: "Aritmetično zaporedje a(n) = a0 + d * n. Parametri: [a0, d]".to_owned(),
            parameters: 2,
            sequences: 0
        }
    }

    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error("Potrebna sta parametra `a0` in `d`."))
        } else {
            Ok(parameters[0] + parameters[1] * (n as f64))
        }
    }
}

pub struct GeometricSequence {}
impl FunctionSequence for GeometricSequence {

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
            Err(Error::sequence_arithmetic_error("Potrebna sta parametra `a0` in `q`."))
        } else {
            Ok(parameters[0] * parameters[1].powf(n as f64))
        }
    }
}

pub struct PEulerSequence {
    cycle: Vec<f64>
}
impl PEulerSequence {

    fn get_bit(num: u8, i: u8) -> bool {
        (num >> i) % 2 == 1
    }

    pub fn new() -> Self {
        let fermat = [3.,5.,17.,257.,65537.];
        let mut cycle = vec![1.];

        // dodamo produkte fermata
        for i in 1..(2 as u8).pow(fermat.len() as u32) {
            let mut val: f64 = 1.;
            for j in 0..fermat.len() {
                if Self::get_bit(i, j as u8) {
                    val *= fermat[j];
                }
            }
            cycle.push(val);
        }

        Self { cycle }
    } 
}
impl FunctionSequence for PEulerSequence {
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        SequenceInfo {
            name: "p_euler".to_owned(),
            description: "(Najverjetneje) V dokaj naključnem vrstnem redu števila M za katere je phi(M) potenca praštevila. phi(n) je Eulerjeva funkcija fi.".to_owned(),
            parameters: 0,
            sequences: 0
        }
    }

    fn evaluate(&self, _parameters: &[f64], n: u64) -> Result<f64> {
        let n: i32 = n.try_into()?;
        let len: i32 = self.cycle.len().try_into()?;

        // Zadnja števka v bazi len izbere produkt fermatovih, preostali del pa eksponent števila 2
        let exp = n / len;
        let rem: usize = (n % len).try_into()?;
        
        Ok(self.cycle[rem]*(2. as f64).powi(exp))
    }
}