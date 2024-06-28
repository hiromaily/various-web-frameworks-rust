use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

pub trait Hash: Debug + Send + Sync + 'static {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String>;
}
