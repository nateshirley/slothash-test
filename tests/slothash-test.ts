import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SlothashTest } from "../target/types/slothash_test";

describe("slothash-test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SlothashTest as Program<SlothashTest>;

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          systemProgram: anchor.web3.SystemProgram.programId,
          slotHashes: anchor.web3.SYSVAR_SLOT_HASHES_PUBKEY,
        })
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });
});
