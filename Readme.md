# Tender Chain — Transparent Public Procurement on Solana

## Problem

Public tenders are meant to be competitive and merit-based, but in practice, selection
often happens through backroom discretion — an official can quietly favor a preferred
company even when better bids exist, because the final decision is made by a person
*after* seeing all submissions, with no enforced, auditable rule tying the outcome to
the bids themselves.

## Solution

Tender Chain moves the decision-making rule on-chain, **before** any bids are submitted:

1. A tender authority publishes a tender with a **locked, immutable scoring formula**
   (weighted combination of price and timeline) on Solana.
2. Companies submit bids, each stored as its own on-chain account.
3. Once bidding closes, the winner is **computed automatically** by the pre-published
   formula — not chosen by a person after the fact.

Because the formula is public and unchangeable once set, and Solana's ledger is
tamper-evident, no one — including the authority that created the tender — can quietly
alter the rules or the outcome after bids are in.

## What this solves (and what it doesn't)

- ✅ Solves: selection corruption after bids are submitted (favoritism/bribery in
  choosing a winner despite better bids existing)
- ✅ Solves: tamper-proof, publicly auditable bid history
- ⚠️ Does not solve (by design, for this MVP): independent verification of
  self-reported quality/certification claims (e.g. ISO numbers). This is a known
  challenge in blockchain systems called the **oracle problem** — smart contracts
  cannot natively verify off-chain facts. A production version would integrate an
  oracle service (e.g. Pyth, Chainlink) or registrar-signed attestations. For this
  MVP, quality data is self-reported by bidders as part of their bid.
- ⚠️ Does not solve: whether a tender gets published in the first place (an official
  could still choose not to use the system at all) — this is a policy/adoption
  problem, not a technical one.

## Tech Stack

- **Blockchain:** Solana (deployed to Devnet for this submission)
- **Smart contract framework:** Anchor (Rust)
- **Frontend:** React + Vite, styled with Tailwind CSS, using `@coral-xyz/anchor` and
  `@solana/wallet-adapter` for wallet connection and on-chain calls. Chosen for fast
  setup, minimal boilerplate, and a clean, modern UI out of the box.
- **Wallet integration:** Phantom/Solflare via Solana Wallet Adapter

## Program Structure

- `create_tender` — authority publishes a tender with locked scoring weights
- `submit_bid` — bidder submits price, timeline, and quality info to a PDA
  derived from `[tender_id, bidder_pubkey]`
- `finalize_tender` — closes bidding and computes the winner from the locked formula

## Status

🚧 **In active development.** Core Anchor program scaffolded; tender creation, bid
submission, and finalization logic in progress. Intended to continue as a longer-term
prototype beyond this hackathon submission.

## Setup

```bash
anchor build
anchor test
```

## Team

- Kshitij Ban
- Krishna Thakur

## License

None
