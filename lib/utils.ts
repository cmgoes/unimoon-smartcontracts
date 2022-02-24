import * as anchor from '@project-serum/anchor';
import * as serumCmn from '@project-serum/common';
import { Provider } from '@project-serum/anchor';
import { TokenInstructions } from '@project-serum/serum';

export const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(
  TokenInstructions.TOKEN_PROGRAM_ID.toString()
);

export async function getTokenAccount(provider: Provider, addr: anchor.web3.PublicKey) {
  return await serumCmn.getTokenAccount(provider, addr);
}

export async function getMintInfo(provider: Provider, mintAddr: anchor.web3.PublicKey) {
  return await serumCmn.getMintInfo(provider, mintAddr);
}

export async function createMint(provider: Provider, authority?: anchor.web3.PublicKey) {
  if (authority == undefined) {
    authority = provider.wallet.publicKey
  }

  const mint = anchor.web3.Keypair.generate();
  const instructions = await createMintInstructions(
    provider,
    authority,
    mint.publicKey
  )

  const tx = new anchor.web3.Transaction();
  tx.add(...instructions);

  await provider.send(tx, [mint])

  return mint.publicKey
}

async function createMintInstructions(provider: Provider, authority: any, mint: anchor.web3.PublicKey) {
  let instructions = [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeMint({
      mint,
      decimals: 0,
      mintAuthority: authority
    })
  ]
  return instructions
}

export async function createTokenAccount(provider: Provider, mint: any, owner: anchor.web3.PublicKey) {
  const vault = anchor.web3.Keypair.generate();
  const tx = new anchor.web3.Transaction();
  tx.add(
    ...(await createTokenAccountInstrs(provider, vault.publicKey, mint, owner))
  )
  await provider.send(tx, [vault])
  return vault.publicKey
}

async function createTokenAccountInstrs(
  provider: Provider,
  newAccountPubkey: anchor.web3.PublicKey,
  mint: any,
  owner: any,
  lamports?: number
) {
  if (lamports === undefined) {
    lamports = await provider.connection.getMinimumBalanceForRentExemption(165);
  }
  return [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey,
      space: 165,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeAccount({
      account: newAccountPubkey,
      mint,
      owner,
    }),
  ];
}