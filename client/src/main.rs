use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey,
        signature::read_keypair_file, signer::Signer,
    },
    Cluster,
};
use little_adventure::GameDataAccount;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting program!");

    let rpc_url = "http://127.0.0.1:8899";
    let cluster = Cluster::Custom(rpc_url.to_string(), rpc_url.to_string());

    let keypair_path = match std::env::current_dir() {
        Ok(current_dir_path) => {
            let path = current_dir_path
                .join("programs/little-adventure/payer-keypair.json");

            println!("Current dir path: {:?}", current_dir_path);
            println!("Path: {:?}", path);

            path
        }
        Err(e) => panic!("Failed to get current working directory: {:?}", e),
    };

    let payer =
        Rc::new(read_keypair_file(keypair_path).expect("failed to read payer"));

    let client = anchor_client::Client::new_with_options(
        cluster,
        payer.clone(),
        CommitmentConfig::processed(),
    );

    let program = client.program(little_adventure::ID).unwrap();

    let (new_account_pda, _bump) =
        Pubkey::find_program_address(&[b"Level4"], &little_adventure::ID);

    // let _ = program
    //     .request()
    //     .accounts(little_adventure::accounts::Initialize {
    //         new_game_data_account: new_account_pda,
    //         signer: payer.pubkey(),
    //         system_program: anchor_client::anchor_lang::system_program::ID,
    //     })
    //     .args(instruction::Initialize {})
    //     .send()?;

    let _ = match program
        .request()
        .accounts(little_adventure::accounts::MovePlayer {
            game_data_account: new_account_pda,
        })
        .args(little_adventure::instruction::MovePlayer {
            next_position: (12, 3, 0),
        })
        .send()
    {
        Ok(sig) => println!("Transaction successful with sig: {}", sig),
        Err(e) => panic!("Transaction failed with: {:?}", e),
    };

    println!("program success");

    let my_account: GameDataAccount = program.account(new_account_pda)?;

    println!("On-chain data is: {:?}", my_account.player_position);

    Ok(())
}
