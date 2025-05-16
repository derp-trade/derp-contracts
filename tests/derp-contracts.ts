import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DerpContracts } from "../target/types/derp_contracts";
import { expect } from "chai";

describe("derp-contracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.derpContracts as Program<DerpContracts>;
  const derpState = anchor.web3.Keypair.generate();

  const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user-account"), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  before(async () => {
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
    console.log("Initialized");

    const tx2 = await program.methods.createUserAccount()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("User account created", tx);
  });

  it("Creates user account", async () => {
    const userAccount = await program.account.userAccount.fetch(userPda);
    console.log("User account data: ", userAccount);
    console.log("Balance: ", userAccount.balance.toString());

    expect(userAccount.balance.toString()).to.equal("10000000000");
  });

  it("Opens position", async () => {
    const tx = await program.methods.openPosition(
      0,
      new anchor.BN(100 * 1_000_000),
      5
    )
      .accounts({
        user: provider.wallet.publicKey,
        pythPriceAccount: new anchor.web3.PublicKey("2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW"),
        derpState: derpState.publicKey,
      })
      .rpc();
    console.log("Position opened", tx);

    const userAccount = await program.account.userAccount.fetch(userPda);
    console.log("User account data: ", userAccount);

    const base = 1_000_000;

    console.log("Balance: ", userAccount.balance.toNumber() / base);
    console.log("Position size: ", userAccount.positions[0].size.toNumber() / base);
    console.log("Entry price: ", userAccount.positions[0].entryPrice.toNumber() / base);
  });
});
