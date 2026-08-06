use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::{PromptRuntimeBundle, PromptRuntimeComponent, TargetPromptBinding};

const RUNTIME_HASH_DOMAIN: &[u8] = b"nauron.prompt-runtime.runtime.v2";
const COMPOSITE_HASH_DOMAIN: &[u8] = b"nauron.prompt-runtime.composite.v2";
const OPTIONAL_HASH_PRESENT_MARKER: &[u8] = b"some";
const OPTIONAL_HASH_ABSENT_MARKER: &[u8] = b"none";

pub fn sha256_hex(value: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(value.as_ref()))
}

pub fn calculate_runtime_hash(components: &[PromptRuntimeComponent]) -> String {
    let mut ordered = components.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        left.execution_order
            .cmp(&right.execution_order)
            .then_with(|| left.id.cmp(&right.id))
    });

    let mut encoder = HashEncoder::new(RUNTIME_HASH_DOMAIN);
    encoder.value(&(ordered.len() as u64).to_be_bytes());
    for component in ordered {
        encoder.value(component.id.as_bytes());
        encoder.value(component.stage.as_ref().as_bytes());
        encoder.value(component.role.as_ref().as_bytes());
        encoder.value(component.activation_condition.as_ref().as_bytes());
        encoder.value(&component.execution_order.to_be_bytes());
        encoder.value(component.content_hash.as_bytes());
    }
    encoder.finish()
}

pub fn calculate_composite_hash(
    runtime_bundle: &PromptRuntimeBundle,
    targets: &BTreeMap<String, TargetPromptBinding>,
) -> String {
    let mut encoder = HashEncoder::new(COMPOSITE_HASH_DOMAIN);
    encoder.value(runtime_bundle.id.as_bytes());
    encoder.value(&runtime_bundle.version.to_be_bytes());
    encoder.value(runtime_bundle.runtime_hash.as_bytes());
    encoder.value(&(targets.len() as u64).to_be_bytes());
    for (target_key, binding) in targets {
        encoder.value(target_key.as_bytes());
        encoder.value(binding.prompt_version_id.as_bytes());
        encoder.value(&binding.prompt_version.to_be_bytes());
        encoder.value(binding.prompt_hash.as_bytes());
        match &binding.source_bundle_hash {
            Some(hash) => {
                encoder.value(OPTIONAL_HASH_PRESENT_MARKER);
                encoder.value(hash.as_bytes());
            }
            None => encoder.value(OPTIONAL_HASH_ABSENT_MARKER),
        }
    }
    encoder.finish()
}

struct HashEncoder {
    hasher: Sha256,
}

impl HashEncoder {
    fn new(domain: &[u8]) -> Self {
        let mut encoder = Self {
            hasher: Sha256::new(),
        };
        encoder.value(domain);
        encoder
    }

    fn value(&mut self, value: &[u8]) {
        self.hasher.update((value.len() as u64).to_be_bytes());
        self.hasher.update(value);
    }

    fn finish(self) -> String {
        hex::encode(self.hasher.finalize())
    }
}
