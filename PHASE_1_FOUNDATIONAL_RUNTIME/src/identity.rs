#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityKind {
    Aggregate,
    Version,
    Event,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalIdentity {
    pub domain: String,
    pub object_type: String,
    pub kind: IdentityKind,
    pub stable_id: String,
    pub version_hash: String,
    pub provenance_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    EmptyComponent,
    DomainSubstitution,
}

impl CanonicalIdentity {
    pub fn new(
        domain: &str,
        object_type: &str,
        kind: IdentityKind,
        stable_id: &str,
        version_hash: &str,
        provenance_ref: &str,
    ) -> Result<Self, IdentityError> {
        if [domain, object_type, stable_id, version_hash, provenance_ref]
            .iter()
            .any(|v| v.is_empty())
        {
            return Err(IdentityError::EmptyComponent);
        }
        Ok(Self {
            domain: domain.into(),
            object_type: object_type.into(),
            kind,
            stable_id: stable_id.into(),
            version_hash: version_hash.into(),
            provenance_ref: provenance_ref.into(),
        })
    }

    pub fn bind_presented_domain(&self, presented_domain: &str) -> Result<&Self, IdentityError> {
        if self.domain != presented_domain {
            Err(IdentityError::DomainSubstitution)
        } else {
            Ok(self)
        }
    }
}

/// UUIDv7-compatible identities expose approximate creation time in their leading 48 bits.
/// Phase 1 validates supplied bytes only; it does not generate identifiers or authority.
pub fn is_uuid_v7_compatible(bytes: &[u8; 16]) -> bool {
    (bytes[6] >> 4) == 0x7 && (bytes[8] & 0xc0) == 0x80
}
