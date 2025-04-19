# Unity Vault Example Clients

This directory contains example client implementations for interacting with the Unity Vault program. Each client demonstrates how to use different modules of the program.

## Available Clients

### 1. Governance Client (`governance_client.rs`)

A client for interacting with the governance module, demonstrating:
- Creating proposals
- Voting on proposals
- Retrieving proposal data
- Getting voting results

Example usage:
```rust
let client = GovernanceClient::new(program_id, rpc_url);
let payer = Keypair::new();

// Create a proposal
let (proposal_pda, signature) = client.create_proposal(
    &payer,
    "Test Proposal".to_string(),
    "A test proposal".to_string(),
    86400, // 1 day voting duration
    10,    // Minimum 10 votes
    60,    // 60% approval required
)?;

// Vote on a proposal
let vote_signature = client.vote(
    &voter,
    proposal_pda,
    VoteType::Approve,
)?;
```

### 2. Community Client (`community_client.rs`)

A client for interacting with the community module, demonstrating:
- Creating communities
- Updating community details
- Suspending communities
- Listing user communities

Example usage:
```rust
let client = CommunityClient::new(program_id, rpc_url);
let payer = Keypair::new();

// Create a community
let params = CommunityParams {
    name: "Test Community".to_string(),
    description: "A test community".to_string(),
    rules: "Be nice".to_string(),
    is_private: false,
};

let (community_pda, signature) = client.create_community(&payer, params)?;
```

### 3. Lending Client (`lending_client.rs`)

A client for interacting with the lending module, demonstrating:
- Initializing lending pools
- Creating loans
- Repaying loans
- Getting loan information

Example usage:
```rust
let client = LendingClient::new(program_id, rpc_url);
let authority = Keypair::new();
let borrower = Keypair::new();
let token_mint = Keypair::new();
let token_vault = Keypair::new();

// Initialize a lending pool
let (lending_pool_pda, signature) = client.init_lending_pool(
    &authority,
    token_mint.pubkey(),
    token_vault.pubkey(),
    500,  // 5% interest rate
    1000000000,  // 1 SOL max loan
    100000000,   // 0.1 SOL min loan
)?;

// Create a loan
let (loan_pda, signature) = client.create_loan(
    &borrower,
    lending_pool_pda,
    1000000000, // 1 SOL
    86400,      // 1 day duration
)?;
```

### 4. Tokenization Client (`tokenization_client.rs`)

A client for interacting with the tokenization module, demonstrating:
- Creating tokens
- Transferring tokens
- Burning tokens
- Getting token information

Example usage:
```rust
let client = TokenizationClient::new(program_id, rpc_url);
let creator = Keypair::new();
let mint = Keypair::new();
let creator_token_account = Keypair::new();

// Create a token
let (token_info_pda, signature) = client.create_token(
    &creator,
    mint.pubkey(),
    creator_token_account.pubkey(),
    "Test Token".to_string(),
    "TEST".to_string(),
    9,  // 9 decimals
    1000000000,  // 1 billion tokens
)?;

// Transfer tokens
let transfer_signature = client.transfer_tokens(
    &creator,
    recipient.pubkey(),
    creator_token_account.pubkey(),
    recipient_token_account.pubkey(),
    1000000, // 1 million tokens
)?;
```

### 5. User Client (`client.rs`)

A client for interacting with the user module, demonstrating:
- Creating user profiles
- Updating user profiles
- Enabling two-factor authentication
- Verifying KYC

Example usage:
```rust
let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
let payer = Keypair::new();

// Create user profile
let create_profile_params = UserProfileParams {
    full_name: "Test User".to_string(),
    email: "test@example.com".to_string(),
    role: UserRole::User,
};

// Create account and profile
let create_account_ix = system_instruction::create_account(...);
let create_profile_ix = Instruction::new_with_borsh(...);
```

## Running the Examples

1. Make sure you have the Unity Vault program deployed to your target network (localnet/devnet/mainnet)
2. Update the program ID in each example to match your deployed program
3. Update the RPC URL to point to your target network
4. Run the examples using:
```bash
cargo run --example governance_client
cargo run --example community_client
cargo run --example lending_client
cargo run --example tokenization_client
cargo run --example client
```

## Dependencies

The example clients require the following dependencies:
- `solana-client`
- `solana-sdk`
- `borsh`
- `unity-vault` (this program)

## Notes

- These are example implementations and should be adapted for production use
- Error handling is simplified for demonstration purposes
- Account sizes and rent calculations are approximate and should be verified
- Security best practices (like proper key management) are not fully implemented in these examples
- Token operations require proper token program integration
- Lending operations require proper token mint and vault setup
- Governance operations require proper voting power setup
- Community operations require proper role-based access control 