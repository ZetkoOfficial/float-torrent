use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    error::{Error, Result}, 
    parse::sequence_provide::{self, SequenceInfo}, 
    sequnce_providers::{ProviderManager, SequenceProvider}
};

pub struct Provider {}
#[async_trait]
impl SequenceProvider for Provider {

    fn get_info(&self) -> SequenceInfo {
        SequenceInfo {
            name: "drop".to_owned(),
            description: "Zaporedje enako s(n+a). Zaporedja: [s], Parametri: [a]".to_owned(),
            parameters: 1,
            sequences: 1
        }
    }
    
    fn generate(&self,_:sequence_provide::Range,_: &[f64],_: &[Vec<f64>]) -> Result<Vec<f64> > {
        panic!("Unreachable code!")
    }

    // modificiramo Range v requestu in prepošljemo naprej
    async fn provide(&self, request: sequence_provide::Request, manager: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let drop_count = request.parameters[0].trunc();
        if drop_count < 0. { return Err(Error::sequence_arithmetic_error(self.get_info(), "Parameter mora biti pozitiven")); }
        let drop_count = drop_count as u64;

        let sequence = request.sequences[0].clone();

        let ammended = sequence_provide::Request {
            range: sequence_provide::Range { from: request.range.from+drop_count, to: request.range.to+drop_count, step: request.range.step },
            parameters: sequence.parameters.clone(),
            sequences: sequence.sequences.clone()
        };

        manager.read().await.find(&sequence.get_info())?.provide(ammended,&manager).await
    }
}