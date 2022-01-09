const anchor = require('@project-serum/anchor');

const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...");
  
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.binary_game;

  const baseAccount1 = anchor.web3.Keypair.generate();
  const baseAccount2 = anchor.web3.Keypair.generate();

  
  //user1
  const tx1 = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount1.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount],
  });

  console.log("user1 transaction signiture", tx1);

  //user2
  const tx2 = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount2.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount],
  });

  console.log("user1 transaction signiture", tx2);

  let account1 = await program.account.baseAccount1.fetch(baseAccount.publicKey);
  let account2 = await program.account.baseAccount2.fetch(baseAccount.publicKey);

  //call place_bet
  const poolWallet = anchor.web3.Keypair.generate();
  await program.rpc.place_bet(0.5, 0,"user1",{
    
    accounts: {
      baseAccount: baseAccount.publicKey,
      poolWallet: poolWallet.publicKey,
      user:provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount],
  });
  await program.rpc.addGif(1, 1,"user2",{
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  });
  // await program.rpc.addResult("winner 1",0,{
  //   accounts: {
  //     baseAccount: baseAccount.publicKey,
  //   }
  // });
  // await program.rpc.placeBet("winner 1",1,125,"user",  {
  //   accounts: {
  //     baseAccount: baseAccount.publicKey,
  //   }
  // })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(account1.publicKey);
  console.log(account2.publicKey);



};

const runMain = async () => {
  try{
    await main();
    process.exit(0);
  }catch (error){
    console.error(error);
  }
};

runMain();