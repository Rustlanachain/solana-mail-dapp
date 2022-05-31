import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaMailDapp } from "../target/types/solana_mail_dapp";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("solana-mail-dapp", () => {
  const provider = anchor.AnchorProvider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaMailDapp as Program<SolanaMailDapp>;

  
  it('can send a new tweet', async () => {
    // Call the "SendTweet" instruction.
    const mail = anchor.web3.Keypair.generate();
    await program.rpc.sendMail('veganism', 'Hummus, am I right?', {
        accounts: {
            mail: mail.publicKey,
            sender: provider.wallet.publicKey,
            receiver: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [mail],
    });

    // Fetch the account details of the created tweet.
    const mailAccount = await program.account.mail.fetch(mail.publicKey);
  	//console.log(mailAccount);
    // Ensure it has the right data.
    assert.equal(mailAccount.sender.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(mailAccount.subject, 'veganism');
    assert.equal(mailAccount.body, 'Hummus, am I right?');
    assert.ok(mailAccount.timestamp);
});

});
