import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";

const PAYER_KEYPAIR = Keypair.generate();

(async () => {
    const connection = new Connection("http://localhost:8899", "confirmed");
    const programId = new PublicKey(
        "B2FMcdS14hFZjZ2emPC2Mdi8VteExebin8Xz6nguautD"
    );

    // Airdop to Payer
    const signature = await connection.requestAirdrop(PAYER_KEYPAIR.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature);

    const [pda, bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("customaddress"), PAYER_KEYPAIR.publicKey.toBuffer()],
        programId
    );

    let disc = await getDiscriminator("PersonState", "account");
    console.log(disc);

    const [pda_descriptor, _] = await PublicKey.findProgramAddressSync(
        [Buffer.from(disc)],
        programId
    );

    console.log(`PDA Pubkey: ${pda.toString()}`);

    const createPDAIx = new TransactionInstruction({
        programId: programId,
        data: Buffer.from(Uint8Array.of(bump)),
        keys: [
            {
                isSigner: true,
                isWritable: true,
                pubkey: PAYER_KEYPAIR.publicKey,
            },
            {
                isSigner: false,
                isWritable: true,
                pubkey: pda,
            },
            {
                isSigner:false,
                isWritable: true,
                pubkey: pda_descriptor,
            },
            {
                isSigner: false,
                isWritable: false,
                pubkey: SystemProgram.programId,
            },
        ],
    });

    const transaction = new Transaction();
    transaction.add(createPDAIx);

    const txHash = await connection.sendTransaction(transaction, [PAYER_KEYPAIR]);
    console.log(`Created PDA successfully. Tx Hash: ${txHash}`);
    await connection.confirmTransaction(txHash);

    let pda_descriptor_account = await connection.getAccountInfo(pda_descriptor);
    console.log(JSON.stringify(pda_descriptor_account));
})();

//TODO set to GC url
async function getDiscriminator(account_name: string, account_type: string): Promise<Uint8Array> {
    const response = await fetch("http://localhost:8080/discriminator-offline/" + account_name + "/" + account_type);
    // @ts-ignore
    return await response.json();
}