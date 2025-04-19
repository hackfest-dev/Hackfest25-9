# Governance Module

A Solana program module for managing decentralized governance through proposals and voting.

## Core Structures

### State (`state.rs`)
- `Proposal`: Main structure with:
  - Basic info: title, description
  - Voting parameters: duration, min_votes, approval_percentage
  - Vote counts: yes/no/abstain
  - Status tracking: Draft/Active/Passed/Rejected/Executed
  - Timestamps: created/updated/executed
- `ProposalStatus`: States (Draft/Active/Passed/Rejected/Executed)
- `VoteType`: Voting options (Yes/No/Abstain)
- `ProposalParams`: Input parameters for proposal creation

### Context (`context.rs`)
- `CreateProposalContext`: Validates new proposal creation
  - Checks authority signature
  - Verifies proposal parameters
- `VoteContext`: Manages voting process
  - Validates voter eligibility
  - Ensures voting period
- `ExecuteProposalContext`: Handles proposal execution
  - Verifies proposal status
  - Checks approval criteria

### Instructions (`instructions.rs`)
- `create_proposal`: Initializes new proposal
- `vote`: Records user votes
- `execute_proposal`: Processes approved proposals

## Flow
1. **State Management** (`state.rs`)
   - Proposal lifecycle tracking
   - Vote counting and validation
   - Status transitions

2. **Context Validation** (`context.rs`)
   - Authority verification
   - Voting period checks
   - Execution conditions

3. **Instruction Processing** (`instructions.rs`)
   - Proposal lifecycle management
   - Vote processing
   - Execution handling

## Security Features
- Authority verification
- Voting period enforcement
- Minimum vote requirements
- Approval percentage checks
- Status transition validation 