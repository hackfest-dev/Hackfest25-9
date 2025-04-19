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
use unity_vault::{Instruction as ProgramInstruction, GovernanceInstruction};
use unity_vault::governance::state::{ProposalParams, VoteType};

pub struct GovernanceClient {
    program_id: Pubkey,
    client: RpcClient,
}

impl GovernanceClient {
    pub fn new(program_id: Pubkey, rpc_url: String) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { program_id, client }
    }

    pub fn create_proposal(
        &self,
        payer: &Keypair,
        title: String,
        description: String,
        voting_duration: i64,
        min_votes: u32,
        min_approval_percentage: u8,
    ) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
        // Derive proposal PDA
        let (proposal_pda, _) = Pubkey::find_program_address(
            &[
                b"proposal",
                payer.pubkey().as_ref(),
                title.as_bytes(),
            ],
            &self.program_id,
        );

        // Calculate minimum rent-exempt balance
        let account_size = 1024; // Size of Proposal account
        let rent = self.client.get_minimum_balance_for_rent_exemption(account_size)?;

        // Create account instruction
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &proposal_pda,
            rent,
            account_size as u64,
            &self.program_id,
        );

        // Create proposal instruction
        let params = ProposalParams {
            title,
            description,
            voting_duration,
            min_votes,
            min_approval_percentage,
        };

        let create_proposal_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Governance(GovernanceInstruction::CreateProposal(params)),
            vec![
                AccountMeta::new(proposal_pda, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix, create_proposal_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[payer], self.client.get_latest_blockhash()?);
        
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok((proposal_pda, signature))
    }

    pub fn vote(
        &self,
        voter: &Keypair,
        proposal_pda: Pubkey,
        vote_type: VoteType,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        // Derive vote PDA
        let (vote_pda, _) = Pubkey::find_program_address(
            &[
                b"vote",
                proposal_pda.as_ref(),
                voter.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        let vote_ix = Instruction::new_with_borsh(
            self.program_id,
            &ProgramInstruction::Governance(GovernanceInstruction::VoteProposal(vote_type)),
            vec![
                AccountMeta::new(proposal_pda, false),
                AccountMeta::new(vote_pda, false),
                AccountMeta::new(voter.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[vote_ix],
            Some(&voter.pubkey()),
        );

        transaction.sign(&[voter], self.client.get_latest_blockhash()?);
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature)
    }

    pub fn get_proposal(&self, proposal_pda: Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let account_data = self.client.get_account_data(&proposal_pda)?;
        Ok(account_data)
    }

    pub fn get_voting_results(
        &self,
        proposal_pda: Pubkey,
    ) -> Result<Vec<(Pubkey, Vec<u8>)>, Box<dyn std::error::Error>> {
        let accounts = self.client.get_program_accounts(&self.program_id)?;
        
        let votes = accounts
            .into_iter()
            .filter(|(_, account)| {
                // Check if account is a vote for this proposal
                // This is a simplified check - you might need to adjust based on your account structure
                account.owner == self.program_id
            })
            .map(|(pubkey, account)| (pubkey, account.data))
            .collect();

        Ok(votes)
    }
}

#[tokio::main]
async fn main() {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("9btUy7Cc2JvTWjAFYaBLfDTGuWHzXmjPXbo2z7N54wdE").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = GovernanceClient::new(program_id, rpc_url);

    // Generate keypairs
    let payer = Keypair::new();

    // Example: Create a proposal
    match client.create_proposal(
        &payer,
        "Test Proposal".to_string(),
        "A test proposal".to_string(),
        86400, // 1 day voting duration
        10, // Minimum 10 votes
        60, // 60% approval required
    ) {
        Ok((proposal_pda, signature)) => {
            println!("Proposal created! PDA: {}, Signature: {}", proposal_pda, signature);
        }
        Err(err) => {
            eprintln!("Error creating proposal: {}", err);
        }
    }
} 