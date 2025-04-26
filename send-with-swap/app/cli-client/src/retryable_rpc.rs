use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Signature, Signer},
    transaction::Transaction,
};
use std::{time::Duration, future::Future};

// Wrap the RPC client with retry capabilities
pub struct RetryableRpcClient {
    client: RpcClient,
    max_retries: usize,
    retry_delay_ms: u64,
}

impl RetryableRpcClient {
    pub fn new(url: &str, commitment: CommitmentConfig, max_retries: usize, retry_delay_ms: u64) -> Self {
        Self {
            client: RpcClient::new_with_commitment(url.to_string(), commitment),
            max_retries,
            retry_delay_ms,
        }
    }

    pub fn get_client(&self) -> &RpcClient {
        &self.client
    }

    // Generic retry function for any RPC operation
    async fn retry<T, F, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut last_error = None;
        for attempt in 0..self.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    println!("Attempt {} failed: {}", attempt + 1, err);
                    last_error = Some(err);
                    tokio::time::sleep(Duration::from_millis(self.retry_delay_ms)).await;
                }
            }
        }
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Retry failed for unknown reason")))
    }

    // Retry sending a transaction
    pub async fn send_and_confirm_transaction<T: Signer>(&self, transaction: &Transaction, signers: &[&T]) -> Result<Signature> {
        self.retry(|| {
            let tx = transaction.clone();
            let client = self.client.clone();
            async move {
                client.send_and_confirm_transaction(&tx).await.map_err(|e| anyhow::anyhow!("RPC error: {}", e))
            }
        }).await
    }

    // Retry getting the latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<solana_sdk::hash::Hash> {
        self.retry(|| {
            let client = self.client.clone();
            async move {
                client.get_latest_blockhash().await.map_err(|e| anyhow::anyhow!("RPC error: {}", e))
            }
        }).await
    }
}
