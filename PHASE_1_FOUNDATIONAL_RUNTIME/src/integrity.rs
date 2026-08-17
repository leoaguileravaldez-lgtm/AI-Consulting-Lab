use sha2::{Digest, Sha256};

const PREFIX: &[u8] = b"AI-CONSULTING-LAB\0SHA-256\0";

pub fn domain_separated_sha256(
    domain: &str,
    schema_version: &str,
    canonical_payload: &[u8],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(PREFIX);
    hasher.update((domain.len() as u64).to_be_bytes());
    hasher.update(domain.as_bytes());
    hasher.update((schema_version.len() as u64).to_be_bytes());
    hasher.update(schema_version.as_bytes());
    hasher.update((canonical_payload.len() as u64).to_be_bytes());
    hasher.update(canonical_payload);
    hasher.finalize().into()
}

pub fn verify_domain(domain: &str, required_domain: &str) -> bool {
    domain == required_domain
}
