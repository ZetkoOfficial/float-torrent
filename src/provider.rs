use crate::{error::error::{Error, Result}, parse::sequence_provide::{self, SequenceInfo}};
use async_trait::async_trait;

#[async_trait]
pub trait SequenceProvider : Sync {
    fn get_info(&self) -> sequence_provide::SequenceInfo;
    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], sequences: &[Vec<f64>]) -> Result<Vec<f64>>;

    async fn provide(&self, request: sequence_provide::Request, manager: &ProviderManager) -> Result<Vec<f64>> {
        let mut sequences = vec![];
        for seq in &request.sequences {
            let result = manager.find(&seq.get_info()).ok_or(Error::missing_provider(&seq.get_info()))?.provide(
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

