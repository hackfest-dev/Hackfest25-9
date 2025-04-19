use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

// Mock data for testing
pub struct MockData {
    pub program_id: Pubkey,
    pub rpc_url: String,
    pub creator: Keypair,
    pub mint: Keypair,
    pub token_account: Keypair,
    pub authority: Keypair,
    pub borrower: Keypair,
    pub token_mint: Keypair,
    pub token_vault: Keypair,
    pub lending_pool: Pubkey,
    pub loan: Pubkey,
    pub proposal: Pubkey,
    pub community: Pubkey,
    pub test_accounts: Vec<Keypair>,
}

impl MockData {
    pub fn new() -> Self {
        Self {
            program_id: Pubkey::from_str("89Lei4JF8Ga19BKsk3WUw1q25bchBupzyKyMZtw43KQ3").unwrap(),
            rpc_url: String::from("http://127.0.0.1:8899"),
            creator: Keypair::new(),
            mint: Keypair::new(),
            token_account: Keypair::new(),
            authority: Keypair::new(),
            borrower: Keypair::new(),
            token_mint: Keypair::new(),
            token_vault: Keypair::new(),
            lending_pool: Pubkey::new_unique(),
            loan: Pubkey::new_unique(),
            proposal: Pubkey::new_unique(),
            community: Pubkey::new_unique(),
            test_accounts: vec![Keypair::new(), Keypair::new(), Keypair::new()],
        }
    }

    // Fund test accounts with SOL
    pub fn fund_test_accounts(&self, client: &RpcClient, funder: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        let recent_blockhash = client.get_latest_blockhash()?;
        let lamports = 1_000_000_000; // 1 SOL

        for account in &self.test_accounts {
            let transfer_ix = system_instruction::transfer(
                &funder.pubkey(),
                &account.pubkey(),
                lamports,
            );

            let mut transaction = Transaction::new_with_payer(
                &[transfer_ix],
                Some(&funder.pubkey()),
            );

            transaction.sign(&[funder], recent_blockhash);
            client.send_and_confirm_transaction(&transaction)?;
        }

        Ok(())
    }

    // Get a test account by index
    pub fn get_test_account(&self, index: usize) -> &Keypair {
        &self.test_accounts[index]
    }

    // Mock token parameters
    pub fn mock_token_params() -> (String, String, u8, u64) {
        (
            "Test Token".to_string(),
            "TEST".to_string(),
            9,  // 9 decimals
            1000000000,  // 1 billion tokens
        )
    }

    // Mock lending pool parameters
    pub fn mock_lending_pool_params() -> (u64, u64, u64) {
        (
            500,  // 5% interest rate
            1000000000,  // 1 SOL max loan
            100000000,   // 0.1 SOL min loan
        )
    }

    // Mock loan parameters
    pub fn mock_loan_params() -> (u64, i64) {
        (
            500000000,  // 0.5 SOL
            86400,      // 1 day duration
        )
    }

    // Mock proposal parameters
    pub fn mock_proposal_params() -> (String, String, i64, u32, u8) {
        (
            "Test Proposal".to_string(),
            "A test proposal".to_string(),
            86400, // 1 day voting duration
            10,    // Minimum 10 votes
            60,    // 60% approval required
        )
    }

    // Mock community parameters
    pub fn mock_community_params() -> (String, String, String, bool) {
        (
            "Test Community".to_string(),
            "A test community".to_string(),
            "Be nice".to_string(),
            false,  // Not private
        )
    }

    // Test function to demonstrate usage of mock parameters
    pub fn test_mock_parameters() {
        let _token = Self::mock_token_params();
        let _lending_pool = Self::mock_lending_pool_params();
        let _loan = Self::mock_loan_params();
        let _proposal = Self::mock_proposal_params();
        let _community = Self::mock_community_params();
    }
}

fn main() {
    MockData::test_mock_parameters();
} 