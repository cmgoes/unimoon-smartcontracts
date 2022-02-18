import assert from 'assert'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CaptureActions } from '../target/types/capture_actions';

describe('capture-actions', () => {
  const provider = anchor.Provider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.CaptureActions as Program<CaptureActions>;

  let _userProfile;

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

  it("Content is uploaded!", async () => {
    const userProfile = _userProfile;

    const tx = await program.rpc.uploadContent({
      accounts: {
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey
      }
    })
    console.log("Your transaction signature", tx);

    const profile = await program.account.userProfile.fetch(userProfile.publicKey);

    assert.ok(profile.score.eq(new anchor.BN(1)));
  });
});
