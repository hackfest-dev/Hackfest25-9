use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
    signature::{Keypair, Signer, Signature},
    transaction::Transaction,
    system_instruction,
    system_program,
    program_pack::Pack,
};
use std::str::FromStr;
use spl_token::{
    instruction as token_instruction,
    state::{Mint, Account as TokenAccount},
};
use unity_vault::{Instruction as ProgramInstruction, TokenizationInstruction};
use unity_vault::tokenization::state::TokenParams;

pub struct TokenizationClient {
    program_id: Pubkey,
    client: RpcClient,
}

impl TokenizationClient {
    pub fn new(program_id: Pubkey, rpc_url: String) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { program_id, client }
    }

    pub fn create_token(
        &self,
        creator: &Keypair,
        mint: &Keypair,
        creator_token_account: &Keypair,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u64,
    ) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
        // Derive token info PDA
        let (token_info_pda, _) = Pubkey::find_program_address(
            &[
                b"token_info",
                creator.pubkey().as_ref(),
                mint.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        // Calculate minimum rent-exempt balance for token info account
        let token_info_size = 1024; // Size of TokenInfo account
        let token_info_rent = self.client.get_minimum_balance_for_rent_exemption(token_info_size)?;

        // Calculate minimum rent-exempt balance for mint account
        let mint_size = Mint::get_packed_len();
        let mint_rent = self.client.get_minimum_balance_for_rent_exemption(mint_size)?;

        // Calculate minimum rent-exempt balance for token account
        let token_account_size = TokenAccount::get_packed_len();
        let token_account_rent = self.client.get_minimum_balance_for_rent_exemption(token_account_size)?;

        // Create token info account instruction
        let create_token_info_ix = system_instruction::create_account(
            &creator.pubkey(),
            &token_info_pda,
            token_info_rent,
            token_info_size as u64,
            &self.program_id,
        );

        // Create mint account instruction
        let create_mint_ix = system_instruction::create_account(
            &creator.pubkey(),
            &mint.pubkey(),
            mint_rent,
            mint_size as u64,
            &spl_token::id(),
        );

        // Create token account instruction
        let create_token_account_ix = system_instruction::create_account(
            &creator.pubkey(),
            &creator_token_account.pubkey(),
            token_account_rent,
            token_account_size as u64,
            &spl_token::id(),
        );

        // Create token instruction
        let params = TokenParams {
            name,
            symbol,
            decimals,
            total_supply,
        };

        let create_token_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Tokenization(TokenizationInstruction::CreateToken(params)),
            vec![
                AccountMeta::new(token_info_pda, false),
                AccountMeta::new(mint.pubkey(), false),
                AccountMeta::new(creator_token_account.pubkey(), false),
                AccountMeta::new(creator.pubkey(), true),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
            ],
        );

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        // Create and send transaction for account creation
        let mut setup_transaction = Transaction::new_with_payer(
            &[
                create_token_info_ix,
                create_mint_ix,
                create_token_account_ix,
            ],
            Some(&creator.pubkey()),
        );

        // Sign setup transaction
        setup_transaction.sign(&[creator, mint, creator_token_account], recent_blockhash);
        self.client.send_and_confirm_transaction(&setup_transaction)?;

        // Create and send transaction for token creation
        let mut token_transaction = Transaction::new_with_payer(
            &[create_token_ix],
            Some(&creator.pubkey()),
        );

        // Sign token transaction
        token_transaction.sign(&[creator], recent_blockhash);
        let signature = self.client.send_and_confirm_transaction(&token_transaction)?;

        Ok((token_info_pda, signature))
    }

    pub fn transfer_tokens(
        &self,
        from: &Keypair,
        to: Pubkey,
        from_token_account: Pubkey,
        to_token_account: Pubkey,
        amount: u64,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let transfer_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Tokenization(TokenizationInstruction::TransferTokens(amount)),
            vec![
                AccountMeta::new(from_token_account, false),
                AccountMeta::new(to_token_account, false),
                AccountMeta::new(from.pubkey(), true),
                AccountMeta::new(to, false),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[transfer_ix],
            Some(&from.pubkey()),
        );

        transaction.sign(&[from], self.client.get_latest_blockhash()?);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn burn_tokens(
        &self,
        owner: &Keypair,
        token_account: Pubkey,
        amount: u64,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let burn_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Tokenization(TokenizationInstruction::BurnTokens(amount)),
            vec![
                AccountMeta::new(token_account, false),
                AccountMeta::new(owner.pubkey(), true),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[burn_ix],
            Some(&owner.pubkey()),
        );

        transaction.sign(&[owner], self.client.get_latest_blockhash()?);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn get_token_info(&self, token_info_pda: Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let account_data = self.client.get_account_data(&token_info_pda)?;
        Ok(account_data)
    }
}

#[tokio::main]
async fn main() {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("89Lei4JF8Ga19BKsk3WUw1q25bchBupzyKyMZtw43KQ3").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = TokenizationClient::new(program_id, rpc_url);

    // Generate keypairs
    let creator = Keypair::new();
    let mint = Keypair::new();
    let creator_token_account = Keypair::new();

    // Example: Create a token
    match client.create_token(
        &creator,
        &mint,
        &creator_token_account,
        "Test Token".to_string(),
        "TEST".to_string(),
        9,  // 9 decimals
        1000000000,  // 1 billion tokens
    ) {
        Ok((token_info_pda, signature)) => {
            println!("Token created! PDA: {}, Signature: {}", token_info_pda, signature);
        }
        Err(err) => {
            eprintln!("Error creating token: {}", err);
        }
    }
} 