import { NextRequest, NextResponse } from 'next/server';
import jwt from 'jsonwebtoken';

export interface AuthenticatedRequest extends NextRequest {
  user?: {
    id: string;
    email: string;
  };
}

export interface AuthResult {
  userId: string;
  userEmail: string;
}

export async function authMiddleware(
  req: AuthenticatedRequest
): Promise<NextResponse | AuthResult> {
  try {
    const token = req.headers.get('Authorization')?.replace('Bearer ', '');

    if (!token) {
      return NextResponse.json(
        { error: 'Authentication token is required' },
        { status: 401 }
      );
    }

    const decoded = jwt.verify(token, process.env.JWT_SECRET!) as {
      id: string;
      email: string;
    };

    // Add user info to request headers
    req.headers.set('x-user-id', decoded.id);
    req.headers.set('x-user-email', decoded.email);

    // Return the user information
    return {
      userId: decoded.id,
      userEmail: decoded.email
    };
  } catch (error) {
    console.error('Auth middleware error:', error);
    return NextResponse.json(
      { error: 'Invalid authentication token' },
      { status: 401 }
    );
  }
} 