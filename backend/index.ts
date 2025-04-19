import express, { type Request, type Response } from 'express';
import cors from 'cors';
import { Connection, Keypair, PublicKey, SendTransactionError } from '@solana/web3.js';
import { UserService } from './services/userService';
import type { CreateUserProfileRequest, UpdateUserProfileRequest, EnableTwoFactorRequest, VerifyKycRequest } from './types/user';
import dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';
import { monitorLogs } from './utils/logger';

dotenv.config();

const app = express();
app.use(cors());
app.use(express.json());

// Initialize Solana connection
const connection = new Connection(process.env.RPC_URL || 'http://localhost:8899');
const walletKeyData = JSON.parse(fs.readFileSync(path.join(__dirname, 'wallet.json'), 'utf-8'));
const wallet = Keypair.fromSecretKey(new Uint8Array(walletKeyData));

// Log wallet info at startup
(async () => {
  try {
    const balance = await connection.getBalance(wallet.publicKey);
    console.log('======= WALLET INFORMATION =======');
    console.log(`Public Key: ${wallet.publicKey.toString()}`);
    console.log(`Balance: ${balance / 1000000000} SOL`);
    console.log(`Network: ${process.env.RPC_URL || 'http://localhost:8899'}`);
    console.log('=================================');
  } catch (error) {
    console.error('Error getting wallet balance:', error);
  }
})();

const userService = new UserService(connection, wallet);

// User Profile Endpoints
app.post('/api/user/profile', async (req: Request, res: Response) => {
  try {
    // Log the incoming request
    console.log('Creating user profile with request:', JSON.stringify(req.body, null, 2));

    // Validate request body
    if (!req.body.full_name || !req.body.email) {
      return res.status(400).json({ 
        success: false, 
        error: 'Full name and email are required' 
      });
    }

    const balance = await connection.getBalance(wallet.publicKey);
    console.log(`Current wallet balance: ${balance / 1000000000} SOL`);

    const params: CreateUserProfileRequest = {
      full_name: req.body.full_name,
      email: req.body.email,
      role: req.body.role || 0 // Default to User role if not specified
    };

    const signature = await userService.createUserProfile(params);
    res.json({ 
      success: true, 
      signature,
      params,
      wallet: wallet.publicKey.toString()
    });
  } catch (error) {
    console.error('Error details:', {
      error,
      wallet: wallet.publicKey.toString(),
      programId: process.env.PROGRAM_ID
    });

    if (error instanceof SendTransactionError) {
      return res.status(500).json({ 
        success: false, 
        error: 'Transaction failed',
        details: {
          message: error.message,
          logs: error.logs,
          stack: error.stack
        }
      });
    }

    res.status(500).json({ 
      success: false, 
      error: error instanceof Error ? error.message : 'Unknown error'
    });
  }
});

app.put('/api/user/profile', async (req: Request, res: Response) => {
  try {
    const params: UpdateUserProfileRequest = req.body;
    const signature = await userService.updateUserProfile(params);
    res.json({ success: true, signature });
  } catch (error) {
    console.error('Error updating user profile:', error);
    res.status(500).json({ success: false, error: 'Failed to update user profile' });
  }
});

app.get('/api/user/profile', async (req: Request, res: Response) => {
  try {
    const userProfile = await userService.getUserProfile();
    res.json({ success: true, data: userProfile });
  } catch (error) {
    console.error('Error fetching user profile:', error);
    res.status(500).json({ success: false, error: 'Failed to fetch user profile' });
  }
});

app.post('/api/user/two-factor', async (req: Request, res: Response) => {
  try {
    const params: EnableTwoFactorRequest = req.body;
    const signature = await userService.enableTwoFactor(params);
    res.json({ success: true, signature });
  } catch (error) {
    console.error('Error enabling two factor:', error);
    res.status(500).json({ success: false, error: 'Failed to enable two factor' });
  }
});

app.post('/api/user/kyc', async (req: Request, res: Response) => {
  try {
    const params: VerifyKycRequest = req.body;
    const signature = await userService.verifyKyc(params);
    res.json({ success: true, signature });
  } catch (error) {
    console.error('Error verifying KYC:', error);
    res.status(500).json({ success: false, error: 'Failed to verify KYC' });
  }
});

// Enhanced wallet info endpoint
app.get('/api/wallet/info', async (req: Request, res: Response) => {
  try {
    const balance = await connection.getBalance(wallet.publicKey);
    const solanaNetwork = process.env.RPC_URL || 'http://localhost:8899';
    
    console.log(`Wallet ${wallet.publicKey.toString()} has ${balance / 1000000000} SOL on ${solanaNetwork}`);
    
    res.json({ 
      success: true, 
      data: {
        publicKey: wallet.publicKey.toString(),
        balance: balance / 1000000000, // Convert to SOL
        balanceInLamports: balance,
        network: solanaNetwork,
        hasEnoughFunds: balance >= 10000000 // At least 0.01 SOL
      } 
    });
  } catch (error) {
    console.error('Error getting wallet info:', error);
    res.status(500).json({ success: false, error: 'Failed to get wallet info' });
  }
});

// Add airdrop endpoint
app.post('/api/wallet/airdrop', async (req: Request, res: Response) => {
  try {
    const amount = req.body.amount || 1; // Default to 1 SOL
    const signature = await connection.requestAirdrop(
      wallet.publicKey,
      amount * 1000000000 // Convert to lamports
    );
    await connection.confirmTransaction(signature);
    const balance = await connection.getBalance(wallet.publicKey);
    
    res.json({
      success: true,
            signature,
            balance: balance / 1000000000 // Return updated balance in SOL
          });
        } catch (error) {
          console.error('Error requesting airdrop:', error);
          res.status(500).json({ success: false, error: 'Failed to request airdrop' });
        }
      });
      
      // Start the server
      const PORT = process.env.PORT || 3000;
      app.listen(PORT, async () => {
        console.log(`Server running on port ${PORT}`);
        
        if (process.env.VERBOSE_LOGS === 'true') {
            await monitorLogs(connection, process.env.PROGRAM_ID!);
            console.log('Program log monitoring enabled');
        }
      });