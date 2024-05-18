import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { airdropSol } from "../utils/utils";
import { BaseAcct, ATA } from "./accounts";
import { Tier } from "../../target/types/tier";
const program = anchor.workspace
  .Tier as anchor.Program<Tier>;

export class User {
  public publicKey: PublicKey;
  public keypair: Keypair;
  public provider: anchor.Provider;
  public data_seed: PublicKey;
  public tokenAccounts: {
    yoiuAta: ATA;
  };  
  constructor(k: Keypair) {
    this.keypair = k;
    this.publicKey = k.publicKey;
  }
  public async init(provider: anchor.Provider) {
    await airdropSol(
      provider,
      this.keypair.publicKey,
      99999 * LAMPORTS_PER_SOL
    );
    this.tokenAccounts = { yoiuAta: new ATA() };    
    this.provider = provider;
    this.data_seed = Keypair.generate().publicKey;
  }
  public async initYoiuAta(yoiuMint: PublicKey) {
    await this.tokenAccounts.yoiuAta.initTokenAccount(
      this.keypair,
      yoiuMint,
      this.provider
    );
  }
}
export class Users {
  public admin: User;
  public test: User;
  public async initAdmin(k: Keypair) {
    this.admin = new User(k);
  }
  public async initTest(k: Keypair) {
    this.test = new User(k);
  }
  public async init(provider: anchor.Provider) {
    await this.admin.init(provider);
    await this.test.init(provider);
  }
}
