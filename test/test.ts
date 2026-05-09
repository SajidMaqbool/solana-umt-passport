import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaPassport } from "../target/types/solana_passport";

describe("solana-passport", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SolanaPassport as Program<SolanaPassport>;

  it("Is creating a passport!", async () => {
    const passportKeypair = anchor.web3.Keypair.generate();
    await program.methods
      .createPassport("Baghi Student", "F2021-123")
      .accounts({
        passport: passportKeypair.publicKey,
        user: anchor.getProvider().publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([passportKeypair])
      .rpc();
    
    console.log("Passport Minted Successfully!");
  });
});
