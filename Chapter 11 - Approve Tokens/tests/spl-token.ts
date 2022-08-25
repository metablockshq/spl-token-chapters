import * as anchor from "@project-serum/anchor";
import { Program, Provider } from "@project-serum/anchor";
import { SplToken } from "../target/types/spl_token";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import idl from "../target/idl/spl_token.json"; // this generated when we run anchor test command
import { assert } from "chai";

describe("spl-token", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.SplToken as Program<SplToken>;
  const payer = anchor.web3.Keypair.generate();
  const anotherWallet = anchor.web3.Keypair.generate(); // newly created another wallet

  before("Add sols to wallet ", async () => {
    await addSols(provider, payer.publicKey); // add some sols before calling test cases
    await addSols(provider, anotherWallet.publicKey); // add sols to another wallet
  });

  it("Spl token is initialized!", async () => {
    const [splTokenMint, _1] = await findSplTokenMintAddress();

    const [vaultMint, _2] = await findVaultAddress();

    const tx = await program.methods
      .createMint()
      .accounts({
        splTokenMint: splTokenMint,
        vault: vaultMint,
        payer: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    const vaultData = await program.account.vault.fetch(vaultMint);

    assert(
      vaultData.splTokenMint.toString() === splTokenMint.toString(),
      "The spl token mint should be same"
    );

    console.log("Your transaction signature", tx);
  });

  // add this block into the describe block of the test file
  it("should mint the spl-token-mint to payer_mint_ata", async () => {
    const [splTokenMint, _1] = await findSplTokenMintAddress();

    const [vaultMint, _2] = await findVaultAddress();

    const [payerMintAta, _3] = await findAssociatedTokenAccount(
      payer.publicKey,
      splTokenMint
    );

    const tx = await program.methods
      .transferMint()
      .accounts({
        splTokenMint: splTokenMint,
        vault: vaultMint,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        payerMintAta: payerMintAta,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("should delegate 5 tokens from payer_mint_ata to another_authority", async () => {
    try {
      const [splTokenMint, _1] = await findSplTokenMintAddress();

      const [vaultMint, _2] = await findVaultAddress();

      const [payerMintAta, _3] = await findAssociatedTokenAccount(
        payer.publicKey,
        splTokenMint
      );

      const tx = await program.methods
        .approveTokens()
        .accounts({
          splTokenMint: splTokenMint,
          vault: vaultMint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          payerMintAta: payerMintAta,
          payer: payer.publicKey,
          anotherAuthority: anotherWallet.publicKey,
        })
        .signers([payer, anotherWallet])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (err) {
      console.log(err);
    }
  });
});

// pda for spl-token-mint account
export const findSplTokenMintAddress = async () => {
  return await PublicKey.findProgramAddress(
    [Buffer.from("spl-token-mint")],
    new PublicKey(idl.metadata.address)
  );
};

// pda for vault account
export const findVaultAddress = async () => {
  return await PublicKey.findProgramAddress(
    [Buffer.from("vault")],
    new PublicKey(idl.metadata.address)
  );
};

export const addSols = async (
  provider: Provider,
  wallet: anchor.web3.PublicKey,
  amount = 1 * anchor.web3.LAMPORTS_PER_SOL
) => {
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(wallet, amount),
    "confirmed"
  );
};

export const findAssociatedTokenAccount = async (
  payerKey: PublicKey,
  mintKey: PublicKey
) => {
  return await PublicKey.findProgramAddress(
    [
      payerKey.toBuffer(), // could be any public key
      TOKEN_PROGRAM_ID.toBuffer(),
      mintKey.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
};
