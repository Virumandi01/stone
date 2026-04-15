use std::time::Duration;
use sha2::{Sha256, Digest};
use rand::Rng;
use tokio::time::sleep;

// Import the generated gRPC code
pub mod stone_proto {
    tonic::include_proto!("stone");
}

use stone_proto::sovereign_execution_client::SovereignExecutionClient;
use stone_proto::{SequenceRequest, PayloadRequest};

// Helper function to generate our S2 and G1 keys
fn generate_ephemeral_key() -> String {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hex::encode(hasher.finalize())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ⚠️ REPLACE THIS WITH YOUR GCP VM'S EXTERNAL IP ADDRESS ⚠️
    let vm_url = "35.200.222.202:50051";

    println!("🛡️ HUDY TECH Sovereign Warden: ONLINE");
    println!("Attempting to connect to Cloud Vault at {}...", vm_url);

    // Establish the gRPC connection to the Cloud VM
    let mut client = match SovereignExecutionClient::connect(vm_url).await {
        Ok(c) => c,
        Err(e) => {
            println!("❌ Connection failed. Check your IP and GCP Firewall: {}", e);
            return Ok(());
        }
    };

    println!("✅ Connected to Vault. Listening for AI Trigger...\n");
    
    // Simulating the Python AI telling the Mac to start
    println!("🚨 [ALERT] AI Agent initiated a transaction!");

    // 1. Generate the Mac's Tier 1 Authorization Key
    let s2_mac_hash = generate_ephemeral_key();
    println!("🔑 Tier 1 (S2) Key Generated: {}", &s2_mac_hash[0..16]);

    let request = tonic::Request::new(SequenceRequest {
        task_id: "AI-TRADE-001".into(),
        target_address: "0G-WALLET-XYZ".into(),
        amount: "100".into(),
        s2_mac_hash: s2_mac_hash.clone(),
    });

    println!("📡 Sending Initiation to Vault...");
    
    // 2. Send the key over the internet to the VM
    let response = client.initiate_sequence(request).await?.into_inner();

    println!("🔒 Vault replied with S2 Lock: {}", &response.s2_vm_hash[0..16]);
    println!("⏳ 47-Second Execution Window Active. (Simulating 5 seconds for test)");

    // Simulating a fast countdown so we don't wait 47 seconds every time we test
    for i in (1..=5).rev() {
        print!("\rTime remaining: {} seconds...", i);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
    }

    // 3. Time expires, release the final payload to the Cloud VM
    println!("\n\n✅ Timer expired. No abort detected.");
    let g1_mac_key = generate_ephemeral_key();

    println!("🔓 Tier 2 (G1) Payload Key unlocked: {}", &g1_mac_key[0..16]);
    println!("🚀 Sending payload to GCP Vault...");

    let payload_req = tonic::Request::new(PayloadRequest {
        sequence_id: response.sequence_id,
        g1_mac_key,
    });

    let payload_res = client.submit_payload_key(payload_req).await?.into_inner();
    
    println!("🎉 SUCCESS! Vault generated ZK-Proof: {}", payload_res.zero_g_tx_hash);

    Ok(())
}