import assert from 'assert'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CaptureActions } from '../target/types/capture_actions';

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

    assert.ok(profile.score.eq(new anchor.BN(0)));
    assert.ok(profile.authority.equals(provider.wallet.publicKey))

    _userProfile = userProfile;
  });

  it("A post is written!", async () => {
    const userProfile = _userProfile;
    const post = anchor.web3.Keypair.generate()
    const tx = await program.rpc.writePost({
      accounts: {
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey,
        post: post.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [post]
    })
    console.log("Your transaction signature", tx);

    const postAccount = await program.account.post.fetch(post.publicKey);

    assert.ok(postAccount.creator.equals(userProfile.publicKey));

    _post = post

  });

  // it("Score is updated!", async () => {
  //   const userProfile = _userProfile;
  //   const post = _post;

  //   const tx = await program.rpc.doPost(new anchor.BN(1), {
  //     accounts: {
  //       userProfile: userProfile.publicKey,
  //       authority: provider.wallet.publicKey,
  //       post: post.publicKey,
  //       postWriter: provider.wallet.publicKey
  //     }
  //   })
  //   console.log("Your transaction signature", tx);
  // });
});
