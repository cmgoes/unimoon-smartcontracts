import assert from 'assert'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { MetadataProgram, DataV2, Metadata, MasterEdition } from "@metaplex-foundation/mpl-token-metadata";
import { UnimoonBase } from '../target/types/unimoon_base';
import { MediaObjects } from '../target/types/media_objects';

describe('unimoon-contracts', () => {
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  const program1 = anchor.workspace.UnimoonBase as Program<UnimoonBase>;
  const program2 = anchor.workspace.MediaObjects as Program<MediaObjects>;

  let _post: anchor.web3.Keypair;
  let _unimoonAccount;

  it("Initialize", async () => {
    const [unimoonAccount, unimoonAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("unimoon_v0"), provider.wallet.publicKey.toBuffer()],
      program1.programId
    )

    const tx = await program1.rpc.initialize(unimoonAccountBump, {
      accounts: {
        unimoon: unimoonAccount,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
    });
    console.log("Your transaction signature", tx);

    const unimoonState = await program1.account.unimoon.fetch(unimoonAccount);

    _unimoonAccount = unimoonAccount
  });

  it("Add pair", async () => {
    const unimoonAccount = _unimoonAccount;

    const user = anchor.web3.Keypair.generate()
    const tx = await program1.rpc.addPair(user.publicKey, new anchor.BN(0), {
      accounts: {
        unimoon: unimoonAccount,
      },
    });
    console.log("Your transaction signature", tx);
  });

  // it("Score is updated!", async () => {
  //   const userProfile = _userProfile;
  //   const post = _post;

  //   const tx = await program.rpc.doPost({'view': {}}, {
  //     accounts: {
  //       // userProfile: userProfile.publicKey,
  //       // authority: provider.wallet.publicKey,
  //       post: post.publicKey,
  //       postCreator: userProfile.publicKey
  //     }
  //   })
  //   console.log("Your transaction signature", tx);
  // });

  // it("Is transferred!", async () => {
  //   const userProfile = _userProfile;

  //   const tx = await program.rpc.transferSol({
  //     accounts: {
  //       userProfile: userProfile.publicKey,
  //       authority: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }
  //   })
  //   console.log("Your transaction signature", tx);
  // });

  it('Create a post', async () => {
    const mint = anchor.web3.Keypair.generate();
    const post = anchor.web3.Keypair.generate();

    const [authority] = (await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("auth"),
    ], program2.programId));

    const tokenAccount = (await anchor.web3.PublicKey.findProgramAddress([
      authority.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mint.publicKey.toBuffer()
    ], ASSOCIATED_TOKEN_PROGRAM_ID))[0];

    const data = new DataV2({
      name: "Collection",
      symbol: "NFT",
      uri: "https://uri",
      sellerFeeBasisPoints: 1000,
      creators: null,
      collection: null,
      uses: null
    });

    const metadataAccount = await Metadata.getPDA(mint.publicKey);
    const editionAccount = await MasterEdition.getPDA(mint.publicKey);

    // @ts-ignore
    const tx = await program2.methods.createPost(data, true, null).accounts({
      post: post.publicKey,
      authority,
      mint: mint.publicKey,
      tokenAccount,
      metadataAccount,
      editionAccount,
      metadataProgram: MetadataProgram.PUBKEY,
    }).signers([mint, post]).rpc();
    console.log("Your transaction signature", tx);
  })
});
