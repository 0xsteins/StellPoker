use crate::types::*;
use soroban_sdk::{crypto, Address, Bytes, Env};

pub fn compute_action_hash(
    env: &Env,
    action: &Action,
    amount: i128,
    nonce: &Bytes,
) -> Bytes {
    let mut preimage = Vec::new(env);

    match action {
        Action::Fold => {
            preimage.push_back(0u8);
        }
        Action::Check => {
            preimage.push_back(1u8);
        }
        Action::Call => {
            preimage.push_back(2u8);
        }
        Action::Bet(_) => {
            preimage.push_back(3u8);
            let amount_bytes = amount.to_le_bytes();
            for byte in &amount_bytes {
                preimage.push_back(*byte);
            }
        }
        Action::Raise(_) => {
            preimage.push_back(4u8);
            let amount_bytes = amount.to_le_bytes();
            for byte in &amount_bytes {
                preimage.push_back(*byte);
            }
        }
        Action::AllIn => {
            preimage.push_back(5u8);
        }
    }

    for i in 0..nonce.len() {
        preimage.push_back(nonce.get(i).unwrap_or(0u8));
    }

    let preimage_bytes = Bytes::from_array(env, preimage.into_fixed_size().unwrap_or_else(|_| {
        let mut arr = [0u8; 256];
        for (i, v) in preimage.iter().enumerate().take(256) {
            arr[i] = v;
        }
        arr
    }));

    Bytes::from_array(env, crypto::keccak256(env, &preimage_bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_hash_fold() {
        let env = soroban_sdk::Env::default();
        let nonce = Bytes::from_slice(&env, &[1u8; 32]);
        let hash = compute_action_hash(&env, &Action::Fold, 0, &nonce);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_action_hash_consistency() {
        let env = soroban_sdk::Env::default();
        let nonce = Bytes::from_slice(&env, &[2u8; 32]);
        let hash1 = compute_action_hash(&env, &Action::Check, 0, &nonce);
        let hash2 = compute_action_hash(&env, &Action::Check, 0, &nonce);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_action_hash_different_actions() {
        let env = soroban_sdk::Env::default();
        let nonce = Bytes::from_slice(&env, &[3u8; 32]);
        let hash_fold = compute_action_hash(&env, &Action::Fold, 0, &nonce);
        let hash_check = compute_action_hash(&env, &Action::Check, 0, &nonce);
        assert_ne!(hash_fold, hash_check);
    }
}
