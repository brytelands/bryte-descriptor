import * as anchor from "@coral-xyz/anchor";
import {LAMPORTS_PER_SOL} from "@solana/web3.js";
import solanaWeb3, {Keypair, sendAndConfirmTransaction} from "@solana/web3.js";
import {initialize, InitializeAccounts} from "../app/generated/instructions";
import {Program} from "@coral-xyz/anchor";
import {SolaluminDemo} from "../app/types/solalumin_demo";
import fs from 'fs';

describe("initialize-demo", () => {
    const program = anchor.workspace.SolaluminDemo as Program<SolaluminDemo>;

    console.log("ProgramID: " + program.programId.toString());
    anchor.setProvider(anchor.AnchorProvider.env());

    let connection = anchor.AnchorProvider.env().connection;
    let payer = Keypair.generate();
    fs.writeFileSync('./tests/payer_demo_keypair.json', JSON.stringify(payer));
    console.log("Payer pubkey: " + payer.publicKey);

    let example_account = Keypair.generate();
    fs.writeFileSync('./tests/example_account_keypair.json', JSON.stringify(example_account));
    console.log("ExampleAccount pubkey: " + example_account.publicKey);

    it("Is initialized!", async () => {

        let disc = await getDiscriminator("ExampleAccount", "account");
        console.log("ExampleAccount discriminator: " + disc);

        const signature = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
        await connection.confirmTransaction(signature);

        const transaction = new solanaWeb3.Transaction();

        const [exampleAccountDescriptorPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(disc)],
            program.programId
        )

        console.log("ExampleAccountDescriptor: " + exampleAccountDescriptorPda);

        let initializeAccounts: InitializeAccounts = {
            exampleAccountDescriptor: exampleAccountDescriptorPda,
            exampleAccount: example_account.publicKey,
            signer: payer.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId
        }

        const ix = initialize(initializeAccounts)
        transaction.add(ix)

        await sendAndConfirmTransaction(connection, transaction, [payer, example_account]);

        let exampleAccountData = await connection.getAccountInfo(example_account.publicKey);
        console.log("ExampleAccount data:")
        console.log(JSON.stringify(exampleAccountData));

        let exampleAccountDescriptorData = await connection.getAccountInfo(exampleAccountDescriptorPda);
        console.log("ExampleAccountDescriptor data:")
        console.log(JSON.stringify(exampleAccountDescriptorData));
    });
});

//TODO set to GC url
async function getDiscriminator(account_name: string, account_type: string): Promise<Uint8Array> {
    const response = await fetch("http://localhost:8080/discriminator-offline/" + account_name + "/" + account_type);
    // @ts-ignore
    return await response.json();
}