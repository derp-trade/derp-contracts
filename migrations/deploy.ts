// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";
import { DerpContracts } from "../target/types/derp_contracts";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  provider.opts.commitment = "confirmed";
  provider.opts.preflightCommitment = "confirmed";
  anchor.setProvider(provider);

  const program = anchor.workspace.derpContracts as anchor.Program<DerpContracts>;

  const derpState = anchor.web3.Keypair.generate();
  console.log("DerpState pub: ", derpState.publicKey.toBase58());
  console.log("DerpState prv: ", derpState.secretKey.toString());

  const tx = await program.methods
    .initialize(
      new anchor.web3.PublicKey("2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW"),
      new anchor.web3.PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
      new anchor.web3.PublicKey("2t8eUbYKjidMs3uSeYM9jXM9uudYZwGkSeTB4TKjmvnC")
    )
    .accounts({
      derpState: derpState.publicKey,
      admin: provider.wallet.publicKey,
    })
    .signers([derpState])
    .rpc();
  console.log("Your transaction signature", tx);
};
