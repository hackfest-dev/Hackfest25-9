import { PublicKey } from '@solana/web3.js';

export interface UserProfileParams {
  username: string;
  email: string;
  bio: string;
  profileImage: string;
  socialLinks: string[];
}

export interface KycData {
  documentType: string;
  documentNumber: string;
  documentImage: string;
  verificationStatus: boolean;
}

export interface UserProfile {
  is_initialized: boolean;
  authority: string;
  full_name: string;
  email: string;
  role: number;
  status: number;
  two_factor_enabled: boolean;
  two_factor_secret: string;
  two_factor_backup_codes: string[];
  kyc_verified: boolean;
  kyc_status: number;
  kyc_data: {
    document_type: string;
    document_number: string;
    verified_at: number;
  };
  accredited_status: boolean;
  created_at: number;
  updated_at: number;
}

export interface TwoFactorSetup {
  secret: string;
  backupCodes: string[];
}

export interface UserAccount {
  publicKey: string;
  account: UserProfile;
}

export interface CreateUserProfileRequest {
  full_name: string;
  email: string;
  role: number; // 0 for User, 1 for Moderator, etc.
}

export interface UpdateUserProfileRequest {
  full_name: string;
  email: string;
  role: number;
}

export interface EnableTwoFactorRequest {
  secret: string;
  backupCodes: string[];
}

export interface VerifyKycRequest {
  document_type: string;
  document_number: string;
  verified_at: number;
} 