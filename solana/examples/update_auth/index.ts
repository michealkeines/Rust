import {
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
  } from '@solana/web3.js';
  import { programIds } from './utils/programIds';
  import { deserializeUnchecked, serialize } from 'borsh';
  //import BN from 'bn.js';
  import { findProgramAddress, StringPublicKey, toPublicKey } from './utils';
  export const METADATA_PREFIX = 'metadata';
  export const EDITION = 'edition';
  export const RESERVATION = 'reservation';
  
  export const MAX_NAME_LENGTH = 32;
  
  export const MAX_SYMBOL_LENGTH = 10;
  
  export const MAX_URI_LENGTH = 200;
  
  export const MAX_CREATOR_LIMIT = 5;
  
  export const MAX_CREATOR_LEN = 32 + 1 + 1;
  export const MAX_METADATA_LEN =
    1 +
    32 +
    32 +
    MAX_NAME_LENGTH +
    MAX_SYMBOL_LENGTH +
    MAX_URI_LENGTH +
    MAX_CREATOR_LIMIT * MAX_CREATOR_LEN +
    2 +
    1 +
    1 +
    198;
  
  export const MAX_EDITION_LEN = 1 + 32 + 8 + 200;
  
  export const EDITION_MARKER_BIT_SIZE = 248;
  
  export enum MetadataKey {
    Uninitialized = 0,
    MetadataV1 = 4,
    EditionV1 = 1,
    MasterEditionV1 = 2,
    MasterEditionV2 = 6,
    EditionMarker = 7,
  }
  
  export enum MetadataCategory {
    Audio = 'audio',
    Video = 'video',
    Image = 'image',
    VR = 'vr',
    HTML = 'html',
  }
  
  export type MetadataFile = {
    uri: string;
    type: string;
  };
  
  export type FileOrString = MetadataFile | string;
  
  export type Attribute = {
    trait_type?: string;
    display_type?: string;
    value: string | number;
  };
  
  export interface IMetadataExtension {
    name: string;
    symbol: string;
  
    creators: Creator[] | null;
    description: string;
    // preview image absolute URI
    image: string;
    animation_url?: string;
  
    attributes?: Attribute[];
  
    // stores link to item on meta
    external_url: string;
  
    seller_fee_basis_points: number;
  
    properties: {
      files?: FileOrString[];
      category: MetadataCategory;
      maxSupply?: number;
      creators?: {
        address: string;
        shares: number;
      }[];
    };
  }
  

  export class Creator {
    address: StringPublicKey;
    verified: boolean;
    share: number;
  
    constructor(args: {
      address: StringPublicKey;
      verified: boolean;
      share: number;
    }) {
      this.address = args.address;
      this.verified = args.verified;
      this.share = args.share;
    }
  }
  
  export class Data {
    name: string;
    symbol: string;
    uri: string;
    sellerFeeBasisPoints: number;
    creators: Creator[] | null;
    constructor(args: {
      name: string;
      symbol: string;
      uri: string;
      sellerFeeBasisPoints: number;
      creators: Creator[] | null;
    }) {
      this.name = args.name;
      this.symbol = args.symbol;
      this.uri = args.uri;
      this.sellerFeeBasisPoints = args.sellerFeeBasisPoints;
      this.creators = args.creators;
    }
  }
  
  export class Metadata {
    key: MetadataKey;
    updateAuthority: StringPublicKey;
    mint: StringPublicKey;
    data: Data;
    primarySaleHappened: boolean;
    isMutable: boolean;
    editionNonce: number | null;
  
    // set lazy
    masterEdition?: StringPublicKey;
    edition?: StringPublicKey;
  
    constructor(args: {
      updateAuthority: StringPublicKey;
      mint: StringPublicKey;
      data: Data;
      primarySaleHappened: boolean;
      isMutable: boolean;
      editionNonce: number | null;
    }) {
      this.key = MetadataKey.MetadataV1;
      this.updateAuthority = args.updateAuthority;
      this.mint = args.mint;
      this.data = args.data;
      this.primarySaleHappened = args.primarySaleHappened;
      this.isMutable = args.isMutable;
      this.editionNonce = args.editionNonce ?? null;
    }

  }
  
  class CreateMetadataArgs {
    instruction: number = 0;
    data: Data;
    isMutable: boolean;
  
    constructor(args: { data: Data; isMutable: boolean }) {
      this.data = args.data;
      this.isMutable = args.isMutable;
    }
  }
  class UpdateMetadataArgs {
    instruction: number = 1;
    data: Data | null;
    // Not used by this app, just required for instruction
    updateAuthority: StringPublicKey | null;
    primarySaleHappened: boolean | null;
    constructor(args: {
      data?: Data;
      updateAuthority?: string;
      primarySaleHappened: boolean | null;
    }) {
      this.data = args.data ? args.data : null;
      this.updateAuthority = args.updateAuthority ? args.updateAuthority : null;
      this.primarySaleHappened = args.primarySaleHappened;
    }
  }
  
  
  export const METADATA_SCHEMA = new Map<any, any>([
    [
      CreateMetadataArgs,
      {
        kind: 'struct',
        fields: [
          ['instruction', 'u8'],
          ['data', Data],
          ['isMutable', 'u8'], // bool
        ],
      },
    ],
    [
      UpdateMetadataArgs,
      {
        kind: 'struct',
        fields: [
          ['instruction', 'u8'],
          ['data', { kind: 'option', type: Data }],
          ['updateAuthority', { kind: 'option', type: 'pubkeyAsString' }],
          ['primarySaleHappened', { kind: 'option', type: 'u8' }],
        ],
      },
    ],

     
    [
      Data,
      {
        kind: 'struct',
        fields: [
          ['name', 'string'],
          ['symbol', 'string'],
          ['uri', 'string'],
          ['sellerFeeBasisPoints', 'u16'],
          ['creators', { kind: 'option', type: [Creator] }],
        ],
      },
    ],
    [
      Creator,
      {
        kind: 'struct',
        fields: [
          ['address', 'pubkeyAsString'],
          ['verified', 'u8'],
          ['share', 'u8'],
        ],
      },
    ],
    [
      Metadata,
      {
        kind: 'struct',
        fields: [
          ['key', 'u8'],
          ['updateAuthority', 'pubkeyAsString'],
          ['mint', 'pubkeyAsString'],
          ['data', Data],
          ['primarySaleHappened', 'u8'], // bool
          ['isMutable', 'u8'], // bool
          ['editionNonce', { kind: 'option', type: 'u8' }],
        ],
      },
    ],
  ]);
  
  // eslint-disable-next-line no-control-regex
  const METADATA_REPLACE = new RegExp('\u0000', 'g');
  
  export const decodeMetadata = (buffer: Buffer): Metadata => {
    const metadata = deserializeUnchecked(
      METADATA_SCHEMA,
      Metadata,
      buffer,
    ) as Metadata;
    metadata.data.name = metadata.data.name.replace(METADATA_REPLACE, '');
    metadata.data.uri = metadata.data.uri.replace(METADATA_REPLACE, '');
    metadata.data.symbol = metadata.data.symbol.replace(METADATA_REPLACE, '');
    return metadata;
  };
  
  
  
  export async function updateMetadata(
    data: Data | undefined,
    newUpdateAuthority: string | undefined,
    primarySaleHappened: boolean | null | undefined,
    mintKey: StringPublicKey,
    updateAuthority: StringPublicKey,
    instructions: TransactionInstruction[],
    metadataAccount?: StringPublicKey,
  ) {
    const metadataProgramId = programIds().metadata;
  
    metadataAccount =
      metadataAccount ||
      (
        await findProgramAddress(
          [
            Buffer.from('metadata'),
            toPublicKey(metadataProgramId).toBuffer(),
            toPublicKey(mintKey).toBuffer(),
          ],
          toPublicKey(metadataProgramId),
        )
      )[0];
  
    const value = new UpdateMetadataArgs({
      data,
      updateAuthority: !newUpdateAuthority ? undefined : newUpdateAuthority,
      primarySaleHappened:
        primarySaleHappened === null || primarySaleHappened === undefined
          ? null
          : primarySaleHappened,
    });
    const txnData = Buffer.from(serialize(METADATA_SCHEMA, value));
    const keys = [
      {
        pubkey: toPublicKey(metadataAccount),
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: toPublicKey(updateAuthority),
        isSigner: true,
        isWritable: false,
      },
    ];
    instructions.push(
      new TransactionInstruction({
        keys,
        programId: toPublicKey(metadataProgramId),
        data: txnData,
      }),
    );
  
    return metadataAccount;
  }
  
  export async function createMetadata(
    data: Data,
    updateAuthority: StringPublicKey,
    mintKey: StringPublicKey,
    mintAuthorityKey: StringPublicKey,
    instructions: TransactionInstruction[],
    payer: StringPublicKey,
  ) {
    const metadataProgramId = programIds().metadata;
  
    const metadataAccount = (
      await findProgramAddress(
        [
          Buffer.from('metadata'),
          toPublicKey(metadataProgramId).toBuffer(),
          toPublicKey(mintKey).toBuffer(),
        ],
        toPublicKey(metadataProgramId),
      )
    )[0];
    console.log('Data', data);
    const value = new CreateMetadataArgs({ data, isMutable: true });
    const txnData = Buffer.from(serialize(METADATA_SCHEMA, value));
  
    const keys = [
      {
        pubkey: toPublicKey(metadataAccount),
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: toPublicKey(mintKey),
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: toPublicKey(mintAuthorityKey),
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: toPublicKey(payer),
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: toPublicKey(updateAuthority),
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
    ];
    instructions.push(
      new TransactionInstruction({
        keys,
        programId: toPublicKey(metadataProgramId),
        data: txnData,
      }),
    );
  
    return metadataAccount;
  }
  
 
  
  export async function getMetadata(
    tokenMint: StringPublicKey,
  ): Promise<StringPublicKey> {
    const PROGRAM_IDS = programIds();
  
    return (
      await findProgramAddress(
        [
          Buffer.from(METADATA_PREFIX),
          toPublicKey(PROGRAM_IDS.metadata).toBuffer(),
          toPublicKey(tokenMint).toBuffer(),
        ],
        toPublicKey(PROGRAM_IDS.metadata),
      )
    )[0];
  }
  
  