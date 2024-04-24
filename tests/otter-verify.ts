import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OtterVerify } from "../target/types/otter_verify";

describe("otter-verify", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OtterVerify as Program<OtterVerify>;

  it("Is initialized!", async () => {
    // Add your test here.

    const user = anchor.web3.Keypair.generate();
    // Airdrop some SOL to the user.
    let airdrop_tx = await provider.connection.requestAirdrop(user.publicKey, 1000000000);
    await provider.connection.confirmTransaction(airdrop_tx, "confirmed");

    let other_program_id = new anchor.web3.PublicKey("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");

    const [otter_verify_pda, _] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("otter_verify"),
          user.publicKey.toBuffer(),
          other_program_id.toBuffer(),  
        ],
        program.programId
      );

    await program.methods
      .initialize({
        command: [
          "https://github.com/Ellipsis-Labs/phoenix-v1",
          "--commit",
          "098551f",
        ],
      })
      .accounts({
        buildParamsInit: otter_verify_pda,
        authority: user.publicKey,
        programAddress: other_program_id,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

        let params = await program.account.buildParams.fetch(otter_verify_pda);
        console.log(params);
  });


});
