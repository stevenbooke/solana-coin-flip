import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCoinFlip } from "../target/types/solana_coin_flip";
import { expect } from "chai";

describe("solana-coin-flip", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaCoinFlip as Program<SolanaCoinFlip>;

  it("Creates a UserAccount!", async () => {
    const [userAccountPDA, userAccountBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('create-user-account'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    console.log('programId:', program.programId.toBase58());
    console.log('userAccountPDA:', userAccountPDA.toBase58());
    console.log('userAccountBump:', userAccountBump);
    console.log('wallet:', provider.wallet.publicKey.toBase58());

    await program.methods
      .createUserAccount()
      .accounts({
        userAccount: userAccountPDA,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('create user account success');

    let userAccount = await program.account.userAccount.fetch(userAccountPDA);
    console.log('userAccount.authority:', userAccount.authority.toBase58());
    console.log('userAccount.total_games_played:', userAccount.totalGamesPlayed);
    console.log('userAccount.totalWins:', userAccount.totalWins);
    console.log('userAccount.totalLosses:', userAccount.totalLosses);
    console.log('userAccount.time:', userAccount.time);
    console.log('userAccount.bump:', userAccount.bump);
    expect(userAccount.totalGamesPlayed).to.equal(0);
    expect(userAccount.totalWins).to.equal(0);
    expect(userAccount.totalLosses).to.equal(0);
  });
});

describe("solana-coin-flip", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaCoinFlip as Program<SolanaCoinFlip>;

  // This is for testing user account creation with a keypair that is not the wallet
  // being used in the current environment. I used solana cli to generate new a keypair
  // and airdropped SOL. Using the secret of the generated keypair we can recreate the keypair as seen below
  const newKeypair = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from([
      154, 241, 23, 217, 165, 217, 142, 10, 221, 237, 87, 217, 218, 61, 96, 147,
      98, 63, 186, 210, 35, 94, 119, 14, 21, 12, 107, 112, 8, 20, 14, 11, 40,
      124, 38, 71, 240, 135, 89, 66, 185, 56, 139, 200, 102, 188, 110, 88, 62,
      20, 68, 77, 10, 66, 111, 91, 80, 70, 89, 169, 225, 95, 39, 130,
    ])
  );

  it("Creates a UserAccount!", async () => {
    const [userAccountPDA, userAccountBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('create-user-account'),
        newKeypair.publicKey.toBuffer(),
      ],
      program.programId
    );

    console.log('programId:', program.programId.toBase58());
    console.log('userAccountPDA:', userAccountPDA.toBase58());
    console.log('userAccountBump:', userAccountBump);
    console.log('wallet:', provider.wallet.publicKey.toBase58());
    console.log("newKeypair:", newKeypair.publicKey.toBase58());

    await program.methods
      .createUserAccount()
      .accounts({
        userAccount: userAccountPDA,
        authority: newKeypair.publicKey,
      })
      .signers([newKeypair])
      .rpc();

    console.log('create user account success');

    // This sometimes does not find the userAccount
    // Probably related to network latency and account creation being confirmed
    let userAccount = await (program.account.userAccount.fetch(userAccountPDA));
    console.log('userAccount.authority:', userAccount.authority.toBase58());
    console.log('userAccount.total_games_played:', userAccount.totalGamesPlayed);
    console.log('userAccount.totalWins:', userAccount.totalWins);
    console.log('userAccount.totalLosses:', userAccount.totalLosses);
    console.log('userAccount.time:', userAccount.time);
    console.log('userAccount.bump:', userAccount.bump);
    expect(userAccount.totalGamesPlayed).to.equal(0);
    expect(userAccount.totalWins).to.equal(0);
    expect(userAccount.totalLosses).to.equal(0);
  });
});
