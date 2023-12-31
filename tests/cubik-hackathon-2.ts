import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CubikHackathon2 } from "../target/types/cubik_hackathon_2";

describe("cubik-hackathon-2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CubikHackathon2 as Program<CubikHackathon2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
