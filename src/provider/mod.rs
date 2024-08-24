use std::vec;

use crate::{error::error::{Error, Result}, parse::sequence_provide::{self, SequenceInfo}};
use async_trait::async_trait;
use function::{ArithmeticSequence, FunctionSequenceProvider, GeometricSequence};
use tokio::sync::RwLock;

pub mod operation;
use operation::{SumSequence, ProductSequence, OperationSequenceProvider};
pub mod function;

#[async_trait]
pub trait SequenceProvider : Sync {
    fn get_info(&self) -> sequence_provide::SequenceInfo;
    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], sequences: &[Vec<f64>]) -> Result<Vec<f64>>;

    async fn provide(&self, request: sequence_provide::Request, manager: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let mut sequences = vec![];
        for seq in &request.sequences {
            let result = manager.read().await.find(&seq.get_info()).ok_or(Error::missing_provider(&seq.get_info()))?.provide(
                sequence_provide::Request { range: request.range, parameters: seq.parameters.clone(), sequences: seq.sequences.clone() },
                &manager
            ).await?;
            sequences.push(result);
        }

        self.generate(request.range, &request.parameters, &sequences)
    }
}
pub struct ProviderManager {
    providers: Vec<Box<dyn SequenceProvider + Send>>
}

impl ProviderManager {
    pub fn new() -> Self {
        ProviderManager { 
            providers: (vec! [
                Box::new(ConstantSequenceProvider {}),
                Box::new(DropSequenceProvider {}),
                Box::new(OperationSequenceProvider::new(Box::new(SumSequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(ProductSequence {}))),
                Box::new(FunctionSequenceProvider::new(Box::new(ArithmeticSequence {}))),
                Box::new(FunctionSequenceProvider::new(Box::new(GeometricSequence {}))),
            ])
        }
    }

    pub fn find(&self, seq: &SequenceInfo) -> Option<&Box<dyn SequenceProvider + Send>> {
        self.providers.iter().filter(|provider| {
                let info = provider.get_info();
                info.parameters == seq.parameters && info.sequences == seq.sequences && info.name == seq.name
        }).next()
    }
    
    pub fn get_info(&self) -> Vec<sequence_provide::SequenceInfo> {
        self.providers.iter().map(|p| p.get_info()).collect()
    }
}

// ---------- implementacije nekaj posebnih primerov ----------

struct ConstantSequenceProvider {}
impl SequenceProvider for ConstantSequenceProvider {
    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], _sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        let mut result = vec![];
        
        let mut i = range.from; 
        while i < range.to {
            result.push(parameters[0]); i += range.step;
        }
        Ok(result) 
    }

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "const".to_owned(),
            description: "Konstantno zaporedje s členi enakimi prvemu parametru.".to_owned(),
            parameters: 1,
            sequences: 0
        }
    }
} 
struct DropSequenceProvider {}
#[async_trait]
impl SequenceProvider for DropSequenceProvider {
    
    fn generate(&self,_:sequence_provide::Range,_: &[f64],_: &[Vec<f64>]) -> Result<Vec<f64> > {
        panic!("Unreachable code")
    }

    async fn provide(&self, request: sequence_provide::Request, manager: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let drop_count = request.parameters[0].trunc();
        if drop_count < 0. { return Err(Error::sequence_arithmetic_error("Parameter mora biti pozitiven")); }
        let drop_count = drop_count as u64;

        let sequence = request.sequences[0].clone();

        let ammended = sequence_provide::Request {
            range: sequence_provide::Range { from: request.range.from+drop_count, to: request.range.to+drop_count, step: request.range.step },
            parameters: sequence.parameters.clone(),
            sequences: sequence.sequences.clone()
        };

        manager.read().await.find(&sequence.get_info())
            .ok_or(Error::missing_provider(&sequence.get_info()))?
            .provide(ammended,&manager).await
    }
    
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "drop".to_owned(),
            description: "Izpusti prvih nekaj členov zaporedja, glede na parameter".to_owned(),
            parameters: 1,
            sequences: 1
        }
    }
}