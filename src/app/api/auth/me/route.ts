import { NextResponse } from 'next/server';
import { authMiddleware } from '@/middleware/auth';
import connectToDatabase from '@/lib/mongodb';
import User from '@/lib/mongodb/models/User';
import { NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    // Apply authentication middleware
    const authResult = await authMiddleware(request);
    if ('error' in authResult) {
      return NextResponse.json(
        { error: authResult.error },
        { status: 401 }
      );
    }

    const { userId } = authResult as { userId: string };

    // Connect to database
    await connectToDatabase();

    // Find user by ID
    const user = await User.findById(userId).select('-password');
    if (!user) {
      return NextResponse.json(
        { error: 'User not found' },
        { status: 404 }
      );
    }

    return NextResponse.json({
      success: true,
      data: user
    });
  } catch (error) {
    console.error('Error fetching user profile:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
} 