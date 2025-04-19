# Unity Vault Client Examples

This directory contains example implementations for interacting with the Unity Vault program from various client environments.

## Available Examples

### 1. User Management
- User profile creation and updates
- KYC verification process
- Two-factor authentication setup
- Role management

### 2. Community Operations
- Community creation and configuration
- Member management
- Community settings updates
- Status management

### 3. Governance Functions
- Proposal creation and submission
- Voting process
- Proposal status tracking
- Execution of approved proposals

### 4. Lending Operations
- Lending pool initialization
- Loan creation and management
- Repayment processing
- Interest calculations

### 5. Token Management
- Token creation and configuration
- Token transfers
- Supply management
- Status control

## Usage Examples

### User Profile Creation
```typescript
const userProfile = await program.methods
  .createUserProfile({
    fullName: "John Doe",
    email: "john@example.com",
    role: "User"
  })
  .accounts({
    userProfile: userProfilePDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId
  })
  .rpc();
```

### Community Creation
```typescript
const community = await program.methods
  .createCommunity({
    name: "My Community",
    description: "A test community",
    rules: "Be nice",
    isPrivate: false
  })
  .accounts({
    community: communityPDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId
  })
  .rpc();
```

### Proposal Creation
```typescript
const proposal = await program.methods
  .createProposal({
    title: "New Feature Proposal",
    description: "Add new feature X",
    votingDuration: 7 * 24 * 60 * 60, // 7 days
    minVotes: 100
  })
  .accounts({
    proposal: proposalPDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId
  })
  .rpc();
```

## Setup Instructions

1. **Install Dependencies**
```bash
npm install
# or
yarn install
```

2. **Configure Environment**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Build the Examples**
```bash
npm run build
# or
yarn build
```

## Testing

Run the example tests:
```bash
npm test
# or
yarn test
```

## Best Practices

1. **Error Handling**
   - Always wrap program calls in try-catch blocks
   - Implement proper error messages
   - Handle network errors gracefully

2. **Transaction Management**
   - Use proper confirmation strategies
   - Implement retry logic for failed transactions
   - Handle transaction timeouts

3. **Security**
   - Never expose private keys
   - Use proper wallet management
   - Implement proper authentication

4. **Performance**
   - Batch transactions when possible
   - Use proper caching strategies
   - Implement loading states

## Contributing

1. Fork the repository
2. Create your feature branch
3. Add your example implementation
4. Include proper documentation
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details. 