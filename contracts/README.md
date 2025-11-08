## EventProof Smart Contracts

Verifiable Event Attendance & Access System (on Stellar Soroban)

EventProof enables event organizers to issue verifiable NFT-based attendance tokens (POAPs) on-chain — providing tamper-proof, transferable proof of participation for conferences, hackathons, or online events.
These attendance NFTs can be verified by anyone, linked to participants’ wallets, and optionally tied to decentralized identity proofs or facial verification data — ensuring that only genuine attendees receive verifiable proof of attendance.

---

### Problem

Event organizers struggle with fake, duplicated, or unverifiable certificates and POAPs, especially for online events. Centralized storage systems are unreliable and easily manipulated.
EventProof solves this by providing on-chain proof of attendance using the Stellar Soroban smart contract framework — ensuring immutable records of participation.

---

## How It Works

### vent Creation

- Organizers deploy an event record via the EventFactory contract.

- Each event stores metadata: event name, description, date, organizer address, and optional verification settings (e.g., zk-proof or facial hash requirement).

- he factory assigns a unique event ID.

### NFT Minting (Proof-of-Attendance Token)

- The organizer calls the mintAttendanceToken() function to issue NFTs to verified participants.

- Each NFT includes metadata stored on IPFS, containing:

    - Event name & date

    - Participant wallet address

    - Verification hash (signature or zk-proof)

    - Organizer signature

- The token acts as an immutable attendance certificate.

### Verification Mechanism

- Anyone can call verifyAttendance(address participant, uint eventId) to check authenticity.

- The contract validates:

    - The NFT ownership (ownerOf() check).

    - The event’s authenticity (eventRegistry[eventId] validity).

    - The hash signature (on-chain verification).

### Web3 Résumé Integration

- Participants’ NFT badges can be aggregated into a “Web3 Résumé” showing all verified participation across events — using on-chain queries or a subgraph indexer (to be added later).

---

## ✨ Key Features

- **Event Registry Contract:** Maintains verified records of all registered events, organizer credentials, and event-specific configurations.
- **Attendance NFT (POAP) Contract:** Mints NFT-based certificates tied to real events and participant wallet addresses.
- **Organizer Access Control:**Uses role-based permissions to restrict minting and verification operations to authorized event organizers.
- **Tamper-Proof Verification:** On-chain validation of attendance using signature or hash-based verification logic.
- **Decentralized Metadata:** Stores event and attendance metadata on IPFS for transparency and permanence.
- **Oracles for Data Integrity:** Chainlink oracles fetch data from carbon certification providers (Verra, Gold Standard, UNFCCC).
- **Upgradeable Contract Design:** Built using Soroban modular patterns for future compatibility and upgrades.

---
### 🧩 Tech Stack
| Layer             | Technologies                                 |
| ----------------- | -------------------------------------------- |
| **Smart Contracts:**      | Stellar Soroban framework (Rust) |
| **Token Standard:** | Custom NFT Schema (compliant with Soroban asset model)|
| **Storage**  | Filecoin|

---

## 🔄 User Flow

```mermaid
flowchart TD
    A[Organizer: Deploy EventFactory Contract] --> B[Register Event with Metadata]
    B --> C[Mint Attendance NFT for Verified Participants]
    C --> D[Store Metadata on IPFS (event + participant proof)]
    D --> E[Participant Receives NFT in Wallet]
    E --> F[Public: Verify On-chain Attendance]
    F --> G[Optional: Display in Web3 Résumé Dashboard]

    classDef actor fill:#f1f0ff,stroke:#5c53d4,color:#2a1f8f,font-weight:bold;
    classDef process fill:#eefcff,stroke:#00bcd4,color:#004d61;
    class A,E actor;
    class B,C,D,F,G process;

```
---

## 🧪 Functions Overview
| Function                                               | Description                                           |
| ------------------------------------------------------ | ----------------------------------------------------- |
| `createEvent(name, date, metadataURI, config)`         | Registers a new event on-chain.                       |
| `mintAttendanceToken(eventId, participant, proofHash)` | Mints an NFT certificate for a verified participant.  |
| `verifyAttendance(participant, eventId)`               | Checks if a participant holds a valid attendance NFT. |
| `setOrganizerRole(address organizer)`                  | Grants organizer privileges to an address.            |
| `revokeOrganizerRole(address organizer)`               | Removes organizer privileges.                         |
| `getEventDetails(eventId)`                             | Fetches metadata for a specific event.                |

## Security & Verification
- Role-based permissions for organizers.

- Signature validation for minting rights.

- zk-SNARK and facial verification hooks for advanced proof mechanisms.

- Metadata integrity via IPFS content hashes.

## Roles
| Role                        | Capabilities                                                                            |
| --------------------------- | --------------------------------------------------------------------------------------- |
| **Organizer**               | Registers events, mints attendance NFTs, sets verification criteria.                    |
| **Participant**             | Attends event, receives NFT badge, verifies participation.                              |
| **Verifier (Public / DAO)** | Audits event authenticity, validates organizer credentials, ensures on-chain integrity. |

## 🏗️ Project Structure

```
EventProof/
├── contracts/
├── src/
│   ├── EventFactory.rs           # Main contract for event creation and registry
│   ├── AttendanceNFT.rs          # NFT minting and metadata linkage
│   ├── Verifier.rs               # Verification logic (signatures, zk-proofs)
│   ├── utils/
│   │   ├── ipfs.rs               # Helper for IPFS hash encoding/decoding
│   │   ├── access_control.rs     # Organizer role-based access
│   │   └── crypto.rs             # Signature verification and hashing
│   ├── interfaces/
│   │   ├── IEventFactory.rs
│   │   ├── IAttendanceNFT.rs
│   │   └── IVerifier.rs
│   └── lib.rs                    # Shared constants and macros
│
├── scripts/
│   ├── deploy_event.rs           # Script for deploying EventFactory contract
│   ├── mint_nft.rs               # Script for minting attendance NFTs
│   ├── verify_attendance.rs      # Script for testing verification flow
│   └── utils.rs                  # Common deployment utilities
│
├── tests/
│   ├── event_factory_test.rs
│   ├── attendance_nft_test.rs
│   ├── verifier_test.rs
│   └── integration_test.rs
│
├── Docs/
|
├── Cargo.toml                    # Rust project configuration
└── README.md                 
```

## Quick Start (Local Development)

```
# Clone repository
git clone https://github.com/YOUR_USERNAME/eventproof.git
cd contracts

# Install Rust toolchain
rustup default stable
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install soroban-cli

# Build smart contracts
soroban contract build

# Deploy locally
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/event_factory.wasm --network testnet

# Run tests
cargo test
```