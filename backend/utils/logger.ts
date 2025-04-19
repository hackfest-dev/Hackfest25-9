import { Connection, PublicKey } from '@solana/web3.js';

export async function monitorLogs(connection: Connection, programId: string) {
    console.log(`Monitoring logs for program: ${programId}`);
    connection.onLogs(
        new PublicKey(programId),
        (logs) => {
            console.log('=== Program Log ===');
            console.log('Signature:', logs.signature);
            console.log('Logs:', logs.logs);
            console.log('==================');
        },
        'confirmed'
    );
}
