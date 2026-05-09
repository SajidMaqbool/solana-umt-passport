# 🛡️ Solana Passport - UMT Edition

A blockchain-based verification system that allows students to link their Solana wallets with their academic identity.

## 🚀 Overview
This project is built for the **Blockchain Course at UMT Lahore**. It uses the **Solana Blockchain** to ensure that student records are immutable, decentralized, and easily verifiable.

## 🛠️ Tech Stack
- **Smart Contract:** Rust & Anchor Framework
- **Frontend:** React.js / Next.js
- **Wallet Support:** Solana Wallet Adapter (Phantom, Solflare)
- **Deployment:** Solana Devnet

## ✨ Features
- **Wallet Verification:** Connect your Solana wallet securely.
- **On-Chain Identity:** Create a "Passport" with your Name and UMT ID.
- **CRUD Operations:** Create, Read, Update, and Delete your on-chain profile.
- **UMT Branding:** Customized UI featuring the UMT Lahore Logo.

## 📜 Smart Contract Logic
The program is deployed on Solana Devnet. It handles:
1. `initialize_passport`: Creates a new record.
2. `update_passport`: Allows users to modify their data.
3. `delete_passport`: Removes the record and claims back the rent (SOL).
