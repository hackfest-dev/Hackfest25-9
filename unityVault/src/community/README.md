# Community Module

A Solana program module for managing communities with role-based access control.

## Core Structures

### State (`state.rs`)
- `Community`: Main structure with:
  - Basic info: name, description, rules
  - Metadata: authority, member_count, timestamps
  - Status: Active/Suspended/Archived
  - Privacy: is_private flag
- `CommunityRole`: User roles (Admin/Moderator/Member)
- `CommunityStatus`: Community states (Active/Suspended/Archived)
- `CommunityParams`: Input parameters for creation/updates

### Context (`context.rs`)
- `CreateCommunityContext`: Validates new community creation
  - Checks authority signature
  - Verifies system program
  - Ensures account initialization
- `UpdateCommunityContext`: Manages community updates
  - Validates authority ownership
  - Ensures community exists
- `SuspendCommunityContext`: Handles community suspension
  - Verifies authority permissions
  - Manages status transitions

### Instructions (`instructions.rs`)
- `create_community`: Initializes new community
- `update_community`: Modifies existing community
- `suspend_community`: Changes community status

## Flow
1. **State Management** (`state.rs`)
   - Data structure definitions
   - Serialization/deserialization
   - State transition logic

2. **Context Validation** (`context.rs`)
   - Permission checks
   - Account validation
   - Security enforcement

3. **Instruction Processing** (`instructions.rs`)
   - Business logic implementation
   - Operation processing
   - Rule enforcement

## Security Features
- Role-based access control
- Authority signature verification
- State transition validation
- Account ownership checks

## Key Operations
- Create → Validate → Initialize community
- Update → Validate → Modify community
- Suspend → Validate → Change status

## Features
- Community creation and initialization
- Community updates and modifications
- Community suspension and status management
- Role-based access control
- Member count tracking
- Timestamp tracking for creation and updates

## Usage
This module is part of the Unity Vault program and is used to manage community-related operations on the Solana blockchain. It provides the foundational structures and logic needed for community management within the larger application. 