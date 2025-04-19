import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import connectDB from '@/lib/mongodb';
import { TokenizationProject } from '@/lib/mongodb/models/TokenizationProject';
import { TokenizationDocument } from '@/lib/mongodb/models/TokenizationDocument';
import { authMiddleware, AuthResult } from '@/middleware/auth';
import { TokenizedAssetType, TokenizationStatus, RiskLevel } from '@/types/tokenization';

export async function POST(request: NextRequest) {
  try {
    // Apply auth middleware
    const authResult = await authMiddleware(request);
    if (authResult instanceof NextResponse) {
      return authResult;
    }

    await connectDB();
    
    // Get user ID from cookie
    const userId = request.cookies.get('userId')?.value;
    
    if (!userId) {
      return NextResponse.json({ 
        success: false,
        error: 'User ID is required',
        details: 'Authentication failed to provide user ID'
      }, { status: 401 });
    }

    const formData = await request.formData();
    const projectDataStr = formData.get('projectData');
    
    if (!projectDataStr || typeof projectDataStr !== 'string') {
      return NextResponse.json({ 
        success: false,
        error: 'Invalid project data',
        details: 'Project data is required and must be a string'
      }, { status: 400 });
    }

    let projectData;
    try {
      projectData = JSON.parse(projectDataStr);
    } catch (error) {
      return NextResponse.json({ 
        success: false,
        error: 'Invalid project data format',
        details: 'Project data must be a valid JSON string'
      }, { status: 400 });
    }

    // Add the user ID to the project data
    projectData.owner = userId;

    // Validate required fields
    if (!projectData.assetType || !Object.values(TokenizedAssetType).includes(projectData.assetType)) {
      return NextResponse.json({ 
        success: false,
        error: 'Invalid asset type',
        details: `Asset type must be one of: ${Object.values(TokenizedAssetType).join(', ')}`
      }, { status: 400 });
    }

    // Create the project
    const project = await TokenizationProject.create(projectData);

    // Handle file uploads if any
    const files = formData.getAll('files');
    if (files.length > 0) {
      const documentPromises = files.map(async (file: any) => {
        if (!(file instanceof File)) {
          throw new Error('Invalid file object');
        }

        // Here you would typically upload the file to your storage service
        // and get back a URL. For now, we'll just create a document record
        const document = await TokenizationDocument.create({
          name: file.name,
          fileUrl: 'pending', // This should be the actual URL after file upload
          fileType: file.type,
          fileSize: file.size,
          uploadedBy: userId,
          projectId: project._id,
          documentType: 'other', // Use lowercase enum value
          isVerified: false
        });

        return document;
      });

      await Promise.all(documentPromises);
    }

    return NextResponse.json({ 
      success: true, 
      project 
    });
  } catch (error) {
    console.error('Error creating tokenization project:', error);
    return NextResponse.json({ 
      success: false,
      error: 'Failed to create tokenization project',
      details: error instanceof Error ? error.message : 'Unknown error'
    }, { status: 500 });
  }
}

export async function GET(request: NextRequest) {
  try {
    // Apply auth middleware
    const authResult = await authMiddleware(request);
    if (authResult instanceof NextResponse) {
      return authResult;
    }

    await connectDB();
    const { userId } = authResult as AuthResult;
    
    const { searchParams } = new URL(request.url);
    const status = searchParams.get('status');
    
    const query = { owner: userId };
    if (status) {
      query.status = status;
    }
    
    const projects = await TokenizationProject.find(query).sort({ createdAt: -1 });
    
    return NextResponse.json(projects);
  } catch (error) {
    console.error('Error fetching tokenization projects:', error);
    return NextResponse.json({ 
      success: false,
      error: 'Failed to fetch tokenization projects',
      details: error instanceof Error ? error.message : 'Unknown error'
    }, { status: 500 });
  }
} 