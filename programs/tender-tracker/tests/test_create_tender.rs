
use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_create_tender() {
    let program_id = tender_tracker::id();
    let authority = Keypair::new();
    let tender_id: u64 = 1;
    let tender = Pubkey::find_program_address(
        &[b"tender", tender_id.to_le_bytes().as_ref()],
        &program_id,
    )
    .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/tender_tracker.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();

let create_ix = Instruction::new_with_bytes(
    program_id,
    &tender_tracker::instruction::CreateTender {
        tender_id: 1,
        title: "Road Repair".to_string(),
        description: "Repair the main road".to_string(),
        price_weight: 60,
        timeline_weight: 40,
        deadline: 9999999999,
    }.data(),
    tender_tracker::accounts::CreateTender {
        payer: authority.pubkey(),
        tender,
        system_program: system_program::ID,
    }
    .to_account_metas(None),
);

let blockhash = svm.latest_blockhash();
let msg = Message::new_with_blockhash(&[create_ix], Some(&authority.pubkey()), &blockhash);
let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&authority]).unwrap();
let res = svm.send_transaction(tx);
assert!(res.is_ok());

let tender_account = svm.get_account(&tender).unwrap();
let mut data:&[u8] = &tender_account.data;
let tender_state = tender_tracker::state::Tender::try_deserialize(&mut data).unwrap(); 
assert_eq!(tender_state.title, "Road Repair".to_string());
   }
