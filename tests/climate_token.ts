import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ClimateToken } from '../target/types/climate_token';

describe('climate_token', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ClimateToken as Program<ClimateToken>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
