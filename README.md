## Blockchain implementation in Rust Lang

This is a basic implementation of a blockchain in Rust, featuring transaction handling, block creation, proof of work, and difficulty adjustments. It simulates a rudimentary blockchain that operates with mining rewards, transactions, and a proof-of-work algorithm.


#### **Overview**
The module defines a simple blockchain system where:

- Transactions are added to the blockchain.
- Blocks are generated using proof of work, and each block contains a hash of the previous block.
- The difficulty can be updated, and mining rewards are distributed to miners.
- Merkle trees are used for transaction validation within each block.
- The blockchain is secured using the SHA-256 hashing algorithm to generate block hashes.
#### **Key Components:**
- Transaction: Represents a simple transaction between a sender and a receiver with a specified amount.
- Blockheader: Contains metadata for each block, including the timestamp, previous block hash, Merkle root, and difficulty.
- Block: Represents a block containing the block header, transactions, and the block count.
- Chain: The blockchain itself, responsible for managing blocks, transactions, difficulty, and mining rewards.
