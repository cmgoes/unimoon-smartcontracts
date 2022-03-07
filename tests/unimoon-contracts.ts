import assert from 'assert'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { MetadataProgram, DataV2, Metadata, MasterEdition } from "@metaplex-foundation/mpl-token-metadata";
import { CaptureActions } from '../target/types/capture_actions';
import { MediaObjects } from '../target/types/media_objects';

describe('unimoon-contracts', () => {
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  const program1 = anchor.workspace.CaptureActions as Program<CaptureActions>;
  const program2 = anchor.workspace.MediaObjects as Program<MediaObjects>;

  let _userProfile: anchor.web3.Keypair;
  let _post: anchor.web3.Keypair;

  it("User's profile is created!", async () => {
    const userProfile = anchor.web3.Keypair.generate()
    const tx = await program1.rpc.createProfile({
      accounts: {
        userProfile: userProfile.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [userProfile]
    });
    console.log("Your transaction signature", tx);

    const profile = await program1.account.userProfile.fetch(userProfile.publicKey);

    assert.ok(profile.sac.eq(new anchor.BN(0)));
    assert.ok(profile.authority.equals(provider.wallet.publicKey))

    _userProfile = userProfile;
  });

  // it("A post is written!", async () => {
  //   const userProfile = _userProfile;
  //   const post = anchor.web3.Keypair.generate()
  //   let mint = await createMint(provider);
  //   let token = await createTokenAccount(provider, mint, userProfile.publicKey)

  //   const tx = await program.rpc.writePost({
  //     accounts: {
  //       userProfile: userProfile.publicKey,
  //       authority: provider.wallet.publicKey,
  //       post: post.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       mint,
  //       token,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     },
  //     signers: [post]
  //   })
  //   console.log("Your transaction signature", tx);

  //   const profileAccount = await program.account.userProfile.fetch(userProfile.publicKey);
  //   const postAccount = await program.account.post.fetch(post.publicKey);
  //   const tokenAccount = await getTokenAccount(provider, token);

  //   assert.ok(profileAccount.sac.eq(new anchor.BN(10)));

  //   assert.ok(postAccount.views.eq(new anchor.BN(0)));
  //   assert.ok(postAccount.likes.eq(new anchor.BN(0)));
  //   assert.ok(postAccount.shares.eq(new anchor.BN(0)));
  //   assert.ok(postAccount.totalComments.eq(new anchor.BN(0)));
  //   assert.ok(postAccount.downloads.eq(new anchor.BN(0)));
  //   assert.ok(postAccount.creator.equals(userProfile.publicKey));
  //   assert.ok(postAccount.token.equals(token));
  //   assert.ok(postAccount.sac.eq(new anchor.BN(0)));

  //   assert.ok(tokenAccount.amount.eq(new anchor.BN(1)));
  //   assert.ok(tokenAccount.owner.equals(userProfile.publicKey));
  //   assert.ok(tokenAccount.mint.equals(mint));
  //   _post = post
  // });

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

  it('Create metadata', async () => {
    const mint = anchor.web3.Keypair.generate();
    const payer = program2.provider.wallet.publicKey;

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
    const tx = await program2.methods.createMasterEdition(data, true, null).accounts({
      authority,
      mint: mint.publicKey,
      tokenAccount,
      metadataAccount,
      editionAccount,
      metadataProgram: MetadataProgram.PUBKEY,
    }).signers([mint]).rpc();
    console.log("Your transaction signature", tx);
  })
});
