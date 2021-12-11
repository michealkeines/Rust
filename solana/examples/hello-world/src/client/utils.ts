import os from 'os';
import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import {Keypair} from '@solana/web3.js';
import { copyFileSync } from 'fs';

/**
 * @private
 */
async function getConfig(): Promise<any> {
  // Path to Solana CLI config file
  const CONFIG_FILE_PATH = path.resolve(
    os.homedir(),
    '.config',
    'solana',
    'cli',
    'config.yml',
  );
  const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
  return yaml.parse(configYml);
}

export async function getRpcUrl(): Promise<string> {
    try {
      const config = await getConfig();
      if (!config.json_rpc_url) throw new Error('Missing RPC URL');
      return config.json_rpc_url;
    } catch (err) {
      console.warn(
        'Failed to read RPC url from CLI config file, falling back to localhost',
      );
      return 'http://localhost:8899';
    }
  }

  export async function getPayer(): Promise<Keypair> {
    try {
      const config = await getConfig();
      if (!config.keypair_path) throw new Error('Missing keypair path');
      return await createKeypairFromFile(config.keypair_path);
    } catch (err) {
      console.warn(
        'Failed to create keypair from CLI config file, falling back to new random keypair',
      );
      return Keypair.generate();
    }
  }

  export async function createKeypairFromFile(
    filePath: string,
  ): Promise<Keypair> {
    const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    console.log(Keypair.fromSecretKey(secretKey));
    console.log(secretKey);
    return Keypair.fromSecretKey(secretKey);
  }