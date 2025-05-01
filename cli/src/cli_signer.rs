use solana_remote_wallet::remote_keypair::RemoteKeypair;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature, Signer, SignerError},
};

pub struct CliSigner {
    pub keypair: Option<Keypair>,
    pub remote_keypair: Option<RemoteKeypair>,
}

impl CliSigner {
    pub fn new(keypair: Option<Keypair>, remote_keypair: Option<RemoteKeypair>) -> Self {
        if keypair.is_none() && remote_keypair.is_none() {
            panic!("No keypair or wallet manager provided");
        }

        Self {
            keypair,
            remote_keypair,
        }
    }

    pub fn new_keypair(keypair: Keypair) -> Self {
        Self::new(Some(keypair), None)
    }

    pub fn new_keypair_from_path(keypair_path: &str) -> Self {
        Self::new(
            Some(read_keypair_file(keypair_path).expect("No keypair found")),
            None,
        )
    }
}

impl Signer for CliSigner {
    fn try_pubkey(&self) -> Result<Pubkey, SignerError> {
        self.keypair.as_ref().map_or_else(
            || {
                self.remote_keypair
                    .as_ref()
                    .map_or(Err(SignerError::NoDeviceFound), |remote_keypair| {
                        Ok(remote_keypair.pubkey)
                    })
            },
            |keypair| Ok(keypair.pubkey()),
        )
    }

    fn try_sign_message(&self, message: &[u8]) -> Result<Signature, SignerError> {
        self.keypair.as_ref().map_or_else(
            || {
                self.remote_keypair
                    .as_ref()
                    .map_or(Err(SignerError::NoDeviceFound), |remote_keypair| {
                        remote_keypair.try_sign_message(message)
                    })
            },
            |keypair| keypair.try_sign_message(message),
        )
    }

    fn is_interactive(&self) -> bool {
        // Remote wallets are typically interactive, local keypairs are not
        self.remote_keypair.is_some()
    }
}
