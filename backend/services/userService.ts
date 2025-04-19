import { Connection, PublicKey, Transaction, sendAndConfirmTransaction, Keypair, SendTransactionError, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import type { UserProfile, UserAccount, CreateUserProfileRequest, UpdateUserProfileRequest, EnableTwoFactorRequest, VerifyKycRequest } from '../types/user';
import { Buffer } from 'buffer';
import * as borsh from 'borsh';

// Instruction enum
enum Instruction {
  User = 0,
  Governance = 1,
  Community = 2,
  Lending = 3,
  Tokenization = 4
}

// User instruction enum
enum UserInstruction {
  CreateUserProfile = 0,
  UpdateUserProfile = 1,
  EnableTwoFactor = 2,
  VerifyKyc = 3
}

// User profile params schema
class UserProfileParams {
  full_name: string;
  email: string;
  role: number;

  constructor(fields: { full_name: string; email: string; role: number }) {
    this.full_name = fields.full_name;
    this.email = fields.email;
    this.role = fields.role;
  }
}

const UserProfileParamsSchema = new Map([
  [UserProfileParams, {
    kind: 'struct',
    fields: [
      ['full_name', 'string'],
      ['email', 'string'],
      ['role', 'u8']
    ]
  }]
]);

export class UserService {
  private connection: Connection;
  private wallet: Keypair;
  private programId: PublicKey;

  constructor(connection: Connection, wallet: Keypair) {
    this.connection = connection;
    this.wallet = wallet;
    this.programId = new PublicKey(process.env.PROGRAM_ID!);
  }

  async createUserProfile(params: CreateUserProfileRequest): Promise<string> {
    try {
      const [userProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), this.wallet.publicKey.toBuffer()],
        this.programId
      );

      console.log('Transaction Parameters:', {
        pda: userProfilePDA.toString(),
        authority: this.wallet.publicKey.toString(),
        params: JSON.stringify(params)
      });

      // Create instruction data using Borsh
      const userProfileParams = new UserProfileParams({
        full_name: params.full_name,
        email: params.email,
        role: params.role
      });

      const userInstructionData = borsh.serialize(UserProfileParamsSchema, userProfileParams);
      const instructionData = Buffer.concat([
        Buffer.from([Instruction.User, UserInstruction.CreateUserProfile]),
        userInstructionData
      ]);

      // Create instruction
      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: userProfilePDA, isSigner: false, isWritable: true },
          { pubkey: this.wallet.publicKey, isSigner: true, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
        ],
        programId: this.programId,
        data: instructionData
      });

      // Create and send transaction
      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [this.wallet],
        {
          commitment: 'confirmed',
          preflightCommitment: 'confirmed',
          skipPreflight: false
        }
      );

      console.log('Transaction successful:', signature);
      return signature;
    } catch (error) {
      console.error('Transaction failed:', error);
      throw error;
    }
  }

  async updateUserProfile(params: UpdateUserProfileRequest): Promise<string> {
    try {
      const [userProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), this.wallet.publicKey.toBuffer()],
        this.programId
      );

      // Create instruction data
      const instructionData = Buffer.from([
        1, // Instruction index for UpdateUserProfile
        ...Buffer.from(params.full_name),
        0, // Null terminator for string
        ...Buffer.from(params.email),
        0, // Null terminator for string
        params.role // Role as u8
      ]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: userProfilePDA, isSigner: false, isWritable: true },
          { pubkey: this.wallet.publicKey, isSigner: true, isWritable: false }
        ],
        programId: this.programId,
        data: instructionData
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [this.wallet]
      );

      return signature;
    } catch (error) {
      console.error('Error updating user profile:', error);
      throw error;
    }
  }

  async enableTwoFactor(params: EnableTwoFactorRequest): Promise<string> {
    try {
      const [userProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), this.wallet.publicKey.toBuffer()],
        this.programId
      );

      // Create instruction data
      const instructionData = Buffer.from([
        2, // Instruction index for EnableTwoFactor
        ...Buffer.from(params.secret),
        0, // Null terminator for string
        ...Buffer.from(JSON.stringify(params.backupCodes)),
        0 // Null terminator for string
      ]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: userProfilePDA, isSigner: false, isWritable: true },
          { pubkey: this.wallet.publicKey, isSigner: true, isWritable: false }
        ],
        programId: this.programId,
        data: instructionData
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [this.wallet]
      );

      return signature;
    } catch (error) {
      console.error('Error enabling two factor:', error);
      throw error;
    }
  }

  async verifyKyc(params: VerifyKycRequest): Promise<string> {
    try {
      const [userProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), this.wallet.publicKey.toBuffer()],
        this.programId
      );

      // Create instruction data
      const instructionData = Buffer.from([
        3, // Instruction index for VerifyKyc
        ...Buffer.from(params.document_type),
        0, // Null terminator for string
        ...Buffer.from(params.document_number),
        0, // Null terminator for string
        ...Buffer.from(params.verified_at.toString()),
        0 // Null terminator for string
      ]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: userProfilePDA, isSigner: false, isWritable: true },
          { pubkey: this.wallet.publicKey, isSigner: true, isWritable: false }
        ],
        programId: this.programId,
        data: instructionData
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [this.wallet]
      );

      return signature;
    } catch (error) {
      console.error('Error verifying KYC:', error);
      throw error;
    }
  }

  async getUserProfile(): Promise<UserAccount | null> {
    try {
      const [userProfilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), this.wallet.publicKey.toBuffer()],
        this.programId
      );

      const accountInfo = await this.connection.getAccountInfo(userProfilePDA);
      if (!accountInfo) return null;

      // Parse account data according to your Rust struct
      const data = accountInfo.data;
      const userProfile: UserProfile = {
        is_initialized: data[0] === 1,
        authority: new PublicKey(data.slice(1, 33)).toString(),
        full_name: this.readString(data, 33),
        email: this.readString(data, 33 + this.readString(data, 33).length + 1),
        role: data[33 + this.readString(data, 33).length + 1 + this.readString(data, 33 + this.readString(data, 33).length + 1).length + 1],
        // ... parse other fields similarly
      };

      return {
        publicKey: userProfilePDA.toString(),
        account: userProfile
      };
    } catch (error) {
      console.error('Error fetching user profile:', error);
      return null;
    }
  }

  private readString(data: Buffer, offset: number): string {
    let end = offset;
    while (data[end] !== 0 && end < data.length) end++;
    return data.slice(offset, end).toString('utf8');
  }
}