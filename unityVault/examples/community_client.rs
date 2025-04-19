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
mod mock_data;
use mock_data::MockData;

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

        // Fund community account
        let fund_community_ix = system_instruction::transfer(
            &payer.pubkey(),
            &community_pda,
            rent,
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

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        // Create and send funding transaction
        let mut fund_transaction = Transaction::new_with_payer(
            &[fund_community_ix],
            Some(&payer.pubkey()),
        );

        fund_transaction.sign(&[payer], recent_blockhash);
        self.client.send_and_confirm_transaction(&fund_transaction)?;

        // Create and send initialization transaction
        let mut init_transaction = Transaction::new_with_payer(
            &[create_community_ix],
            Some(&payer.pubkey()),
        );

        init_transaction.sign(&[payer], recent_blockhash);
        let signature = self.client.send_and_confirm_transaction(&init_transaction)?;
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
        _user_pubkey: Pubkey,
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
    // Use mock data for testing
    let mock_data = MockData::new();
    let client = CommunityClient::new(mock_data.program_id, mock_data.rpc_url);

    // Example: Create a community using mock data
    let (name, description, rules, is_private) = MockData::mock_community_params();
    let params = CommunityParams {
        name,
        description,
        rules,
        is_private,
    };

    match client.create_community(&Keypair::new(), params) {
        Ok((community_pda, signature)) => {
            println!("Community created! PDA: {}, Signature: {}", community_pda, signature);
        }
        Err(err) => {
            eprintln!("Error creating community: {}", err);
        }
    }
} 