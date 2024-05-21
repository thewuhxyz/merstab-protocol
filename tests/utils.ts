import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MProtocol } from '../target/types/m_protocol';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { manager as managerPair, depositor as depositorPair, depositor2 as depositor2Pair } from '../keypair';
import * as m from '@blockworks-foundation/mango-client';

const manager = Keypair.fromSecretKey(managerPair);
const depositor = Keypair.fromSecretKey(depositorPair);
const depositor2 = Keypair.fromSecretKey(depositor2Pair);

const managerAta = new PublicKey('AaG4TM2DcyEthmcHCiznvFdxrpLVTwEDAEbpCedJtkiv');
const depositorAta = new PublicKey('61ziXYbM94m6a5MGHF952wf7h4FHdKKcfsUYDkX7wHud');
const depositor2Ata = new PublicKey('HAGWevQVSCMvyS64P7EwYnyWNA9FPAXvo3svtYEwBdVr');

export const keys = {manager, depositor, depositor2, managerAta, depositorAta, depositor2Ata}

export const connection = new Connection('https://api.devnet.solana.com');
export const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(manager), {});
anchor.setProvider(provider);

export const program = anchor.workspace.MProtocol as Program<MProtocol>;

export const fetchVaultAccount = async (key: PublicKey) => {
  return await program.account.vault.fetch(key);
};

export const fetchUserVaultAccount = async (key: PublicKey) => {
  return await program.account.userVaultAccount.fetch(key);
};

export const fetchUnstakeReqAccount = async (key: PublicKey) => {
  return await program.account.unstakeReq.fetch(key);
};

export const fetchStakeReqAccount = async (key: PublicKey) => {
  return await program.account.stakeReq.fetch(key);
};

export const fetchTokenAccount = async (key: PublicKey) => {
  return await connection.getTokenAccountBalance(key);
};

export const fetchAllUserVaultAccountForVault = async (vaultKey: PublicKey) => {
    return await program.account.userVaultAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: vaultKey.toBase58(),
        },
      },
    ]);
}

// export const vaultName = Keypair.generate().publicKey.toString().slice(0, 7);

export const getPda = async (vaultName: string) => {
  let [vaultKey, vaultBump] = await PublicKey.findProgramAddress(
    [Buffer.from(vaultName), Buffer.from('vault')],
    program.programId
  );

  let [usdcTokenKey] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), Buffer.from('usdc')],
    program.programId
  );

  let [vaultAuthority, vaultAuthorityBump] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), Buffer.from('pdaauthority')],
    program.programId
  );

//   let stakereq = Keypair.generate();
//   console.log('stake_req:', stakereq.publicKey.toBase58());

//   let unstakereq = Keypair.generate();
//   console.log('unstake_req:', unstakereq.publicKey.toBase58());

  let vaultAuthorityAta = new PublicKey('DzpnbdyVVEs62FmR7Gj4qmomskBbh5sGRrSJpKmK2aHz');

  // * Depositor 1 credentials

  let [depositorUsdcTokenKey] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), depositor.publicKey.toBuffer(), Buffer.from('usdc')],
    program.programId
  );

  let [depositorVaultKey, depositorBump] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), depositor.publicKey.toBuffer()],
    program.programId
  );

  // * Depositor 2 credentials

  let [depositor2UsdcTokenKey] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), depositor2.publicKey.toBuffer(), Buffer.from('usdc')],
    program.programId
  );

  let [depositor2VaultKey, depositor2Bump] = await PublicKey.findProgramAddress(
    [vaultKey.toBuffer(), depositor2.publicKey.toBuffer()],
    program.programId
  );

  return {
    vaultKey,
    usdcTokenKey,
    vaultAuthority,
    vaultAuthorityBump,
    vaultBump,
    depositorBump,
    depositor2Bump,
    // stakereq,
    // unstakereq,
    vaultAuthorityAta,
    depositorUsdcTokenKey,
    depositor2UsdcTokenKey,
    depositorVaultKey,
    depositor2VaultKey,
  };
};

// * mango credentials
export const getMangoData = async (vaultAuthority: PublicKey) => {
  const mangoAddress = new PublicKey('4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA');
  const mangoGroup = new PublicKey('Ec2enZyoC4nGpEfu2sUNAa2nUGJHWxoUWYSEJ2hNTWTA');
  const quoteMint = new PublicKey('8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN');
  // const quoteIndex = 8;
  const quoteIndex = 7;

  let mangoAccountNum = new anchor.BN(2);

  const [mangoAccountKey, mangoBump] = await PublicKey.findProgramAddress(
    [mangoGroup.toBuffer(), vaultAuthority.toBuffer(), mangoAccountNum.toBuffer('le', 8)],
    mangoAddress
  );

  const mangoClient = new m.MangoClient(connection, mangoAddress);

  let mGroup = await mangoClient.getMangoGroup(mangoGroup);

  let rootBanks = (await mGroup.loadRootBanks(connection)).filter((rb) => rb !== undefined) as m.RootBank[];

  for (let i = 0; i < rootBanks.length; i++) {
    await rootBanks[i].loadNodeBanks(connection);
  }

  let quoteRootBank = rootBanks[quoteIndex];

  let quoteVaultKey = quoteRootBank.nodeBankAccounts[0].vault;
  let nodeVaultKey = quoteRootBank.nodeBankAccounts[0].publicKey;
  let mangoCache = await mGroup.loadCache(connection);

  return {
    mangoAddress,
    mGroup,
    quoteMint,
    quoteIndex,
    mangoAccountNum,
    mangoAccountKey,
    mangoBump,
    mangoClient,
    rootBanks,
    quoteRootBank,
    quoteVaultKey,
    nodeVaultKey,
    mangoCache,
  };
};
