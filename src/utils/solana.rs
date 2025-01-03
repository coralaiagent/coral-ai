use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use crate::core::types::CoralError;

pub struct SolanaUtils {
    rpc_client: RpcClient,
}

impl SolanaUtils {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new_with_commitment(
                rpc_url.to_string(),
                CommitmentConfig::confirmed(),
            ),
        }
    }

    pub async fn get_token_balance(&self, token_account: &Pubkey) -> Result<u64, CoralError> {
        self.rpc_client
            .get_token_account_balance(token_account)
            .map(|balance| balance.ui_amount_u64())
            .map_err(|e| CoralError::NetworkError(e.to_string()))
    }

    pub async fn send_transaction(
        &self,
        transaction: Transaction,
        signers: &[&Keypair],
    ) -> Result<Signature, CoralError> {
        let signature = self.rpc_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .map_err(|e| CoralError::NetworkError(e.to_string()))?;

        Ok(signature)
    }

    pub async fn get_latest_blockhash(&self) -> Result<solana_sdk::hash::Hash, CoralError> {
        self.rpc_client
            .get_latest_blockhash()
            .map_err(|e| CoralError::NetworkError(e.to_string()))
    }

    pub async fn is_token_valid(&self, token: &Pubkey) -> Result<bool, CoralError> {
        let account = self.rpc_client
            .get_account(token)
            .map_err(|e| CoralError::NetworkError(e.to_string()))?;

        // Verify if the account is a valid SPL token
        Ok(account.owner == spl_token::id())
    }
}
