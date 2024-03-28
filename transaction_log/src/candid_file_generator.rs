use crate::cycles::WalletReceiveResult;
use crate::init_method::InitArgs;
use crate::types::*;
use candid::export_service;
use candid::Principal;
use ic_cdk_macros::query;

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("transaction_log.did"), export_candid()).expect("Write failed.");
    }
}
