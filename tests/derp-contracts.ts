import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DerpContracts } from "../target/types/derp_contracts";

describe("derp-contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.derpContracts as Program<DerpContracts>;
  const derpState = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(
      new anchor.web3.PublicKey("2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW"),
      new anchor.web3.PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
      new anchor.web3.PublicKey("2t8eUbYKjidMs3uSeYM9jXM9uudYZwGkSeTB4TKjmvnC")
    )
      .accounts({
        derpState: derpState.publicKey,
        admin: anchor.AnchorProvider.env().wallet.publicKey,
      })
      .signers([derpState])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
