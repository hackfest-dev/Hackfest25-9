# Lending Module

A Solana program module for managing decentralized lending pools and loans.

## Core Structures

### State (`state.rs`)
- `LendingPool`: Main structure with:
  - Pool parameters: interest_rate, min/max loan amounts
  - Token management: mint, vault
  - Financial tracking: total_borrowed, total_deposited
  - Timestamps: created/updated
- `Loan`: Individual loan structure with:
  - Loan details: amount, interest_rate
  - Time tracking: start_time, due_time
  - Status: Active/Repaid/Defaulted
  - Borrower and pool references
- `LoanStatus`: Loan states (Active/Repaid/Defaulted)
- `LendingPoolParams`: Pool configuration parameters
- `LoanParams`: Loan creation parameters

### Context (`context.rs`)
- `CreateLendingPoolContext`: Validates new pool creation
  - Checks authority signature
  - Verifies token accounts
- `CreateLoanContext`: Manages loan creation
  - Validates borrower eligibility
  - Checks pool parameters
- `RepayLoanContext`: Handles loan repayment
  - Verifies loan status
  - Processes payments

### Instructions (`instructions.rs`)
- `create_lending_pool`: Initializes new lending pool
- `create_loan`: Processes new loan requests
- `repay_loan`: Handles loan repayments

## Flow
1. **State Management** (`state.rs`)
   - Pool and loan tracking
   - Financial calculations
   - Status transitions

2. **Context Validation** (`context.rs`)
   - Authority verification
   - Financial parameter checks
   - Status validation

3. **Instruction Processing** (`instructions.rs`)
   - Pool management
   - Loan processing
   - Repayment handling

## Security Features
- Authority verification
- Financial parameter validation
- Loan status enforcement
- Token account verification
- Interest rate calculations 