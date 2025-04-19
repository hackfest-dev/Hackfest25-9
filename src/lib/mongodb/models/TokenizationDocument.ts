import mongoose from 'mongoose';
import { TokenizationDocumentType } from '@/types/tokenization';

const tokenizationDocumentSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true,
    trim: true
  },
  description: String,
  fileUrl: {
    type: String,
    required: true
  },
  fileType: {
    type: String,
    required: true
  },
  fileSize: {
    type: Number,
    required: true
  },
  uploadedAt: {
    type: Date,
    default: Date.now
  },
  uploadedBy: {
    type: String,
    required: true
  },
  documentType: {
    type: String,
    enum: Object.values(TokenizationDocumentType),
    required: true
  },
  isVerified: {
    type: Boolean,
    default: false
  },
  verifiedAt: Date,
  verifiedBy: String,
  projectId: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'TokenizationProject',
    required: true
  }
});

export const TokenizationDocument = mongoose.models.TokenizationDocument || mongoose.model('TokenizationDocument', tokenizationDocumentSchema); 