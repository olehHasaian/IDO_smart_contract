import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect, assert, use as chaiUse } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { EscrowTier } from "../target/types/escrow_tier";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as keys from "./utils/keys"
import { User, Users } from "./config/users";
import { Accounts } from "./config/accounts";
import { YOIU_DECIMALS } from "./utils/constants";
import { emergency_withdraw, helloWorld, initializeProgram, stake, withdraw } from "./utils/instruction";
chaiUse(chaiAsPromised);

describe("escrow_tier", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tier as Program<Tier>;
  //const user = (program.provider as anchor.AnchorProvider).wallet

  const accts = new Accounts();
  const users = new Users();

  it("setup", async () => {
    await accts.init();
    users.initAdmin((provider.wallet as anchor.Wallet).payer);
    users.initTest(Keypair.generate());    
    await users.init(provider);

    await accts.yoiuTokenMint.createMint(
      provider,
      users.admin.keypair,
      users.admin.keypair.publicKey,
      YOIU_DECIMALS
    );
    await users.admin.initYoiuAta(accts.yoiuTokenMint.publicKey);    
    await users.test.initYoiuAta(accts.yoiuTokenMint.publicKey);

    // mint tokens to test user
    await accts.yoiuTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      users.test.tokenAccounts.yoiuAta.publicKey,
      1_000_000_000
    );
    // mint tokens to admin user
    await accts.yoiuTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      users.admin.tokenAccounts.yoiuAta.publicKey,
      1_000_000_000
    );
  })

  // it("Hello World", async () => {
  //   const tx = await helloWorld(users.admin);
  // });
  // return;

  it("Is initialized!", async () => {
    const tx = await initializeProgram(
      users.admin,
      accts.yoiuTokenMint.publicKey
    );
    console.log("Your transaction signature", tx);
    await accts.yoiuTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      await keys.getPoolKey(),
      1_000_000_000_000
    );
  });

  it("Stake tokens", async () => {
    const tx = await stake(users.test, accts, new anchor.BN(1_000_000_000));
    console.log("Your transaction signature", tx);
  });
  
  it("Stake tokens - ADMIN", async () => {
    const tx = await stake(users.admin, accts, new anchor.BN(3_00_000_000));
    console.log("Your transaction signature", tx);
  });

  it("Withdraw tokens - Test", async () => {
    const tx = await emergency_withdraw(users.admin, users.test, accts);
    console.log("Your transaction signature", tx);
  });
  
});
