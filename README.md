CONSUS is a pseudo-layer of execution for Solana, leveraging zero-knowledge proofs to offload heavy computations off-chain while preserving trustless on-chain verification.
Its goal is to reduce network congestion by shifting intensive logic outside the base layer and validating it with zk-based cryptographic guarantees.

Architecture

CONSUS is composed of several independent modules, forming a modular zk-execution stack:

Verifier Contract – Anchor-based smart contract deployed on Solana to validate zk proofs.

zkVM Core – Executes WASM guest programs and produces verifiable proof transcripts.

Host/Guest Runtime – WASI-compatible execution contexts for flexible deployments.

Custom Executor – Manages off-chain logic, memory, and proof orchestration.

Frontend Dashboard – React interface for real-time visualization of the system’s state.

Orchestrator (WIP) – REST-based AI module for coordination, fault recovery, and adaptive logic.

Development Stage

Proof circuits and smart contracts are operational and fully testable in local environments.

zkVM handles WASM workloads and outputs valid proofs.

Verifier contract integrates with Solana for on-chain validation.

Orchestrator is in progress and will introduce LLM-based logic routing and AI conflict resolution.

Repo Structure

The monorepo contains fully decoupled folders:

zkvm
guest
host
executor
verifier-contract
frontend-dashboard
docs
consus_full_stack

License

CONSUS is released under the MIT License.
Built for developers rethinking execution, integrity, scalability, and zk-applications within high-performance chains like Solana.
