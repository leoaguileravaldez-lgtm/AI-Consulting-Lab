use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    FloatingPointUnsupported,
    IntegerOutOfRange,
}

fn major(out: &mut Vec<u8>, kind: u8, value: u64) {
    let prefix = kind << 5;
    match value {
        0..=23 => out.push(prefix | value as u8),
        24..=0xff => out.extend([prefix | 24, value as u8]),
        0x100..=0xffff => {
            out.push(prefix | 25);
            out.extend((value as u16).to_be_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(prefix | 26);
            out.extend((value as u32).to_be_bytes());
        }
        _ => {
            out.push(prefix | 27);
            out.extend(value.to_be_bytes());
        }
    }
}

fn encode(value: &Value, out: &mut Vec<u8>) -> Result<(), CanonicalError> {
    match value {
        Value::Null => out.push(0xf6),
        Value::Bool(false) => out.push(0xf4),
        Value::Bool(true) => out.push(0xf5),
        Value::Number(number) => {
            if let Some(n) = number.as_u64() {
                major(out, 0, n);
            } else if let Some(n) = number.as_i64() {
                let encoded = u64::try_from(-1i128 - n as i128)
                    .map_err(|_| CanonicalError::IntegerOutOfRange)?;
                major(out, 1, encoded);
            } else {
                return Err(CanonicalError::FloatingPointUnsupported);
            }
        }
        Value::String(text) => {
            major(out, 3, text.len() as u64);
            out.extend(text.as_bytes());
        }
        Value::Array(values) => {
            major(out, 4, values.len() as u64);
            for item in values {
                encode(item, out)?;
            }
        }
        Value::Object(map) => {
            let mut entries = Vec::with_capacity(map.len());
            for (key, value) in map {
                let mut encoded_key = Vec::new();
                encode(&Value::String(key.clone()), &mut encoded_key)?;
                entries.push((encoded_key, value));
            }
            entries.sort_by(|a, b| a.0.len().cmp(&b.0.len()).then_with(|| a.0.cmp(&b.0)));
            major(out, 5, entries.len() as u64);
            for (key, value) in entries {
                out.extend(key);
                encode(value, out)?;
            }
        }
    }
    Ok(())
}

/// RFC 8949 deterministic CBOR for the authoritative Phase 1 subset.
/// Floats are rejected; null, booleans, integers, UTF-8 strings, arrays and maps are defined.
pub fn canonical_cbor(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    let mut out = Vec::new();
    encode(value, &mut out)?;
    Ok(out)
}
