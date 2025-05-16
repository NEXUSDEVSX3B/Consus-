# Consus Architecture

Consus is composed of five layers:

1. zkVM Layer: Generates verifiable ZK-proofs (Risc0/SP1/Succinct)
2. Executor Layer: Rust-based system running parallel off-chain logic and batching
3. Verifier Contract: Anchor contract on Solana validating ZK proofs
4. AI Swarm Layer: LangChain-based classification of batch priority and spam detection
5. Frontend: React dashboard visualizing execution and system state
