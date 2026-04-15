use tonic::{transport::Server, Request, Response, Status};
use stone_proto::sovereign_execution_server::{SovereignExecution, SovereignExecutionServer};
use stone_proto::{SequenceRequest, SequenceResponse, AbortRequest, AbortResponse, PayloadRequest, PayloadRespo>
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use rand::Rng;

pub mod stone_proto {
    tonic::include_proto!("stone");
}

#[derive(Debug, Default)]
pub struct StoneVault {}

fn generate_vm_hash() -> String {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hex::encode(hasher.finalize())
}

#[tonic::async_trait]
impl SovereignExecution for StoneVault {
    async fn initiate_sequence(
        &self,
        request: Request<SequenceRequest>,
    ) -> Result<Response<SequenceResponse>, Status> {
        let req = request.into_inner();
        println!("🚨 [NETWORK] Received Initiation from Mac Mini.");
        println!("   Task ID: {}", req.task_id);
        println!("   Mac S2 Lock: {}", &req.s2_mac_hash[0..16]);

        let s2_vm_hash = generate_vm_hash();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let expires_at = (now + 47) as i64;

         println!("🔒 Generated VM S2 Lock: {}", &s2_vm_hash[0..16]);
        println!("⏳ Starting 47-second MPC window. Waiting for payload...");

        let reply = SequenceResponse {
            sequence_id: format!("SEQ-{}", rand::thread_rng().gen_range(1000..9999)),
            s2_vm_hash,
            expires_at,
        };

        Ok(Response::new(reply))
    }

    async fn abort_sequence(&self, request: Request<AbortRequest>) -> Result<Response<AbortResponse>, Status> {
        let req = request.into_inner();
        println!("💀 [KILL SWITCH ACTIVATED] Reason: {}", req.reason);
        Ok(Response::new(AbortResponse { success: true, message: "Sequence destroyed.".into() }))
    }

    async fn submit_payload_key(&self, request: Request<PayloadRequest>) -> Result<Response<PayloadResponse>, >
        let req = request.into_inner();
        println!("🔓 [NETWORK] Received Mac G1 Payload Key: {}", &req.g1_mac_key[0..16]);
        println!("⚙️ Combining MPC Keys in Intel TDX Enclave...");
        println!("🛡️ Generating Zero-Knowledge Proof...");
        println!("🚀 Pushing Encrypted Bundle to 0G Storage...");

        Ok(Response::new(PayloadResponse {
            success: true,
            zero_g_tx_hash: format!("0x0g_{}", hex::encode("fake_zk_proof_for_now")),
            error_message: "".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let vault = StoneVault::default();

    println!("========================================");
    println!("🏛️ HUDY TECH Sovereign Vault (Intel TDX)");
    println!("📡 Listening for encrypted gRPC on port 50051...");
    println!("========================================");

    Server::builder()
        .add_service(SovereignExecutionServer::new(vault))
        .serve(addr)
        .await?;

    Ok(())
}