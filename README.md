# Unity Vault

A comprehensive Solana program implementing a decentralized platform with community governance, lending, tokenization, and user management capabilities.

## Program Architecture

The program consists of five core modules:

1. **User Module**: Identity and access management
2. **Community Module**: Group organization and management
3. **Governance Module**: Decentralized decision-making
4. **Lending Module**: Financial operations
5. **Tokenization Module**: Asset management

Each module follows a consistent structure:
- `state.rs`: Data structures and state management
- `context.rs`: Validation and security checks
- `instructions.rs`: Business logic implementation
- `mod.rs`: Module exports and organization

## Getting Started

### Prerequisites

- Node.js v18.18.0 or higher
- Rust v1.77.2 or higher
- Anchor CLI 0.30.1 or higher
- Solana CLI 1.18.17 or higher

### Installation

1. **Clone the repository**
```shell
git clone <repo-url>
cd unity-vault
```

2. **Install Dependencies**
```shell
pnpm install
```

3. **Build the Program**
```shell
pnpm anchor-build
```

4. **Start Local Development**
```shell
pnpm anchor-localnet
```

## Development Workflow

### Program Development

#### Sync Program ID
```shell
pnpm anchor keys sync
```
This will:
- Create a new keypair in `anchor/target/deploy`
- Update the Anchor config file
- Update the `declare_id!` macro in `src/lib.rs`
- Update the constant in `anchor/lib/basic-exports.ts`

#### Run Tests
```shell
pnpm anchor-test
```

#### Deploy to Devnet
```shell
pnpm anchor deploy --provider.cluster devnet
```

### Web App Development

The web interface is built with React and uses the Anchor-generated client to interact with the Solana program.

#### Start Development Server
```shell
pnpm dev
```

#### Build for Production
```shell
pnpm build
```

## Module Documentation

Detailed documentation for each module can be found in their respective directories:
- [User Module](./src/user/README.md)
- [Community Module](./src/community/README.md)
- [Governance Module](./src/governance/README.md)
- [Lending Module](./src/lending/README.md)
- [Tokenization Module](./src/tokenization/README.md)

## Security Considerations

The program implements several security measures:
- Role-based access control
- Two-factor authentication
- KYC verification
- Financial parameter validation
- Governance security checks

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
