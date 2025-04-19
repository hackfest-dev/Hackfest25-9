export const IDL = {
  version: "0.1.0",
  name: "unity_vault",
  instructions: [
    {
      name: "createUserProfile",
      accounts: [
        {
          name: "userProfile",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "params",
          type: {
            defined: "UserProfileParams",
          },
        },
      ],
    },
    {
      name: "updateUserProfile",
      accounts: [
        {
          name: "userProfile",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "params",
          type: {
            defined: "UserProfileParams",
          },
        },
      ],
    },
    {
      name: "enableTwoFactor",
      accounts: [
        {
          name: "userProfile",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "secret",
          type: "string",
        },
        {
          name: "backupCodes",
          type: {
            vec: "string",
          },
        },
      ],
    },
    {
      name: "verifyKyc",
      accounts: [
        {
          name: "userProfile",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "kycData",
          type: {
            defined: "KycData",
          },
        },
      ],
    },
  ],
  accounts: [
    {
      name: "UserProfile",
      type: {
        kind: "struct",
        fields: [
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "username",
            type: "string",
          },
          {
            name: "email",
            type: "string",
          },
          {
            name: "bio",
            type: "string",
          },
          {
            name: "profileImage",
            type: "string",
          },
          {
            name: "socialLinks",
            type: {
              vec: "string",
            },
          },
          {
            name: "twoFactorEnabled",
            type: "bool",
          },
          {
            name: "kycVerified",
            type: "bool",
          },
          {
            name: "kycData",
            type: {
              option: {
                defined: "KycData",
              },
            },
          },
          {
            name: "createdAt",
            type: "i64",
          },
          {
            name: "updatedAt",
            type: "i64",
          },
        ],
      },
    },
  ],
  types: [
    {
      name: "UserProfileParams",
      type: {
        kind: "struct",
        fields: [
          {
            name: "username",
            type: "string",
          },
          {
            name: "email",
            type: "string",
          },
          {
            name: "bio",
            type: "string",
          },
          {
            name: "profileImage",
            type: "string",
          },
          {
            name: "socialLinks",
            type: {
              vec: "string",
            },
          },
        ],
      },
    },
    {
      name: "KycData",
      type: {
        kind: "struct",
        fields: [
          {
            name: "documentType",
            type: "string",
          },
          {
            name: "documentNumber",
            type: "string",
          },
          {
            name: "documentImage",
            type: "string",
          },
          {
            name: "verificationStatus",
            type: "bool",
          },
        ],
      },
    },
  ],
}; 