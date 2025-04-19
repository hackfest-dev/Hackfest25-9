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

        // Create account instruction
        let create_account_ix = system_instruction::create_account(
            &authority.pubkey(),
            &lending_pool_pda,
            rent,
            account_size as u64,
            &self.program_id,
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

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix, init_pool_ix],
            Some(&authority.pubkey()),
        );

        transaction.sign(&[authority], self.client.get_latest_blockhash()?);
        
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
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

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix, create_loan_ix],
            Some(&borrower.pubkey()),
        );

        transaction.sign(&[borrower], self.client.get_latest_blockhash()?);
        
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

        let mut transaction = Transaction::new_with_payer(
            &[repay_ix],
            Some(&borrower.pubkey()),
        );

        transaction.sign(&[borrower], self.client.get_latest_blockhash()?);
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
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("9btUy7Cc2JvTWjAFYaBLfDTGuWHzXmjPXbo2z7N54wdE").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = LendingClient::new(program_id, rpc_url);

    // Generate keypairs
    let authority = Keypair::new();
    let borrower = Keypair::new();
    let token_mint = Keypair::new();
    let token_vault = Keypair::new();

    // Example: Initialize a lending pool
    match client.init_lending_pool(
        &authority,
        token_mint.pubkey(),
        token_vault.pubkey(),
        500,  // 5% interest rate
        1000000000,  // 1 SOL max loan
        100000000,   // 0.1 SOL min loan
    ) {
        Ok((lending_pool_pda, signature)) => {
            println!("Lending pool initialized! PDA: {}, Signature: {}", lending_pool_pda, signature);
        }
        Err(err) => {
            eprintln!("Error initializing lending pool: {}", err);
        }
    }
} 