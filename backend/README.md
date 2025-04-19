# Solana User Profile Backend Service

A backend service for managing user profiles on Solana blockchain.

## Prerequisites

- Node.js 16+ and npm
- Solana CLI tools
- Local Solana validator (for development)

## Setup

1. Install dependencies:
```bash
npm install
```

2. Configure environment:
```bash
cp .env.example .env
```

Update `.env` with your settings:
- `RPC_URL`: Solana RPC URL (default: http://localhost:8899)
- `PROGRAM_ID`: Your deployed Solana program ID
- `PORT`: Server port (default: 3000)
- `VERBOSE_LOGS`: Enable detailed program logs (true/false)

3. Set up wallet:
- Place your Solana wallet keypair in `wallet.json`
- Ensure wallet has SOL for transactions

## Running the Service

1. Start local Solana validator (if using locally):
```bash
solana-test-validator
```

2. Start the server:
```bash
npm start
```

## API Endpoints

### User Profile Management

- **Create Profile**
  ```
  POST /api/user/profile
  {
    "username": "string",
    "email": "string",
    "bio": "string",
    "profileImage": "string",
    "socialLinks": ["string"]
  }
  ```

- **Update Profile**
  ```
  PUT /api/user/profile
  {
    "username": "string",
    "email": "string",
    "bio": "string",
    "profileImage": "string",
    "socialLinks": ["string"]
  }
  ```

- **Get Profile**
  ```
  GET /api/user/profile
  ```

### Security Features

- **Enable 2FA**
  ```
  POST /api/user/two-factor
  {
    "secret": "string",
    "backupCodes": ["string"]
  }
  ```

- **Verify KYC**
  ```
  POST /api/user/kyc
  {
    "documentType": "string",
    "documentNumber": "string",
    "documentImage": "string"
  }
  ```

### Wallet Management

- **Get Wallet Info**
  ```
  GET /api/wallet/info
  ```

- **Request Airdrop** (devnet/localhost only)
  ```
  POST /api/wallet/airdrop
  {
    "amount": number  // Optional, defaults to 1 SOL
  }
  ```

## Development

- Enable verbose logging by setting `VERBOSE_LOGS=true` in `.env`
- Monitor program logs: `solana logs -u localhost`
- Check wallet balance: `solana balance -u localhost`

## Error Handling

The service provides detailed error responses:
```json
{
  "success": false,
  "error": "Error message",
  "details": {
    "message": "Detailed error message",
    "logs": ["Program logs..."]
  }
}
```

## Security Notes

- Never commit `wallet.json` to version control
- Use environment variables for sensitive configuration
- Always validate input data
- Monitor transaction logs for any issues
