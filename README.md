# solana_multisig_wallet


# 🔐 Solana Multisig Wallet (Anchor + Codigo)

Hi! This is a **Multisig Wallet** smart contract built on **Solana** using the **Anchor framework**. I created this as part of the **Codigo x Superteam Nigeria DevQuest**.

It allows multiple people to control a wallet together, so no single person can send funds without the others agreeing. This is perfect for DAOs, team treasuries, or any group fund that needs shared control.

---

## ✨ What It Does

- 👥 Create a wallet with multiple signers (e.g., 5 people)
- ✅ Set how many people must approve a transaction (e.g., 3 out of 5)
- 📝 Any signer can create a proposal (e.g., send tokens)
- 👍 Others can approve the proposal
- 🚀 Once enough approvals are collected, anyone can execute it
- ⌛ Proposals can have expiration timestamps
- ❌ Creators can cancel their proposals before they’re executed
- 🔄 You can update the signer list or approval threshold (only if **everyone agrees**)

---

## 🧠 Use Cases

- DAO treasury management
- Team or co-founder shared wallets
- Community grant distribution
- Secure token disbursement with checks and balances

---

## 🧱 How It Works (Under the Hood)

### 🧾 Data Structures

- `MultisigConfig`: Holds the signer list, threshold, and a counter for proposals
- `Proposal`: Holds transaction details (destination, amount, approvals, expiration, etc.)

### 🧩 Instructions (Methods)

- `create_multisig`: Create a new multisig wallet
- `create_proposal`: Start a new transaction proposal
- `approve_proposal`: Let a signer approve a proposal
- `execute_proposal`: Run the transaction if enough signers approved
- `cancel_proposal`: Cancel a proposal before it's executed
- `update_multisig`: Change signers or approval threshold with full approval

### 🔐 PDAs (Program Derived Addresses)

- `Multisig PDA`: Unique address for each multisig wallet
- `Proposal PDA`: Unique address for each proposal

### ⚠️ Error Handling

- `NotASigner`: Unauthorized approval attempt
- `ThresholdNotMet`: Tried to execute too early
- `ProposalExpired`: Proposal expired before approval
- `AlreadyExecuted`: Proposal was already run
- `NotAllSignersApproved`: Not everyone agreed to update the wallet

---

## 🧪 How to Run Tests

Make sure you have Anchor installed:

```bash
npm install -g @coral-xyz/anchor-cli


Then run

anchor test


🛠 Built With
🧱 Solana

⚓ Anchor Framework

🤖 Codigo AI — used to generate and scaffold this project


💬 Feedback on Codigo
"Codigo helped me build and structure a full Anchor smart contract quickly. I loved the AI-generated code and tests — it saved me time. It would be even better if we could test directly inside the platform or have automatic deployment setup!"


👤 Author
Kassim Taiwo
GitHub: @taiwokassim

Project submitted for the Superteam Nigeria x Codigo DevQuest 🚀


