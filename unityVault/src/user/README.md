# User Module

A Solana program module for managing user profiles, roles, and KYC verification.

## Core Structures

### State (`state.rs`)
- `UserProfile`: Main structure with:
  - Personal info: full_name, email
  - Role management: Admin/Moderator/User
  - Security: 2FA settings, backup codes
  - KYC data and status
  - Timestamps and status tracking
- `UserRole`: Access levels (Admin/Moderator/User)
- `UserStatus`: Account states (Active/Suspended/Banned)
- `KycStatus`: Verification states (Pending/Verified/Rejected)
- `KycData`: Verification information
  - Document details
  - Verification timestamps

### Context (`context.rs`)
- `CreateUserContext`: Validates user creation
  - Checks authority
  - Verifies profile data
- `UpdateUserContext`: Manages profile updates
  - Validates permissions
  - Handles role changes
- `VerifyKycContext`: Processes KYC verification
  - Validates documents
  - Updates verification status

### Instructions (`instructions.rs`)
- `create_user`: Initializes user profile
- `update_user`: Modifies user data
- `verify_kyc`: Processes KYC verification
- `update_role`: Manages user permissions

## Flow
1. **State Management** (`state.rs`)
   - Profile data tracking
   - Role and status management
   - KYC verification handling

2. **Context Validation** (`context.rs`)
   - Authority verification
   - Data validation
   - Permission checks

3. **Instruction Processing** (`instructions.rs`)
   - Profile management
   - Role updates
   - KYC processing

## Security Features
- Role-based access control
- Two-factor authentication
- KYC verification
- Status management
- Permission validation 