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

    const listenerMyEvent = program.addEventListener(
      "otterVerifyEvent",
      (event, slot) => {
        console.log("Event received! slot ${slot} Value", event);
        assert(event.program == "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");
      }
    );

    await program.methods
      .initialize({
        version: "1.0.0",
        gitUrl: "https://github.com/Ellipsis-Labs/phoenix-v1",
        commit: "",
        args: [],
        deploySlot: new anchor.BN(1234567890),
      })
      .accounts({
        buildParams: otter_verify_pda,
        authority: user.publicKey,
        programAddress: other_program_id,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    await new Promise((resolve) => setTimeout(resolve, 5000));
    program.removeEventListener(listenerMyEvent);

    let params = await program.account.buildParams.fetch(otter_verify_pda);
    assert(params.gitUrl == "https://github.com/Ellipsis-Labs/phoenix-v1");
    let slot = Number(params.deploySlot);
    assert(slot == 1234567890);
  });

  it("Is Updated!", async () => {
    await program.methods
      .update({
        version: "1.0.0",
        gitUrl: "https://github.com/Ellipsis-Labs/phoenix-v1",
        commit: "098551f",
        args: ["--libname", "phoenix-v1"],
        deploy_slot: 0,
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
    assert(params.commit == "098551f");
    assert(params.args[0] == "--libname");
    assert(params.args[1] == "phoenix-v1");
  });

  it("Is Closed!", async () => {
    await program.methods
      .close()
      .accounts({
        buildParams: otter_verify_pda,
        authority: user.publicKey,
        programAddress: other_program_id,
      })
      .signers([user])
      .rpc();

    try {
      await program.account.buildParams.fetch(otter_verify_pda);
      // Should not reach here. If it does, the test should fail.
      assert(false);
    } catch (err) {
      assert(err.toString().includes("Account does not exist or has no data"));
    }
  });
});
