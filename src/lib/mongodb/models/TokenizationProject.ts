import mongoose from 'mongoose';
import { TokenizedAssetType, TokenizationStatus, RiskLevel } from '@/types/tokenization';

const tokenizationFeesSchema = new mongoose.Schema({
  platformFee: { type: Number, required: true },
  managementFee: { type: Number, required: true },
  performanceFee: { type: Number, required: true },
  entryFee: { type: Number, required: true },
  exitFee: { type: Number, required: true },
  otherFees: [{
    name: { type: String, required: true },
    amount: { type: Number, required: true },
    description: String
  }]
});

const tokenizationProjectSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true,
    trim: true
  },
  description: {
    type: String,
    required: true
  },
  assetType: {
    type: String,
    enum: Object.values(TokenizedAssetType),
    required: true
  },
  status: {
    type: String,
    enum: Object.values(TokenizationStatus),
    default: TokenizationStatus.DRAFT
  },
  owner: {
    type: String,
    required: true
  },
  targetRaise: {
    type: Number,
    required: true
  },
  minimumInvestment: {
    type: Number,
    required: true
  },
  tokenPrice: {
    type: Number,
    required: true
  },
  totalTokens: {
    type: Number,
    required: true
  },
  soldTokens: {
    type: Number,
    default: 0
  },
  startDate: Date,
  endDate: Date,
  legalStructure: {
    type: String,
    required: true
  },
  jurisdiction: {
    type: String,
    required: true
  },
  riskLevel: {
    type: String,
    enum: Object.values(RiskLevel),
    required: true
  },
  expectedReturn: Number,
  fees: {
    type: tokenizationFeesSchema,
    required: true
  },
  tags: [String],
  images: [String],
  createdAt: {
    type: Date,
    default: Date.now
  },
  updatedAt: {
    type: Date,
    default: Date.now
  }
});

// Update the updatedAt field before saving
tokenizationProjectSchema.pre('save', function(next) {
  this.updatedAt = new Date();
  next();
});

export const TokenizationProject = mongoose.models.TokenizationProject || mongoose.model('TokenizationProject', tokenizationProjectSchema); 