use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
    signature::{Keypair, Signer, Signature},
    transaction::Transaction,
    system_instruction,
    system_program,
};
use std::str::FromStr;
use borsh::{BorshSerialize, BorshDeserialize};
use unity_vault::{Instruction as ProgramInstruction, LendingInstruction};
use unity_vault::lending::state::{LoanParams, LendingPoolParams};
mod mock_data;
use mock_data::MockData;

pub struct LendingClient {
    program_id: Pubkey,
    client: RpcClient,
}

impl LendingClient {
    pub fn new(program_id: Pubkey, rpc_url: String) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { program_id, client }
    }

    pub fn init_lending_pool(
        &self,
        authority: &Keypair,
        token_mint: Pubkey,
        token_vault: Pubkey,
        interest_rate: u64,
        max_loan_amount: u64,
        min_loan_amount: u64,
    ) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
        // Derive lending pool PDA
        let (lending_pool_pda, _) = Pubkey::find_program_address(
            &[
                b"lending_pool",
                authority.pubkey().as_ref(),
                token_mint.as_ref(),
            ],
            &self.program_id,
        );

        // Calculate minimum rent-exempt balance
        let account_size = 1024; // Size of LendingPool account
        let rent = self.client.get_minimum_balance_for_rent_exemption(account_size)?;

        // Fund lending pool account
        let fund_pool_ix = system_instruction::transfer(
            &authority.pubkey(),
            &lending_pool_pda,
            rent,
        );

        // Initialize lending pool instruction
        let params = LendingPoolParams {
            interest_rate,
            max_loan_amount,
            min_loan_amount,
        };

        let init_pool_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Lending(LendingInstruction::InitLendingPool(params)),
            vec![
                AccountMeta::new(lending_pool_pda, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new(token_mint, false),
                AccountMeta::new(token_vault, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        // Create and send funding transaction
        let mut fund_transaction = Transaction::new_with_payer(
            &[fund_pool_ix],
            Some(&authority.pubkey()),
        );

        fund_transaction.sign(&[authority], recent_blockhash);
        self.client.send_and_confirm_transaction(&fund_transaction)?;

        // Create and send initialization transaction
        let mut init_transaction = Transaction::new_with_payer(
            &[init_pool_ix],
            Some(&authority.pubkey()),
        );

        init_transaction.sign(&[authority], recent_blockhash);
        let signature = self.client.send_and_confirm_transaction(&init_transaction)?;
        Ok((lending_pool_pda, signature))
    }

    pub fn create_loan(
        &self,
        borrower: &Keypair,
        lending_pool: Pubkey,
        amount: u64,
        duration: i64,
    ) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
        // Derive loan PDA
        let (loan_pda, _) = Pubkey::find_program_address(
            &[
                b"loan",
                lending_pool.as_ref(),
                borrower.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        // Calculate minimum rent-exempt balance
        let account_size = 1024; // Size of Loan account
        let rent = self.client.get_minimum_balance_for_rent_exemption(account_size)?;

        // Create account instruction
        let create_account_ix = system_instruction::create_account(
            &borrower.pubkey(),
            &loan_pda,
            rent,
            account_size as u64,
            &self.program_id,
        );

        // Create loan instruction
        let params = LoanParams {
            amount,
            duration,
        };

        let create_loan_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Lending(LendingInstruction::CreateLoan(params)),
            vec![
                AccountMeta::new(loan_pda, false),
                AccountMeta::new(lending_pool, false),
                AccountMeta::new(borrower.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix, create_loan_ix],
            Some(&borrower.pubkey()),
        );

        transaction.sign(&[borrower], recent_blockhash);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok((loan_pda, signature))
    }

    pub fn repay_loan(
        &self,
        borrower: &Keypair,
        loan_pda: Pubkey,
        lending_pool: Pubkey,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let repay_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Lending(LendingInstruction::RepayLoan),
            vec![
                AccountMeta::new(loan_pda, false),
                AccountMeta::new(lending_pool, false),
                AccountMeta::new(borrower.pubkey(), true),
            ],
        );

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        let mut transaction = Transaction::new_with_payer(
            &[repay_ix],
            Some(&borrower.pubkey()),
        );

        transaction.sign(&[borrower], recent_blockhash);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn get_loan(&self, loan_pda: Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let account_data = self.client.get_account_data(&loan_pda)?;
        Ok(account_data)
    }
}

#[tokio::main]
async fn main() {
    // Use mock data for testing
    let mock_data = MockData::new();
    let client = LendingClient::new(mock_data.program_id, mock_data.rpc_url.clone());

    // Fund test accounts
    if let Err(err) = mock_data.fund_test_accounts(&client.client, &mock_data.authority) {
        eprintln!("Error funding test accounts: {}", err);
        return;
    }

    // Example: Initialize a lending pool using mock data
    let (interest_rate, max_loan_amount, min_loan_amount) = MockData::mock_lending_pool_params();
    
    match client.init_lending_pool(
        &mock_data.authority,
        mock_data.token_mint.pubkey(),
        mock_data.token_vault.pubkey(),
        interest_rate,
        max_loan_amount,
        min_loan_amount,
    ) {
        Ok((lending_pool_pda, signature)) => {
            println!("Lending pool initialized! PDA: {}, Signature: {}", lending_pool_pda, signature);
            
            // Example: Create a loan using a test account
            let borrower = mock_data.get_test_account(0);
            let (amount, duration) = MockData::mock_loan_params();
            
            match client.create_loan(
                borrower,
                lending_pool_pda,
                amount,
                duration,
            ) {
                Ok((loan_pda, signature)) => {
                    println!("Loan created! PDA: {}, Signature: {}", loan_pda, signature);
                }
                Err(err) => {
                    eprintln!("Error creating loan: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error initializing lending pool: {}", err);
        }
    }
} 