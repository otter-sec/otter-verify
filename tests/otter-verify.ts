import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OtterVerify } from "../target/types/otter_verify";
import { assert } from "chai";

describe("otter-verify", async () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OtterVerify as Program<OtterVerify>;
  const user = anchor.web3.Keypair.generate();

  let other_program_id = new anchor.web3.PublicKey(
    "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY"
  );

  const [otter_verify_pda, _] =
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("otter_verify"),
        user.publicKey.toBuffer(),
        other_program_id.toBuffer(),
      ],
      program.programId
    );

  it("Is initialized!", async () => {
    // Airdrop some SOL to the user.
    let airdrop_tx = await provider.connection.requestAirdrop(
      user.publicKey,
      1000000000
    );
    await provider.connection.confirmTransaction(airdrop_tx, "confirmed");

    await program.methods
      .initialize({
        command: [
          "--commit",
          "098551f",
        ],
      })
      .accounts({
        buildParams: otter_verify_pda,
        authority: user.publicKey,
        programAddress: other_program_id,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    let params = await program.account.buildParams.fetch(otter_verify_pda);
    assert(params.command[0] == "--commit");
    assert(params.command.length == 2);
  });

  it("Is Updated!", async () => {
    await program.methods
      .update({
        command: [
          "https://github.com/Ellipsis-Labs/phoenix-v1",
          "--commit",
          "098551f",
        ],
      })
      .accounts({
        buildParams: otter_verify_pda,
        authority: user.publicKey,
        programAddress: other_program_id,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    let buildParams = await program.account.buildParams.fetch(otter_verify_pda);
    assert(
      buildParams.command[0]=="https://github.com/Ellipsis-Labs/phoenix-v1"
    );
  });

  it("Is Closed!", async () => {
    await program.methods.close().accounts({
      buildParams: otter_verify_pda,
      programAddress: other_program_id,
      authority: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    
    try {
      await program.account.buildParams.fetch(otter_verify_pda);
      // Should not reach here. If it does, the test should fail.
      assert(false);
    } catch (err) {
      assert(err.toString().includes("Account does not exist or has no data"));
    }
  });
});
