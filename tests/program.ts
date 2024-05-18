import * as anchor from "@project-serum/anchor";
import { Tier } from "../target/types/tier";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace
  .Tier as anchor.Program<Tier>;

export const getProgram = () => {
  return program;
};
