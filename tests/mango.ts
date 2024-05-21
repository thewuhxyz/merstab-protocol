import * as anchor from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID, transferChecked } from '@solana/spl-token';
import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import chaiAsPromised from 'chai-as-promised';
import { expect, use } from 'chai';
import {
  program,
  fetchVaultAccount,
  getMangoData,
  getPda,
  keys,
  fetchTokenAccount,
  fetchUserVaultAccount,
  fetchStakeReqAccount,
  connection,
  fetchAllUserVaultAccountForVault,
  fetchUnstakeReqAccount,
} from './utils';

describe('Vault Tests', () => {
  use(chaiAsPromised);
  const vaultName = Keypair.generate().publicKey.toBase58().slice(0, 7);
  const anotherVaultName = Keypair.generate().publicKey.toBase58().slice(0, 7);
  const { manager, depositor, depositor2, managerAta, depositor2Ata, depositorAta } = keys;

  const stakereq = Keypair.generate();
  const unstakereq = Keypair.generate();

  const anotherStakereq = Keypair.generate();
  const anotherUnstakereq = Keypair.generate();

  let tx;

  it('creates a vault', async () => {
    const { vaultKey, vaultAuthorityBump, usdcTokenKey, vaultAuthority, vaultBump } = await getPda(vaultName);
    const { quoteMint } = await getMangoData(vaultAuthority);

    const limit = new anchor.BN(5_000e6);

    tx = await expect(
      program.rpc.createVault(vaultName, limit, vaultBump, vaultAuthorityBump, {
        accounts: {
          vault: vaultKey,
          manager: manager.publicKey,
          // payer: manager.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          stakeReq: stakereq.publicKey,
          unstakeReq: unstakereq.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenAccount: usdcTokenKey,
          tokenMint: quoteMint,
          vaultPdaAuthority: vaultAuthority,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          await program.account.stakeReq.createInstruction(stakereq),
          await program.account.unstakeReq.createInstruction(unstakereq),
        ],
        signers: [manager, stakereq, unstakereq],
      })
    ).to.be.fulfilled;
    console.log('Your transaction signature', tx);

    let vaultInfo = await fetchVaultAccount(vaultKey);

    console.log('vault info:', vaultInfo);
  });

  it('should not create vault with wrong credentials', async () => {
    const { vaultKey, vaultAuthorityBump, usdcTokenKey, vaultAuthority, vaultBump } = await getPda(vaultName);
    const {
      vaultKey: vaultKey2,
      vaultBump: vaultBump2,
      vaultAuthorityBump: vaultAuthorityBump2,
      usdcTokenKey: usdcTokenKey2,
      vaultAuthority: vaultAuthority2,
    } = await getPda(anotherVaultName);
    const { quoteMint } = await getMangoData(vaultAuthority);

    const limit = new anchor.BN(5_000e6);

    // * Recreating Vault should fail
    tx = await expect(
      program.rpc.createVault(vaultName, limit, vaultBump, vaultAuthorityBump, {
        accounts: {
          vault: vaultKey,
          manager: manager.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          stakeReq: stakereq.publicKey,
          unstakeReq: unstakereq.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenAccount: usdcTokenKey,
          tokenMint: quoteMint,
          vaultPdaAuthority: vaultAuthority,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          await program.account.stakeReq.createInstruction(stakereq),
          await program.account.unstakeReq.createInstruction(unstakereq),
        ],
        signers: [manager, stakereq, unstakereq],
      })
    ).to.be.rejected;
    // console.log('recreating vault fail:', tx);

    // * Creating Vault with wrong name should fail
    tx = await expect(
      program.rpc.createVault(
        'wrong name', //! wrong name
        limit,
        vaultBump2,
        vaultAuthorityBump2,
        {
          accounts: {
            vault: vaultKey2,
            manager: manager.publicKey,
            rent: SYSVAR_RENT_PUBKEY,
            stakeReq: anotherStakereq.publicKey,
            unstakeReq: anotherUnstakereq.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenAccount: usdcTokenKey2,
            tokenMint: quoteMint,
            vaultPdaAuthority: vaultAuthority2,
            systemProgram: SystemProgram.programId,
          },
          preInstructions: [
            await program.account.stakeReq.createInstruction(anotherStakereq),
            await program.account.unstakeReq.createInstruction(anotherUnstakereq),
          ],
          signers: [manager, anotherStakereq, anotherUnstakereq],
        }
      )
    ).to.be.rejected;
    // console.log('wrong name fail', tx);

    // * Creating Vault with wrong name should fail
    tx = await expect(
      program.rpc.createVault(anotherVaultName, limit, vaultBump2, vaultAuthorityBump2, {
        accounts: {
          vault: vaultKey, //! wrong vault keys
          manager: manager.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          stakeReq: anotherStakereq.publicKey,
          unstakeReq: anotherUnstakereq.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenAccount: usdcTokenKey2,
          tokenMint: quoteMint,
          vaultPdaAuthority: vaultAuthority2,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          await program.account.stakeReq.createInstruction(anotherStakereq),
          await program.account.unstakeReq.createInstruction(anotherUnstakereq),
        ],
        signers: [manager, anotherStakereq, anotherUnstakereq],
      })
    ).to.be.rejected;
    // console.log('wrong vault key fail', tx);

    tx = await expect(
      program.rpc.createVault(anotherVaultName, limit, vaultBump2, vaultAuthorityBump2, {
        accounts: {
          vault: vaultKey2,
          manager: manager.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          stakeReq: anotherStakereq.publicKey,
          unstakeReq: anotherUnstakereq.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenAccount: usdcTokenKey, //! wrong token account
          tokenMint: quoteMint,
          vaultPdaAuthority: vaultAuthority2,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          await program.account.stakeReq.createInstruction(anotherStakereq),
          await program.account.unstakeReq.createInstruction(anotherUnstakereq),
        ],
        signers: [manager, anotherStakereq, anotherUnstakereq],
      })
    ).to.be.rejected;
    // console.log('wrong token account fail', tx);

    tx = await expect(
      program.rpc.createVault(anotherVaultName, limit, vaultBump2, vaultAuthorityBump2, {
        accounts: {
          vault: vaultKey2,
          manager: manager.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          stakeReq: anotherStakereq.publicKey,
          unstakeReq: anotherUnstakereq.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenAccount: usdcTokenKey2,
          tokenMint: quoteMint,
          vaultPdaAuthority: vaultAuthority2,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          await program.account.stakeReq.createInstruction(anotherStakereq),
          await program.account.unstakeReq.createInstruction(anotherUnstakereq),
        ],
        signers: [manager, anotherStakereq, anotherUnstakereq],
      })
    ).to.be.fulfilled;
    console.log('tx successful', tx);

    let vaultInfo = await fetchVaultAccount(vaultKey);
    console.log('vault info:', vaultInfo);
  });

  it('creates mango account', async () => {
    const { vaultAuthority, vaultKey } = await getPda(vaultName);
    const { mangoAccountKey, mangoAccountNum, mangoBump, mGroup, mangoAddress } = await getMangoData(vaultAuthority);

    tx = await program.rpc.createMangoAccount(mangoAccountNum, mangoBump, {
        accounts: {
          manager: manager.publicKey,
          mangoGroupAi: mGroup.publicKey,
          mangoProgramId: mangoAddress,
          systemProgram: SystemProgram.programId,
          unverifiedMangoAccountPda: mangoAccountKey,
          vault: vaultKey,
          vaultAuthority: vaultAuthority,
        },
        signers: [manager],
      })
    // ).to.be.fulfilled;

    console.log('tx successful:', tx);
  });

  // it("delegates mango account", async () => {
  //   const { vaultAuthority, vaultKey } = await getPda(vaultName);
  //   const { mangoAccountKey, mGroup, mangoAddress } = await getMangoData(vaultAuthority);

  //   tx = await expect(program.rpc.delegateMangoAccount({
  //     accounts: {
  //       delegatePubkey: manager.publicKey,
  //       manager: manager.publicKey,
  //       mangoAccount: mangoAccountKey,
  //       mangoGroup: mGroup.publicKey,
  //       mangoProgramId: mangoAddress,
  //       vault: vaultKey,
  //       vaultAuthority,
  //     },
  //     signers: [manager],
  //   })).to.be.fulfilled;

  //   console.log("tx successful:", tx);
    
  // })

  // it('creates a user vault account', async () => {
  //   const { vaultKey, vaultAuthority, depositorVaultKey, depositorUsdcTokenKey, depositorBump } = await getPda(vaultName);
  //   const { quoteMint } = await getMangoData(vaultAuthority);

  //   console.log("dep acct key:", depositorVaultKey.toBase58())

  //   const address = await PublicKey.createProgramAddress(
  //     [
  //       vaultKey.toBuffer(),
  //       depositor.publicKey.toBuffer(),
  //       Buffer.from([depositorBump])
  //     ],
  //     program.programId
  //   )
    
  //   console.log("address:", address.toBase58());
    

  //   // * Creating account for Depositor 1
  //   const x = await program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositorBump, {
  //     accounts: {
  //       rent: SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenAccount: depositorUsdcTokenKey,
  //       tokenMint: quoteMint,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       userAccountAuthority: depositor.publicKey,
  //       userVaultAccount: depositorVaultKey,
  //       vault: vaultKey,
  //       vaultPdaAuthority: vaultAuthority,
  //     },
  //     signers: [depositor],
  //   })

  //   console.log('tx successful:', x);
  // });

  // it('should not create user vault account with wrong credentials', async () => {
  //   const {
  //     vaultKey,
  //     vaultAuthority,
  //     depositorVaultKey,
  //     depositor2VaultKey,
  //     depositorUsdcTokenKey,
  //     depositor2UsdcTokenKey,
  //     depositor2Bump,
  //   } = await getPda(vaultName);
  //   const { quoteMint } = await getMangoData(vaultAuthority);

  //   // * Creating account for Depositor 2 with wrong token account, should fail
  //   tx = await expect(
  //     program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositor2Bump, {
  //       accounts: {
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenAccount: depositorUsdcTokenKey, // ! wrong token account
  //         tokenMint: quoteMint,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAccountAuthority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.rejected;

  //   // console.log('tx with wrong token account:', tx);

  //   // * Creating account for Depositor 2 with wrong authority, should fail
  //   tx = await expect(
  //     program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositor2Bump, {
  //       accounts: {
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenAccount: depositor2UsdcTokenKey,
  //         tokenMint: quoteMint,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAccountAuthority: depositor.publicKey, // ! wrong authority
  //         userVaultAccount: depositor2VaultKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.rejected;

  //   // console.log('tx with wrong authority account:', tx);

  //   // * Creating account for Depositor 2 with wrong signer, should fail
  //   tx = await expect(
  //     program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositor2Bump, {
  //       accounts: {
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenAccount: depositor2UsdcTokenKey,
  //         tokenMint: quoteMint,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAccountAuthority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor], // ! wrong signer
  //     })
  //   ).to.be.rejected;

  //   // console.log('tx with wrong authority account:', tx);

  //   const { vaultKey: wrongVaultKey, vaultAuthority: wrongVA } = await getPda(anotherVaultName);

  //   // * Creating account for Depositor 2 with wrong signer, should fail
  //   tx = await expect(
  //     program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositor2Bump, {
  //       accounts: {
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenAccount: depositor2UsdcTokenKey,
  //         tokenMint: quoteMint,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAccountAuthority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vault: wrongVaultKey, // ! wrong vault
  //         vaultPdaAuthority: wrongVA, // ! wrong vault authority
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.rejected;

  //   // console.log('tx with wrong vault:', tx);

  //   // * Creating account for Depositor 2
  //   tx = await expect(
  //     program.rpc.createUserVaultAccount(new anchor.BN(500e6), depositor2Bump, {
  //       accounts: {
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenAccount: depositor2UsdcTokenKey,
  //         tokenMint: quoteMint,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAccountAuthority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.fulfilled;

  //   console.log('tx:', tx);
  // });

  // it('deposit to user vault account', async () => {
  //   const { vaultKey, vaultAuthority, depositorVaultKey, depositorUsdcTokenKey } = await getPda(vaultName);

  //   tx = await program.rpc.depositToUserVaultAccount(new anchor.BN(10e6), {
  //     accounts: {
  //       authority: depositor.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       userAta: depositorAta,
  //       userVaultAccount: depositorVaultKey,
  //       userVaultUsdcTokenAccount: depositorUsdcTokenKey,
  //       vault: vaultKey,
  //       vaultPdaAuthority: vaultAuthority,
  //     },
  //     signers: [depositor],
  //   });

  //   console.log('tx successful:', tx);
  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);

  //   expect(depositorTokenAccount.value.amount).to.equal((10e6).toString());

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   expect(depositorVaultInfo.deposit.toNumber()).to.equal(10e6);
  //   expect(depositorVaultInfo.depositLimit.toNumber()).to.equal(500e6 - 10e6);

  //   expect(vaultInfo.deposit.toNumber()).to.equal(10e6);
  //   expect(vaultInfo.limit.toNumber()).to.equal(5000e6 - 10e6);
  // });

  // it('should not deposit to wrong user vault account', async () => {
  //   // todo: deposit over user account limit should fail

  //   const {
  //     vaultKey,
  //     vaultAuthority,
  //     depositorVaultKey,
  //     depositor2VaultKey,
  //     depositorUsdcTokenKey,
  //     depositor2UsdcTokenKey,
  //   } = await getPda(vaultName);
  //   const { vaultKey: vaultKey2, vaultAuthority: vaultAuthority2 } = await getPda(anotherVaultName);

  //   tx = await expect(
  //     program.rpc.depositToUserVaultAccount(new anchor.BN(10e6), {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAta: depositor2Ata,
  //         userVaultAccount: depositor2VaultKey,
  //         userVaultUsdcTokenAccount: depositor2UsdcTokenKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor], // ! wrong signer
  //     })
  //   ).to.be.rejected;

  //   tx = await expect(
  //     program.rpc.depositToUserVaultAccount(new anchor.BN(10e6), {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAta: depositor2Ata,
  //         userVaultAccount: depositor2VaultKey,
  //         userVaultUsdcTokenAccount: depositorUsdcTokenKey, // ! wrong token account
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.rejected;

  //   tx = await expect(
  //     program.rpc.depositToUserVaultAccount(new anchor.BN(10e6), {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAta: depositor2Ata,
  //         userVaultAccount: depositor2VaultKey,
  //         userVaultUsdcTokenAccount: depositor2UsdcTokenKey,
  //         vault: vaultKey2, // ! wrong vault
  //         vaultPdaAuthority: vaultAuthority2, // ! wrong vault authority
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.rejected;

  //   tx = await expect(
  //     program.rpc.depositToUserVaultAccount(new anchor.BN(10e6), {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAta: depositor2Ata,
  //         userVaultAccount: depositor2VaultKey,
  //         userVaultUsdcTokenAccount: depositor2UsdcTokenKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.fulfilled;

  //   console.log('tx:', tx);
  //   let depositor2TokenAccount = await fetchTokenAccount(depositor2UsdcTokenKey);
  //   expect(depositor2TokenAccount.value.amount).to.equal((10e6).toString());

  //   let depositor2VaultInfo = await fetchUserVaultAccount(depositor2VaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   expect(depositor2VaultInfo.deposit.toNumber()).to.equal(10e6);
  //   expect(depositor2VaultInfo.depositLimit.toNumber()).to.equal(500e6 - 10e6);

  //   expect(vaultInfo.deposit.toNumber()).to.equal(20e6);
  //   expect(vaultInfo.limit.toNumber()).to.equal(5000e6 - 20e6);
  // });

  // it('requests to stake', async () => {
  //   const { depositorVaultKey } = await getPda(vaultName);

  //   tx = await expect(
  //     program.rpc.requestToStake(new anchor.BN(5e6), false, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vaultStakeReqAccount: stakereq.publicKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;
  //   console.log('tx successful:', tx);

  //   let stakeReqInfo = await fetchStakeReqAccount(stakereq.publicKey);
  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);

  //   expect(stakeReqInfo.count).to.equal(1);
  //   expect(stakeReqInfo.orders[0].toBase58()).to.equal(depositorVaultKey.toBase58());
  //   expect(depositorVaultInfo.userStake.max).equals(false);
  //   expect(depositorVaultInfo.userStake.cancel).equals(false);
  //   expect(depositorVaultInfo.userStake.stakeRequestActive).equals(true);
  //   expect(depositorVaultInfo.userStake.stakeAmount.toNumber()).equals(5e6);
  // });

  // it('processes stake', async () => {
  //   const { vaultKey, vaultAuthority, usdcTokenKey, depositorVaultKey, depositorUsdcTokenKey } = await getPda(
  //     vaultName
  //   );
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   let stakereqInfo = await fetchStakeReqAccount(stakereq.publicKey);
  //   let count = new anchor.BN(stakereqInfo.count);

  //   let requests = stakereqInfo.orders.slice(0, count.toNumber());

  //   expect(requests[0].toBase58()).to.equal(depositorVaultKey.toBase58());

  //   for (const key of requests) {
  //     const userAcc = await fetchUserVaultAccount(key);
  //     let tx = await expect(
  //       program.rpc.processStake({
  //         accounts: {
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           userTokenAccount: userAcc.tokenAccount,
  //           userVaultAccount: key,
  //           vault: vaultKey,
  //           vaultAuthority: vaultAuthority,
  //           manager: manager.publicKey,
  //           vaultTokenAccount: usdcTokenKey,
  //           mangoAccount: mangoAccountKey,
  //           mangoCache: mangoCache.publicKey,
  //           mangoGroup: mGroup.publicKey,
  //           mangoNodeBank: nodeVaultKey,
  //           mangoProgramId: mangoAddress,
  //           mangoRootBank: quoteRootBank.publicKey,
  //           mangoVault: quoteVaultKey,
  //         },
  //         signers: [manager],
  //       })
  //     ).to.be.fulfilled;
  //     console.log('tx succesful:', tx);
  //   }

  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);
  //   expect(depositorTokenAccount.value.amount).to.equal((10e6 - 5e6).toString());

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   expect(depositorVaultInfo.userStake.max).equals(false);
  //   expect(depositorVaultInfo.userStake.cancel).equals(false);
  //   expect(depositorVaultInfo.userStake.stakeRequestActive).equals(false);
  //   expect(depositorVaultInfo.userStake.stakeAmount.toNumber()).equals(5e6);

  //   expect(depositorVaultInfo.userTotalStake.toNumber()).to.equal(5e6);
  //   expect(depositorVaultInfo.equity.toNumber()).to.equal(5e6);

  //   expect(vaultInfo.totalEquity.toNumber()).to.equal(5e6);
  //   expect(mangoBalance).to.be.approximately(vaultInfo.totalEquity.toNumber(), 0.1);
  // });

  // it('simulates profit by depositing funds to mango account', async () => {
  //   const { vaultKey, vaultAuthority, usdcTokenKey } = await getPda(vaultName);
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     quoteMint,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   let hash = await transferChecked(
  //     connection,
  //     manager,
  //     managerAta,
  //     quoteMint,
  //     usdcTokenKey,
  //     manager,
  //     new anchor.BN(3e6).toNumber(),
  //     6
  //   );

  //   console.log('hash:', hash);

  //   // simulate profit by depositing to mango account
  //   tx = await program.rpc.depositToMango(new anchor.BN(2e6), {
  //     accounts: {
  //       manager: manager.publicKey,
  //       mangoAccount: mangoAccountKey,
  //       mangoCache: mangoCache.publicKey,
  //       mangoGroup: mGroup.publicKey,
  //       mangoNodeBank: nodeVaultKey,
  //       mangoProgramId: mangoAddress,
  //       mangoRootBank: quoteRootBank.publicKey,
  //       mangoVault: quoteVaultKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       vault: vaultKey,
  //       vaultAuthority: vaultAuthority,
  //       vaultTokenAccount: usdcTokenKey,
  //     },
  //     signers: [manager],
  //   });
  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();
  //   expect(mangoBalance).to.be.approximately(7e6, 0.1);
  // });

  // it('updates vault balance', async () => {
  //   const { vaultKey, vaultAuthority, usdcTokenKey } = await getPda(vaultName);
  //   const { mangoAccountKey, mangoCache, mGroup, mangoAddress, quoteRootBank, mangoClient } = await getMangoData(
  //     vaultAuthority
  //   );

  //   tx = await expect(
  //     program.rpc.updateVaultBalance({
  //       accounts: {
  //         manager: manager.publicKey,
  //         mangoAccount: mangoAccountKey,
  //         mangoGroup: mGroup.publicKey,
  //         mangoProgramId: mangoAddress,
  //         mangoRootBank: quoteRootBank.publicKey,
  //         vault: vaultKey,
  //         vaultAuthority: vaultAuthority,
  //         vaultTokenAccount: usdcTokenKey,
  //         mangoCache: mangoCache.publicKey,
  //       },
  //       signers: [manager],
  //     })
  //   ).to.be.fulfilled;

  //   console.log('tx successful:', tx);

  //   let vaultInfo = await fetchVaultAccount(vaultKey);
  //   console.log('vault info:', vaultInfo);

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   console.log('mango balance:', mangoBalance);

  //   expect(vaultInfo.totalEquityBeforeSettlements.toNumber()).to.equal(vaultInfo.totalEquity.toNumber());
  //   expect(vaultInfo.totalEquity.toNumber()).to.be.approximately(mangoBalance, 0.1);
  //   expect(vaultInfo.dayPnl).to.be.approximately(0.4, 0.0001);
  // });

  // it('updates user balances', async () => {
  //   const { vaultKey, depositorVaultKey } = await getPda(vaultName);

  //   const allUserVaultAccounts = await fetchAllUserVaultAccountForVault(vaultKey);
  //   // console.log('all vault accounts:', allUserVaultAccounts);

  //   expect(
  //     allUserVaultAccounts.map((acct) => {
  //       return acct.publicKey.toBase58();
  //     })
  //   ).contains(depositorVaultKey.toBase58());

  //   for (const i in allUserVaultAccounts) {
  //     console.log(`updating balances of account ${i}...`);

  //     tx = await program.rpc.updateUserBalance({
  //       accounts: {
  //         vault: vaultKey,
  //         userVaultAccount: allUserVaultAccounts[i].publicKey,
  //         manager: manager.publicKey,
  //       },
  //       signers: [manager],
  //     });
  //     console.log('tx:', tx);
  //   }

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   expect(depositorVaultInfo.equity.toNumber()).to.be.approximately(7e6, 0.1);
  //   expect(depositorVaultInfo.userPnl).to.equal(0.4);
  // });

  // it('requests to unstake', async () => {
  //   const { depositorVaultKey } = await getPda(vaultName);

  //   tx = await expect(
  //     program.rpc.requestToUnstake(new anchor.BN(3e6), false, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vaultUnstakeReqAccount: unstakereq.publicKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   let unstakeReqInfo = await fetchUnstakeReqAccount(unstakereq.publicKey);
  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);

  //   expect(unstakeReqInfo.count).to.equal(1);
  //   expect(unstakeReqInfo.orders[0].toBase58()).to.equal(depositorVaultKey.toBase58());
  //   expect(depositorVaultInfo.userUnstake.max).equals(false);
  //   expect(depositorVaultInfo.userUnstake.cancel).equals(false);
  //   expect(depositorVaultInfo.userUnstake.unstakeRequestActive).equals(true);
  //   expect(depositorVaultInfo.userUnstake.unstakeAmount.toNumber()).equals(3e6);
  // });

  // it('processes unstake', async () => {
  //   const { vaultKey, vaultAuthority, usdcTokenKey, depositorVaultKey, depositorUsdcTokenKey } = await getPda(
  //     vaultName
  //   );
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   let unstakereqInfo = await fetchUnstakeReqAccount(unstakereq.publicKey);
  //   let count = new anchor.BN(unstakereqInfo.count);
  //   let requests = unstakereqInfo.orders.slice(0, count.toNumber());
  //   expect(requests[0].toBase58()).to.equal(depositorVaultKey.toBase58());

  //   for (const key of requests) {
  //     const userAcc = await fetchUserVaultAccount(key);
  //     let tx = await expect(
  //       program.rpc.processUnstake({
  //         accounts: {
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           userTokenAccount: userAcc.tokenAccount,
  //           userVaultAccount: key,
  //           vault: vaultKey,
  //           vaultAuthority: vaultAuthority,
  //           manager: manager.publicKey,
  //           vaultTokenAccount: usdcTokenKey,
  //           mangoAccount: mangoAccountKey,
  //           mangoCache: mangoCache.publicKey,
  //           mangoGroup: mGroup.publicKey,
  //           mangoNodeBank: nodeVaultKey,
  //           mangoProgramId: mangoAddress,
  //           mangoRootBank: quoteRootBank.publicKey,
  //           mangoVault: quoteVaultKey,
  //           signer: mGroup.signerKey,
  //         },
  //         signers: [manager],
  //         remainingAccounts: new Array(15).fill({
  //           pubkey: PublicKey.default,
  //           isSigner: false,
  //           isWritable: false,
  //         }),
  //       })
  //     ).to.be.fulfilled;
  //     console.log('tx succesful:', tx);
  //   }

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   expect(depositorVaultInfo.userUnstake.max).equals(false);
  //   expect(depositorVaultInfo.userUnstake.cancel).equals(false);
  //   expect(depositorVaultInfo.userUnstake.unstakeRequestActive).equals(false);
  //   expect(depositorVaultInfo.userUnstake.unstakeAmount.toNumber()).equals(3e6);

  //   expect(depositorVaultInfo.userTotalUnstake.toNumber()).to.equal(3e6);
  //   expect(depositorVaultInfo.equity.toNumber()).to.equal(4e6);

  //   expect(vaultInfo.totalEquity.toNumber()).to.equal(4e6);
  //   expect(mangoBalance).to.be.approximately(vaultInfo.totalEquity.toNumber(), 0.1);

  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);
  //   expect(depositorTokenAccount.value.amount).to.equal((5e6 + 3e6).toString());
  // });

  // it('withdraws from user vault account', async () => {
  //   const { vaultKey, vaultAuthority, depositorVaultKey, depositorUsdcTokenKey } = await getPda(vaultName);
  //   tx = await expect(
  //     program.rpc.withdrawFromUserVaultAccount(new anchor.BN(6e6), {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         userAta: depositorAta,
  //         userVaultAccount: depositorVaultKey,
  //         userVaultUsdcTokenAccount: depositorUsdcTokenKey,
  //         vault: vaultKey,
  //         vaultPdaAuthority: vaultAuthority,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   console.log('tx successful:', tx);

  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);
  //   expect(depositorTokenAccount.value.amount).to.equal((2e6).toString());

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   expect(depositorVaultInfo.withdrawal.toNumber()).to.equal(6e6);
  // });

  // it('clears stake and unstake requests', async () => {
  //   const { vaultKey } = await getPda(vaultName);
  //   tx = await expect(
  //     program.rpc.clearStakeRequest({
  //       accounts: {
  //         manager: manager.publicKey,
  //         stakeRequestAccount: stakereq.publicKey,
  //         vault: vaultKey,
  //       },
  //       signers: [manager],
  //     })
  //   ).to.be.fulfilled;

  //   tx = await expect(
  //     program.rpc.clearUnstakeRequest({
  //       accounts: {
  //         manager: manager.publicKey,
  //         unstakeRequestAccount: unstakereq.publicKey,
  //         vault: vaultKey,
  //       },
  //       signers: [manager],
  //     })
  //   ).to.be.fulfilled;

  //   let stakereqInfo = await fetchStakeReqAccount(stakereq.publicKey);
  //   let unstakereqInfo = await fetchUnstakeReqAccount(unstakereq.publicKey);

  //   expect(stakereqInfo.count).to.equal(0);
  //   expect(stakereqInfo.orders[0].toBase58()).to.equal(PublicKey.default.toBase58());

  //   expect(unstakereqInfo.count).to.equal(0);
  //   expect(unstakereqInfo.orders[0].toBase58()).to.equal(PublicKey.default.toBase58());
  // });

  // it('stake max, cancel stake', async () => {
  //   const {
  //     depositorVaultKey,
  //     vaultKey,
  //     depositor2VaultKey,
  //     vaultAuthority,
  //     usdcTokenKey,
  //     depositorUsdcTokenKey,
  //     depositor2UsdcTokenKey,
  //   } = await getPda(vaultName);
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   let amountToStake = new anchor.BN(2e6);

  //   tx = await expect(
  //     program.rpc.requestToStake(amountToStake, false, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vaultStakeReqAccount: stakereq.publicKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   // should stake max
  //   tx = await expect(
  //     program.rpc.requestToStake(amountToStake, true, {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vaultStakeReqAccount: stakereq.publicKey,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.fulfilled;

  //   // should cancel stake
  //   tx = await expect(
  //     program.rpc.updateStakeRequest(amountToStake, false, true, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vault: vaultKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   let stakereqInfo = await fetchStakeReqAccount(stakereq.publicKey);
  //   let count = new anchor.BN(stakereqInfo.count);

  //   let requests = stakereqInfo.orders.slice(0, count.toNumber());

  //   for (const key of requests) {
  //     const userAcc = await fetchUserVaultAccount(key);
  //     let tx = await expect(
  //       program.rpc.processStake({
  //         accounts: {
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           userTokenAccount: userAcc.tokenAccount,
  //           userVaultAccount: key,
  //           vault: vaultKey,
  //           vaultAuthority: vaultAuthority,
  //           manager: manager.publicKey,
  //           vaultTokenAccount: usdcTokenKey,
  //           mangoAccount: mangoAccountKey,
  //           mangoCache: mangoCache.publicKey,
  //           mangoGroup: mGroup.publicKey,
  //           mangoNodeBank: nodeVaultKey,
  //           mangoProgramId: mangoAddress,
  //           mangoRootBank: quoteRootBank.publicKey,
  //           mangoVault: quoteVaultKey,
  //         },
  //         signers: [manager],
  //       })
  //     ).to.be.fulfilled;
  //     console.log('tx succesful:', tx);
  //   }

  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);
  //   expect(depositorTokenAccount.value.amount).to.equal((2e6).toString());

  //   let depositor2TokenAccount = await fetchTokenAccount(depositor2UsdcTokenKey);
  //   expect(depositor2TokenAccount.value.amount).to.equal((0e6).toString());

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let depositor2VaultInfo = await fetchUserVaultAccount(depositor2VaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   expect(depositorVaultInfo.userStake.max).equals(false);
  //   expect(depositorVaultInfo.userStake.cancel).equals(true);
  //   expect(depositorVaultInfo.userStake.stakeRequestActive).equals(false);
  //   expect(depositorVaultInfo.userStake.stakeAmount.toNumber()).equals(2e6);

  //   expect(depositor2VaultInfo.userStake.max).equals(true);
  //   expect(depositor2VaultInfo.userStake.cancel).equals(false);
  //   expect(depositor2VaultInfo.userStake.stakeRequestActive).equals(false);
  //   expect(depositor2VaultInfo.userStake.stakeAmount.toNumber()).equals(2e6);

  //   expect(depositorVaultInfo.userTotalStake.toNumber()).to.equal(5e6);
  //   expect(depositorVaultInfo.userTotalUnstake.toNumber()).to.equal(3e6);
  //   expect(depositorVaultInfo.equity.toNumber()).to.equal(4e6);

  //   expect(depositor2VaultInfo.userTotalStake.toNumber()).to.equal(10e6);
  //   expect(depositor2VaultInfo.userTotalUnstake.toNumber()).to.equal(0e6);
  //   expect(depositor2VaultInfo.equity.toNumber()).to.equal(10e6);

  //   expect(vaultInfo.totalEquity.toNumber()).to.equal(14e6);
  //   expect(mangoBalance).to.be.approximately(vaultInfo.totalEquity.toNumber(), 0.1);
  // });

  // it('simulates loss by withdrawing from mango account, updates vault balances', async () => {
  //   const { vaultKey, vaultAuthority, usdcTokenKey } = await getPda(vaultName);
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   tx = await expect(
  //     program.rpc.withdrawFromMango(new anchor.BN(3.5e6), {
  //       accounts: {
  //         manager: manager.publicKey,
  //         mangoAccount: mangoAccountKey,
  //         mangoCache: mangoCache.publicKey,
  //         mangoGroup: mGroup.publicKey,
  //         mangoNodeBank: nodeVaultKey,
  //         mangoProgramId: mangoAddress,
  //         mangoRootBank: quoteRootBank.publicKey,
  //         mangoVault: quoteVaultKey,
  //         vault: vaultKey,
  //         vaultAuthority: vaultAuthority,
  //         vaultTokenAccount: usdcTokenKey,
  //         signer: mGroup.signerKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //       },
  //       remainingAccounts: new Array(15).fill({
  //         pubkey: PublicKey.default,
  //         isSigner: false,
  //         isWritable: false,
  //       }),
  //       signers: [manager],
  //     })
  //   ).to.be.fulfilled;

  //   tx = await expect(
  //     program.rpc.updateVaultBalance({
  //       accounts: {
  //         manager: manager.publicKey,
  //         mangoAccount: mangoAccountKey,
  //         mangoGroup: mGroup.publicKey,
  //         mangoProgramId: mangoAddress,
  //         mangoRootBank: quoteRootBank.publicKey,
  //         vault: vaultKey,
  //         vaultAuthority: vaultAuthority,
  //         vaultTokenAccount: usdcTokenKey,
  //         mangoCache: mangoCache.publicKey,
  //       },
  //       signers: [manager],
  //     })
  //   ).to.be.fulfilled;

  //   console.log('tx successful:', tx);

  //   let vaultInfo = await fetchVaultAccount(vaultKey);
  //   // console.log('vault info:', vaultInfo);

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   console.log('mango balance:', mangoBalance);

  //   expect(vaultInfo.totalEquityBeforeSettlements.toNumber()).to.equal(vaultInfo.totalEquity.toNumber());
  //   expect(vaultInfo.totalEquity.toNumber()).to.be.approximately(mangoBalance, 0.1);
  //   expect(vaultInfo.dayPnl).to.be.approximately(-0.25, 0.0001);
  // });

  // it('updates user balances 2', async () => {
  //   const { vaultKey, depositorVaultKey, depositor2VaultKey } = await getPda(vaultName);

  //   const allUserVaultAccounts = await fetchAllUserVaultAccountForVault(vaultKey);
  //   // console.log('all vault accounts:', allUserVaultAccounts);

  //   expect(
  //     allUserVaultAccounts.map((acct) => {
  //       return acct.publicKey.toBase58();
  //     })
  //   ).contains(depositorVaultKey.toBase58());

  //   for (const i in allUserVaultAccounts) {
  //     console.log(`updating balances of account ${i}...`);

  //     tx = await program.rpc.updateUserBalance({
  //       accounts: {
  //         vault: vaultKey,
  //         userVaultAccount: allUserVaultAccounts[i].publicKey,
  //         manager: manager.publicKey,
  //       },
  //       signers: [manager],
  //     });
  //     console.log('tx:', tx);
  //   }

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let depositor2VaultInfo = await fetchUserVaultAccount(depositor2VaultKey);

  //   expect(depositorVaultInfo.equity.toNumber()).to.be.approximately(3e6, 0.1);
  //   expect(depositorVaultInfo.userPnl).to.equal(0.2);

  //   expect(depositor2VaultInfo.equity.toNumber()).to.be.approximately(7.5e6, 0.1);
  //   expect(depositor2VaultInfo.userPnl).to.equal(-0.25);
  // });

  // it('cancels unstake, max unstake', async () => {
  //   const {
  //     depositorVaultKey,
  //     vaultKey,
  //     depositor2VaultKey,
  //     vaultAuthority,
  //     usdcTokenKey,
  //     depositorUsdcTokenKey,
  //     depositor2UsdcTokenKey,
  //   } = await getPda(vaultName);
  //   const {
  //     mangoAccountKey,
  //     mangoCache,
  //     mGroup,
  //     nodeVaultKey,
  //     mangoAddress,
  //     quoteRootBank,
  //     quoteVaultKey,
  //     mangoClient,
  //   } = await getMangoData(vaultAuthority);

  //   let amountToUnstake = new anchor.BN(2e6);

  //   tx = await expect(
  //     program.rpc.requestToUnstake(amountToUnstake, false, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vaultUnstakeReqAccount: unstakereq.publicKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   tx = await expect(
  //     program.rpc.requestToUnstake(amountToUnstake, true, {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vaultUnstakeReqAccount: unstakereq.publicKey,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.fulfilled;

  //   // should max unstake
  //   tx = await expect(
  //     program.rpc.updateUnstakeRequest(amountToUnstake, true, false, {
  //       accounts: {
  //         authority: depositor.publicKey,
  //         userVaultAccount: depositorVaultKey,
  //         vault: vaultKey,
  //       },
  //       signers: [depositor],
  //     })
  //   ).to.be.fulfilled;

  //   // update to cancel unstake
  //   tx = await expect(
  //     program.rpc.updateUnstakeRequest(amountToUnstake, false, true, {
  //       accounts: {
  //         authority: depositor2.publicKey,
  //         userVaultAccount: depositor2VaultKey,
  //         vault: vaultKey,
  //       },
  //       signers: [depositor2],
  //     })
  //   ).to.be.fulfilled;

  //   let unstakereqInfo = await fetchUnstakeReqAccount(unstakereq.publicKey);
  //   let count = new anchor.BN(unstakereqInfo.count);
  //   let requests = unstakereqInfo.orders.slice(0, count.toNumber());
  //   // expect(requests[0].toBase58()).to.equal(depositorVaultKey.toBase58());

  //   for (const key of requests) {
  //     const userAcc = await fetchUserVaultAccount(key);
  //     let tx = await expect(
  //       program.rpc.processUnstake({
  //         accounts: {
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           userTokenAccount: userAcc.tokenAccount,
  //           userVaultAccount: key,
  //           vault: vaultKey,
  //           vaultAuthority: vaultAuthority,
  //           manager: manager.publicKey,
  //           vaultTokenAccount: usdcTokenKey,
  //           mangoAccount: mangoAccountKey,
  //           mangoCache: mangoCache.publicKey,
  //           mangoGroup: mGroup.publicKey,
  //           mangoNodeBank: nodeVaultKey,
  //           mangoProgramId: mangoAddress,
  //           mangoRootBank: quoteRootBank.publicKey,
  //           mangoVault: quoteVaultKey,
  //           signer: mGroup.signerKey,
  //         },
  //         signers: [manager],
  //         remainingAccounts: new Array(15).fill({
  //           pubkey: PublicKey.default,
  //           isSigner: false,
  //           isWritable: false,
  //         }),
  //       })
  //     ).to.be.fulfilled;
  //     console.log('tx succesful:', tx);
  //   }

    

  //   let mAcct = await mangoClient.getMangoAccount(mangoAccountKey, mGroup.dexProgramId);
  //   let mangoBalance = mAcct.getAvailableBalance(mGroup, mangoCache, 15).toNumber();

  //   let depositorVaultInfo = await fetchUserVaultAccount(depositorVaultKey);
  //   let depositor2VaultInfo = await fetchUserVaultAccount(depositor2VaultKey);
  //   let vaultInfo = await fetchVaultAccount(vaultKey);

  //   // console.log("depositor vault info:", depositorVaultInfo);
  //   // console.log("depositor2 vault info:", depositor2VaultInfo);
  //   console.log("depositor equity:", depositorVaultInfo.equity.toNumber());
  //   console.log("depositor2 equity:", depositor2VaultInfo.equity.toNumber());

  //   expect(depositorVaultInfo.userUnstake.max).equals(true);
  //   expect(depositorVaultInfo.userUnstake.cancel).equals(false);
  //   expect(depositorVaultInfo.userUnstake.unstakeRequestActive).equals(false);
  //   expect(depositorVaultInfo.userUnstake.unstakeAmount.toNumber()).equals(2e6);

  //   expect(depositor2VaultInfo.userUnstake.max).equals(false);
  //   expect(depositor2VaultInfo.userUnstake.cancel).equals(true);
  //   expect(depositor2VaultInfo.userUnstake.unstakeRequestActive).equals(false);
  //   expect(depositor2VaultInfo.userUnstake.unstakeAmount.toNumber()).equals(2e6);

  //   let depositorTokenAccount = await fetchTokenAccount(depositorUsdcTokenKey);
  //   expect(depositorTokenAccount.value.amount).to.equal((2e6 + 3e6).toString(), 'why?');

  //   let depositor2TokenAccount = await fetchTokenAccount(depositor2UsdcTokenKey);
  //   expect(depositor2TokenAccount.value.amount).to.equal((0e6).toString());
    

  //   expect(depositorVaultInfo.userTotalStake.toNumber()).to.equal(0e6);
  //   expect(depositorVaultInfo.userTotalUnstake.toNumber()).to.equal(0e6);
  //   expect(depositorVaultInfo.equity.toNumber()).to.equal(0e6);
    
  //   expect(depositorVaultInfo.lastTradeStat.userTotalStake.toNumber()).to.equal(5e6);
  //   expect(depositorVaultInfo.lastTradeStat.userTotalUnstake.toNumber()).to.equal(3e6+3e6);
  //   expect(depositorVaultInfo.lastTradeStat.userRealisedPnl).to.equal(0.2);

  //   expect(depositor2VaultInfo.userTotalStake.toNumber()).to.equal(10e6);
  //   expect(depositor2VaultInfo.userTotalUnstake.toNumber()).to.equal(0e6);
  //   expect(depositor2VaultInfo.equity.toNumber()).to.equal(7.5e6);

  //   expect(vaultInfo.totalEquity.toNumber()).to.equal(7.5e6);
  //   expect(mangoBalance).to.be.approximately(vaultInfo.totalEquity.toNumber(), 0.1);
  // });
});
