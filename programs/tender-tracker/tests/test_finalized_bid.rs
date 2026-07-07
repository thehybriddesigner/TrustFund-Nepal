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

use anchor_lang::solana_program::instruction::AccountMeta;
#[test]
fn test_finalize_tendor() {
    let program_id = tender_tracker::id();
    let authority = Keypair::new();

    let bidder_a = Keypair::new();
    let bidder_b = Keypair::new();
    let bidder_c = Keypair::new();

    let tender_id: u64 = 1;
    let tender = Pubkey::find_program_address(
        &[b"tender", tender_id.to_le_bytes().as_ref()],
        &program_id,
    )
    .0;

    let bid_a = Pubkey::find_program_address(
        &[b"bid", tender_id.to_le_bytes().as_ref(), bidder_a.pubkey().as_ref()],
        &program_id,
    )
    .0;
    let bid_b = Pubkey::find_program_address(
        &[b"bid", tender_id.to_le_bytes().as_ref(), bidder_b.pubkey().as_ref()],
        &program_id,
    )
    .0;
    let bid_c = Pubkey::find_program_address(
        &[b"bid", tender_id.to_le_bytes().as_ref(), bidder_c.pubkey().as_ref()],
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
    svm.airdrop(&bidder_a.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&bidder_b.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&bidder_c.pubkey(), 1_000_000_000).unwrap();

    let create_ix = Instruction::new_with_bytes(
        program_id,
        &tender_tracker::instruction::CreateTender {
            tender_id,
            title: "City Road Repair Contract".to_string(),
            description: "Repair and resurfacing of the main city road, 5km stretch".to_string(),
            price_weight: 60,
            timeline_weight: 40,
            deadline: 9999999999,
        }
        .data(),
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
    let mut data: &[u8] = &tender_account.data;
    let tender_state = tender_tracker::state::Tender::try_deserialize(&mut data).unwrap();
    assert_eq!(tender_state.title, "City Road Repair Contract".to_string());

    let submit_a_ix = Instruction::new_with_bytes(
        program_id,
        &tender_tracker::instruction::SubmitBid {
            tender_id,
            price: 1000,
            timeline_days: 20,
            quality_cert: "ISO9001".to_string(),
        }
        .data(),
        tender_tracker::accounts::SubmitBid {
            bidder: bidder_a.pubkey(),
            bid: bid_a,
            tender,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[submit_a_ix], Some(&bidder_a.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&bidder_a]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let bid_a_account = svm.get_account(&bid_a).unwrap();
    let mut bid_a_data: &[u8] = &bid_a_account.data;
    let bid_a_state = tender_tracker::state::Bid::try_deserialize(&mut bid_a_data).unwrap();
    assert_eq!(bid_a_state.price, 1000);
    assert_eq!(bid_a_state.timeline_days, 20);
    assert_eq!(bid_a_state.bidder, bidder_a.pubkey());

    let submit_b_ix = Instruction::new_with_bytes(
        program_id,
        &tender_tracker::instruction::SubmitBid {
            tender_id,
            price: 1200,
            timeline_days: 15,
            quality_cert: "ISO9001".to_string(),
        }
        .data(),
        tender_tracker::accounts::SubmitBid {
            bidder: bidder_b.pubkey(),
            bid: bid_b,
            tender,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[submit_b_ix], Some(&bidder_b.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&bidder_b]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let bid_b_account = svm.get_account(&bid_b).unwrap();
    let mut bid_b_data: &[u8] = &bid_b_account.data;
    let bid_b_state = tender_tracker::state::Bid::try_deserialize(&mut bid_b_data).unwrap();
    assert_eq!(bid_b_state.price, 1200);
    assert_eq!(bid_b_state.timeline_days, 15);
    assert_eq!(bid_b_state.bidder, bidder_b.pubkey());

    let submit_c_ix = Instruction::new_with_bytes(
        program_id,
        &tender_tracker::instruction::SubmitBid {
            tender_id,
            price: 900,
            timeline_days: 25,
            quality_cert: "ISO9001".to_string(),
        }
        .data(),
        tender_tracker::accounts::SubmitBid {
            bidder: bidder_c.pubkey(),
            bid: bid_c,
            tender,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[submit_c_ix], Some(&bidder_c.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&bidder_c]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let bid_c_account = svm.get_account(&bid_c).unwrap();
    let mut bid_c_data: &[u8] = &bid_c_account.data;
    let bid_c_state = tender_tracker::state::Bid::try_deserialize(&mut bid_c_data).unwrap();
    assert_eq!(bid_c_state.price, 900);
    assert_eq!(bid_c_state.timeline_days, 25);
    assert_eq!(bid_c_state.bidder, bidder_c.pubkey());
    

let mut finalize_accounts = tender_tracker::accounts::FinalizeTender {
    tender,
    authority: authority.pubkey(),
}
.to_account_metas(None);

finalize_accounts.push(AccountMeta::new_readonly(bid_a, false));
finalize_accounts.push(AccountMeta::new_readonly(bid_b, false));
finalize_accounts.push(AccountMeta::new_readonly(bid_c, false));

let finalize_ix = Instruction::new_with_bytes(
    program_id,
    &tender_tracker::instruction::FinalizeTender {}.data(),
    finalize_accounts,
);
let blockhash = svm.latest_blockhash();
let msg = Message::new_with_blockhash(&[finalize_ix],Some(&authority.pubkey()), &blockhash);
let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&authority]).unwrap();
let res = svm.send_transaction(tx);
assert!(res.is_ok());

let final_account = svm.get_account(&tender).unwrap();
let mut final_account_data: &[u8] = &final_account.data;
let final_account_state = tender_tracker::state::Tender::try_deserialize(&mut final_account_data).unwrap();
assert_eq!(final_account_state.winner, Some(bid_b));

}
