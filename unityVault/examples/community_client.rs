use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    system_program,
    signature::Signature,
};
use std::str::FromStr;
use borsh::{BorshSerialize, BorshDeserialize};
use unity_vault::community::state::CommunityParams;
use unity_vault::{Instruction as ProgramInstruction, CommunityInstruction};

pub struct CommunityClient {
    program_id: Pubkey,
    client: RpcClient,
}

impl CommunityClient {
    pub fn new(program_id: Pubkey, rpc_url: String) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { program_id, client }
    }

    pub fn create_community(
        &self,
        payer: &Keypair,
        params: CommunityParams,
    ) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
        // Derive community PDA
        let (community_pda, _) = Pubkey::find_program_address(
            &[
                b"community",
                payer.pubkey().as_ref(),
                params.name.as_bytes(),
            ],
            &self.program_id,
        );

        // Calculate minimum rent-exempt balance
        let account_size = 1024; // Size of Community account
        let rent = self.client.get_minimum_balance_for_rent_exemption(account_size)?;

        // Create account instruction
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &community_pda,
            rent,
            account_size as u64,
            &self.program_id,
        );

        // Create community instruction
        let create_community_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Community(CommunityInstruction::CreateCommunity(params)),
            vec![
                AccountMeta::new(community_pda, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix, create_community_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[payer], self.client.get_latest_blockhash()?);
        
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok((community_pda, signature))
    }

    pub fn update_community(
        &self,
        payer: &Keypair,
        community_pda: Pubkey,
        params: CommunityParams,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let update_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Community(CommunityInstruction::UpdateCommunity(params)),
            vec![
                AccountMeta::new(community_pda, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[update_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[payer], self.client.get_latest_blockhash()?);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn suspend_community(
        &self,
        payer: &Keypair,
        community_pda: Pubkey,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        let suspend_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Community(CommunityInstruction::SuspendCommunity),
            vec![
                AccountMeta::new(community_pda, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[suspend_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[payer], self.client.get_latest_blockhash()?);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn get_community(&self, community_pda: Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let account_data = self.client.get_account_data(&community_pda)?;
        Ok(account_data)
    }

    pub fn list_user_communities(
        &self,
        user_pubkey: Pubkey,
    ) -> Result<Vec<(Pubkey, Vec<u8>)>, Box<dyn std::error::Error>> {
        let accounts = self.client.get_program_accounts(&self.program_id)?;
        
        let user_communities = accounts
            .into_iter()
            .filter(|(_, account)| {
                // Check if account belongs to the user
                // This is a simplified check - you might need to adjust based on your account structure
                account.owner == self.program_id
            })
            .map(|(pubkey, account)| (pubkey, account.data))
            .collect();

        Ok(user_communities)
    }
}

#[tokio::main]
async fn main() {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("9btUy7Cc2JvTWjAFYaBLfDTGuWHzXmjPXbo2z7N54wdE").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = CommunityClient::new(program_id, rpc_url);

    // Generate keypairs
    let payer = Keypair::new();

    // Example: Create a community
    let params = CommunityParams {
        name: "Test Community".to_string(),
        description: "A test community".to_string(),
        rules: "Be nice".to_string(),
        is_private: false,
    };

    match client.create_community(&payer, params) {
        Ok((community_pda, signature)) => {
            println!("Community created! PDA: {}, Signature: {}", community_pda, signature);
        }
        Err(err) => {
            eprintln!("Error creating community: {}", err);
        }
    }
} 