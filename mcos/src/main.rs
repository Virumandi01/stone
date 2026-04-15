use std::time::Duration;
use sha2::{Sha256, Digest};
use rand::Rng;
use tokio::time::sleep;

// Import the gRPC code generated from stone.proto
pub mod stone_proto {
    tonic::include_proto!("stone");
}

use stone_proto::{SequenceRequest, SequenceResponse};

// A helper function to generate our secure, ephemeral keys
fn generate_ephemeral_key() -> String {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hex::encode(hasher.finalize())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️  HUDY TECH Sovereign Warden: OFFLINE");
    println!("Starting Secure gRPC Listener on Mac Mini...");
    
    // In a full implementation, this would be a Tonic gRPC Server listening on port 50052
    // For now, let's simulate receiving the trigger from the Python AI
    
    println!("🛡️  Warden is ONLINE and listening for AI Agent...\n");
    
    // Simulating the AI pinging the Mac
    simulate_ai_trigger().await;

    Ok(())
}

async fn simulate_ai_trigger() {
    println!("🚨 [ALERT] AI Agent initiated a transaction!");
    
    // 1. Generate the S2 Key (The 47-second Authorization Hash)
    let s2_mac_hash = generate_ephemeral_key();
    println!("🔑 Tier 1 (S2) Key Generated: {}", &s2_mac_hash[0..16]);

    // 2. Start the Sovereign Override Countdown
    println!("⏳ 47-Second Execution Window Active.");
    println!("Press [Ctrl+C] to trigger the Kill Switch and abort.\n");

    for i in (1..=47).rev() {
        print!("\rTime remaining: {} seconds...", i);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
    }

    // 3. If the user didn't abort, generate the Payload Key
    println!("\n\n✅ Timer expired. No abort detected.");
    let g1_mac_key = generate_ephemeral_key();
    
    println!("🔓 Tier 2 (G1) Payload Key unlocked: {}", &g1_mac_key[0..16]);
    println!("🚀 Sending payload to GCP Vault (Node C) via gRPC...");
    
    // (Next step: We will add the Tonic gRPC client here to actually send it to the GCP IP address)
}