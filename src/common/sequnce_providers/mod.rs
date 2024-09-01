pub mod variants;
mod implementations;

use std::vec;
use rand::seq::SliceRandom;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    error::{Error, Result}, 
    parse::{
        parse_helper::Sendable, remote::Remote, 
        sequence_provide::{self, SequenceInfo},
        settings::SettingsPonudnik
    }
};

use variants::{function::*, operation::*};
use implementations::*;

/// Ponudnik neke vrste zaporedja
#[async_trait]
pub trait SequenceProvider : Sync {
    /// Vrne informacije o zaporedju
    fn get_info(&self) -> sequence_provide::SequenceInfo;
    /// Generira zaporedje iz delov
    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], sequences: &[Vec<f64>]) -> Result<Vec<f64>>;

    /// Najprej pridobi potrebna zaporedja in potem pokliče generate, da združi v eno
    async fn provide(&self, request: sequence_provide::Request, manager: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let mut sequences = vec![];
        for seq in &request.sequences {
            let result = manager.read().await.find(&seq.get_info())?.provide(
                sequence_provide::Request { range: request.range, parameters: seq.parameters.clone(), sequences: seq.sequences.clone() },
                &manager
            ).await?;
            sequences.push(result);
        }

        self.generate(request.range, &request.parameters, &sequences)
    }
}
/// Struktura, ki hrani vse ponudnike zaporedij
pub struct ProviderManager {
    pub local_providers:    Vec<Box<dyn SequenceProvider + Send>>,
    remote_providers:   Vec<Box<dyn SequenceProvider + Send>>,
    generator:          Remote,
    central:            Remote
}

impl ProviderManager {
    pub fn new(settings: &SettingsPonudnik, generator: &Remote, central: &Remote) -> Self {

        let mut local_providers: Vec<Box<dyn SequenceProvider + Send>> = vec![
                Box::new(constant::Provider {}),
                Box::new(drop::Provider {}),
                Box::new(power_mod::Provider {}),
                Box::new(p_euler::Provider::new()),
                Box::new(OperationSequenceProvider::new(Box::new(sum::Sequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(prod::Sequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(lin_com::Sequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(round::Sequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(max_seqs::Sequence {}))),
                Box::new(OperationSequenceProvider::new(Box::new(min_seqs::Sequence {}))),
                Box::new(FunctionSequenceProvider::new(Box::new(arithmetic::Sequence {}))),
                Box::new(FunctionSequenceProvider::new(Box::new(geometric::Sequence {}))),
        ];

        if settings.lin_recur_globina > 0 {
            for i in 1..=settings.lin_recur_globina {
                local_providers.push(Box::new(linear_recursion_h::Provider::new(i as usize)))
            }
        }

        ProviderManager { 
            local_providers,
            remote_providers: vec![],
            generator: generator.clone(),
            central: central.clone()
        }

    }

    /// Najde ponudnika, ki ponuja zaporedje z podano signaturo
    pub fn find(&self, seq: &SequenceInfo) -> Result<&Box<dyn SequenceProvider + Send>> {
        let mut close = vec![];
        
        let local = self.local_providers.iter().filter(|provider| {
            let info = &provider.get_info();
            if info.name == seq.name { close.push(info.clone()); }
            info == seq
        }).next();

        if local.is_some() { Ok(local.unwrap()) }
        else {
            let valid: Vec<&Box<dyn SequenceProvider + Send>> = self.remote_providers.iter().filter(|provider| {
                let info = &provider.get_info();
                if info.name == seq.name { close.push(info.clone()); }
                info == seq
            }).collect();
            valid.choose(&mut rand::thread_rng()).map(|provider| *provider).ok_or(
                Error::missing_provider(seq.clone(), &close)
            )
        }
    }
    
    pub fn get_info(&self) -> Vec<SequenceInfo> {
        self.local_providers.iter().map(|p| p.get_info()).collect()
    }

    /// Pridobi ponudnike vseh zaporedij, ki jih ponuja nek Remote 
    async fn get_remote_sequence_providers(remote: &Remote) -> Result<Vec<Box<dyn SequenceProvider + Send>>> {
        let mut result: Vec<Box<dyn SequenceProvider + Send>> = vec![];
        let (reason, status, data) = remote.get("/sequence/", None).await?;

        if (reason, status) == ("OK".to_owned(), 200) {
            let list: Vec<SequenceInfo> = serde_json::from_slice(&data)?;
            for info in list {
                result.push(
                    Box::new(RemoteSequenceProvider { host: remote.clone(), info: info.clone() })
                );
            }
            Ok(result)
        } else { Err(Error::remote_invalid_response(&remote.get_url(), &data)) }
    }

    /// Posodobi vse oddaljene ponudnike zaporedij
    pub async fn update_providers(register_endpoint: &str, manager: &RwLock<Self>) -> Result<()> {
        let generator = manager.read().await.generator.clone();
        let central_server = manager.read().await.central.clone();      
        let (reason, status, data) = central_server.get(register_endpoint, None).await?;

        if (reason, status) == ("OK".to_owned(), 200) {
            let list: Vec<Remote> = serde_json::from_slice(&data)?;
            let mut providers = vec![];
            for remote in list {
                if remote != generator {
                    match ProviderManager::get_remote_sequence_providers(&remote).await {
                        Err(_) => (),
                        Ok(mut extra) => providers.append(&mut extra)
                    }
                }    
            }

            let mut manager = manager.write().await;
            manager.remote_providers = providers;
            Ok(())
        } else { Err(Error::remote_invalid_response(&central_server.get_url(),&data)) }
    } 
}

// ---------- implementacije posebnega primera za oddaljene ponudnike ----------

/// Ponudnik zaporedja, ki ga ponuja nek Remote
struct RemoteSequenceProvider {
    host:   Remote,
    info:   SequenceInfo
}

#[async_trait]
impl SequenceProvider for RemoteSequenceProvider {
    fn generate(&self,_:sequence_provide::Range,_: &[f64],_: &[Vec<f64>]) -> Result<Vec<f64> > { panic!("Unreachable code!") }
    fn get_info(&self) -> sequence_provide::SequenceInfo { self.info.clone() }

    async fn provide(&self, request: sequence_provide::Request, _: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let endpoint = format!("/sequence/{}", self.info.name);
        let (reason, status, data) = self.host.post(&endpoint, &request.as_sendable()?, None).await?;

        if (reason, status) == ("OK".to_owned(), 200) {
            let list: Vec<f64> = serde_json::from_slice(&data)?;
            Ok(list)
        } else { Err(Error::remote_invalid_response(&self.host.get_url(), &data)) }
    }
}