import assert from 'assert'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CaptureActions } from '../target/types/capture_actions';
import { createMint, createTokenAccount, getTokenAccount, TOKEN_PROGRAM_ID } from '../lib/utils';

describe('capture-actions', () => {
  const provider = anchor.Provider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.CaptureActions as Program<CaptureActions>;

  let _userProfile: anchor.web3.Keypair;
  let _post: anchor.web3.Keypair;

  it("User's profile is created!", async () => {
    const userProfile = anchor.web3.Keypair.generate()
    const tx = await program.rpc.createProfile({
      accounts: {
        userProfile: userProfile.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [userProfile]
    });
    console.log("Your transaction signature", tx);

    const profile = await program.account.userProfile.fetch(userProfile.publicKey);

    assert.ok(profile.sac.eq(new anchor.BN(0)));
    assert.ok(profile.authority.equals(provider.wallet.publicKey))

    _userProfile = userProfile;
  });

  it("A post is written!", async () => {
    const userProfile = _userProfile;
    const post = anchor.web3.Keypair.generate()
    let mint = await createMint(provider);
    let token = await createTokenAccount(provider, mint, userProfile.publicKey)

    const tx = await program.rpc.writePost({
      accounts: {
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey,
        post: post.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mint,
        token,
        tokenProgram: TOKEN_PROGRAM_ID
      },
      signers: [post]
    })
    console.log("Your transaction signature", tx);

    const profileAccount = await program.account.userProfile.fetch(userProfile.publicKey);
    const postAccount = await program.account.post.fetch(post.publicKey);
    const tokenAccount = await getTokenAccount(provider, token);

    assert.ok(profileAccount.sac.eq(new anchor.BN(10)));

    assert.ok(postAccount.views.eq(new anchor.BN(0)));
    assert.ok(postAccount.likes.eq(new anchor.BN(0)));
    assert.ok(postAccount.shares.eq(new anchor.BN(0)));
    assert.ok(postAccount.totalComments.eq(new anchor.BN(0)));
    assert.ok(postAccount.downloads.eq(new anchor.BN(0)));
    assert.ok(postAccount.creator.equals(userProfile.publicKey));
    assert.ok(postAccount.token.equals(token));
    assert.ok(postAccount.sac.eq(new anchor.BN(0)));

    assert.ok(tokenAccount.amount.eq(new anchor.BN(1)));
    assert.ok(tokenAccount.owner.equals(userProfile.publicKey));
    assert.ok(tokenAccount.mint.equals(mint));
    _post = post
  });

  it("Is transferred!", async () => {
    const userProfile = _userProfile;

    const tx = await program.rpc.transferSol({
      accounts: {
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    })
    console.log("Your transaction signature", tx);
  });

  // it("Score is updated!", async () => {
  //   const userProfile = _userProfile;
  //   const post = _post;

  //   const tx = await program.rpc.doPost(new anchor.BN(0), {
  //     accounts: {
  //       userProfile: userProfile.publicKey,
  //       authority: provider.wallet.publicKey,
  //       post: post.publicKey,
  //       postCreator: provider.wallet.publicKey
  //     }
  //   })
  //   console.log("Your transaction signature", tx);
  // });
});
