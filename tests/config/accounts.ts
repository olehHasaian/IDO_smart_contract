import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import {
  // @ts-ignore
  createAssociatedTokenAccountInstruction,
  // @ts-ignore
  mintTo,
  createMint,
  createAccount,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

import {
  getDaoTreasuryKey,
  getGlobalStateKey,
  getPoolKey,
  getUserDataKey,
} from "../utils/keys";

import { Tier } from "../../target/types/tier";
import { User } from "./users";
const program = anchor.workspace
  .Tier as anchor.Program<Tier>;

export class BaseAcct {
  public publicKey: PublicKey;
}
export class TokenAcc extends BaseAcct {
  public mint: PublicKey;
  public owner: PublicKey;

  public async initTokenAccount(
    owner: Keypair,
    mint: PublicKey,
    provider?: anchor.Provider
  ) {
    this.mint = mint;
    this.owner = owner.publicKey;
    if (provider) {
      this.publicKey = await createAccount(
        provider.connection,
        owner,
        mint,
        owner.publicKey
      );
    }
  }
  // todo: add getbalance
}

export class ATA extends TokenAcc {
  public async initTokenAccount(
    owner: Keypair,
    mint: PublicKey,
    provider?: anchor.Provider
  ): Promise<void> {
    this.mint = mint;
    this.owner = owner.publicKey;
    if (provider) {
      this.publicKey = await createAssociatedTokenAccount(
        provider.connection,
        owner,
        mint,
        owner.publicKey
      );
    }
  }
}

export class MintAcc extends BaseAcct {
  public mint_authority: PublicKey;
  public freeze_authority: PublicKey;

  public async createMint(
    provider: anchor.Provider,
    payer: Keypair,
    authority: PublicKey,
    decimals: number
  ) {
    this.mint_authority = authority;
    this.publicKey = await createMint(
      provider.connection,
      payer,
      authority,
      null,
      decimals
    );
  }
  public async mintTokens(
    provider: anchor.Provider,
    payer: Keypair,
    toTokenAcc: PublicKey,
    amount: number
  ) {
    console.log("toTokenAcc =", toTokenAcc.toBase58());
    await mintTo(
      provider.connection,
      payer,
      this.publicKey,
      toTokenAcc,
      payer,
      amount
    );
  }
}

export class Accounts {
  public globalState: GlobalStateAccount;
  public pool: PoolTokenAcc;
  public daoTreasury: DaoTreasuryTokenAcc;
  public yoiuTokenMint: MintAcc;

  constructor() {
    this.globalState = new GlobalStateAccount();
    this.pool = new PoolTokenAcc();
    this.daoTreasury = new DaoTreasuryTokenAcc();
    this.yoiuTokenMint = new MintAcc();
  }
  public async init() {
    await this.globalState.initKey();
    await this.pool.initKey();
    await this.daoTreasury.initKey();
  }
}

export class GlobalStateAccount extends BaseAcct {
  public async getInfo() {
    console.log('this.publicKey = ' + this.publicKey)

    return await program.account.globalState.fetchNullable(this.publicKey);
  }
  public async initKey() {
    this.publicKey = await getGlobalStateKey();
  }
}

export class PoolTokenAcc extends TokenAcc {
  public async initKey() {
    this.publicKey = await getPoolKey();
  }
}

export class DaoTreasuryTokenAcc extends TokenAcc {
  public async initKey() {
    this.publicKey = await getDaoTreasuryKey();
  }
}
