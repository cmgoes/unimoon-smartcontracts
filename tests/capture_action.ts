import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CaptureAction } from '../target/types/capture_action';

describe('capture_action', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.CaptureAction as Program<CaptureAction>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
