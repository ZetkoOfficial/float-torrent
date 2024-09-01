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

// ---------- implementacije nekaj posebnih primerov ----------

pub struct SumSequence {}
impl OperationSequence for SumSequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "sum".to_owned(),
            description: "Vsota zaporedij. Zaporedje f(n) = a(n) + b(n). Zaporedja: [a, b]".to_owned(),
            parameters: 0,
            sequences: 2
        }
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        if sequences.len() != 2 {
            Err(Error::sequence_arithmetic_error("Številu parametrov ali zaporedij je nepravilno."))
        } else {
            Ok(sequences[0] + sequences[1])
        }
    }
}

pub struct ProductSequence {}
impl OperationSequence for ProductSequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "prod".to_owned(),
            description: "Produkt zaporedij. Zaporedje f(n) = a(n) * b(n). Zaporedja: [a, b]".to_owned(),
            parameters: 0,
            sequences: 2
        }
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        if sequences.len() != 2 {
            Err(Error::sequence_arithmetic_error("Številu parametrov ali zaporedij je nepravilno."))
        } else {
            Ok(sequences[0] * sequences[1])
        }
    }
}

pub struct LinComSequence {}
impl OperationSequence for LinComSequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "lin_com".to_owned(),
            description: "Linearna kombinacija zaporedij. Zaporedje f(n) = k1(n) * a(n) + k2(n) * b(n). Zaporedja: [a, b, k1, k2]".to_owned(),
            parameters: 0,
            sequences: 4
        }
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        if sequences.len() != 4 {
            Err(Error::sequence_arithmetic_error("Številu parametrov ali zaporedij je nepravilno."))
        } else {
            Ok(sequences[2] * sequences[0] + sequences[3] * sequences[1])
        }
    }
}

pub struct RoundSequence {}
impl OperationSequence for RoundSequence {

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