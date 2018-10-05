#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate indy;
#[allow(unused_variables)]
#[allow(unused_macros)]
#[allow(dead_code)]
#[macro_use]
pub mod utils;

use indy::did::Did;
use indy::ErrorCode;
use indy::ledger::Ledger;
use indy::pool::Pool;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::constants::{INVALID_TIMEOUT, PROTOCOL_VERSION, VALID_TIMEOUT};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

const REQUEST_JSON: &str = r#"{
                              "reqId":1496822211362017764,
                              "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                              "operation":{
                                   "type":"1",
                                   "dest":"VsKV7grR1BUE29mG2Fm2kX",
                                   "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                                   },
                              "protocolVersion":2
                          }"#;
#[cfg(test)]
mod test_sign_and_submit_request {

    use super::*;

    #[test]
    pub fn sign_and_submit_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did, REQUEST_JSON);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => { },
            Err(ec) => { assert!(false, "sign_and_submit_request_success got error code {:?}", ec); },
        }


        /*
         * The format of SignAndSubmitRequestAsync response is like this.
         *
            {"result":{
                "reqSignature":{
                    "type":"ED25519",
                    "values":[{"value":"7kDrVBrmrKAvSs1QoQWYq6F774ZN3bRXx5e3aaUFiNvmh4F1yNqQw1951Az35nfrnGjZ99vtCmSFXZ5GqS1zLiG","from":"V4SGRU86Z58d6TV7PBUe6f"}]
                },
                "txnMetadata":{
                    "txnTime":1536876204,
                    "seqNo":36,
                    "txnId":"5d38ac6a242239c97ee28884c2b5cadec62248b2256bce51afd814c7847a853e"
                },
                "ver":"1",
                "auditPath":["DATtzSu9AMrArv8C2oribQh4wJ6TaD2K9o76t7EL2N7G","AbGuM7s9MudnT8M2eZe1yaG2EGUGxggMXSSbXCm4DFDx","3fjMoUdsbNrRfG5ZneHaQuX994oA4Z2pYPZtRRPmkngw"],
                "rootHash":"A9LirjLuoBT59JJTJYvUgfQyEJA32Wb7njrbD9XqT2wc",
                "txn":{
                    "data":{
                        "dest":"KQRpY4EmSG4MwH7md8gMoN","verkey":"B2nW4JfqZ2omHksoCmwD8zXXmtBsvbQk6WVSboazd8QB"
                    },
                    "protocolVersion":2,
                    "type":"1",
                    "metadata":{
                        "digest":"14594e0b31f751faf72d4bf4abdc6f54af34dab855fe1a0c67fe651b47bb93b5","reqId":1536876205519496000,"from":"V4SGRU86Z58d6TV7PBUe6f"
                    }
                }
            },
            "op":"REPLY"}
        */
    }

    #[test]
    pub fn sign_and_submit_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        Ledger::sign_and_submit_request_async(pool_handle, wallet.handle, &did, REQUEST_JSON, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success);
    }

    #[test]
    pub fn sign_and_submit_request_timeout_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, VALID_TIMEOUT);
        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {  },
            Err(ec) => { assert!(false, "sign_and_submit_request_timeout_success got error code {:?}", ec); },
        }


    }

    #[test]
    pub fn sign_and_submit_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();;
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, INVALID_TIMEOUT);
        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {
                assert!(false, "sign_and_submit_request_timeout DID NOT time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "sign_and_submit_request_timeout error code didn't match expected => {:?}", ec);
            },
        }
    }

}

#[cfg(test)]
mod test_submit_request {
    use super::*;

    #[test]
    pub fn submit_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = Ledger::submit_request(pool_handle, REQUEST_JSON);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(submit_request_response) => {
                // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
                // and getting that response back indicates success
                assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
                assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
            },
            Err(ec) => {
                assert!(false, "submit_request failed with {:?}", ec);
            }
        }

    }

    #[test]
    pub fn submit_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        indy::ledger::Ledger::submit_request_async(pool_handle, REQUEST_JSON, cb);

        let (ec, submit_request_response) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "submit_request did not return ErrorCode::Success => {:?}", ec);

        // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
        // and getting that response back indicates success
        assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
        assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
    }

    #[test]
    pub fn submit_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = indy::ledger::Ledger::submit_request_timeout(pool_handle, REQUEST_JSON, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(submit_request_response) => {
                // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
                // and getting that response back indicates success
                assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
                assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
            },
            Err(ec) => {
                assert!(false, "submit_request failed with {:?}", ec);
            }
        }
    }

    #[test]
    pub fn submit_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = indy::ledger::Ledger::submit_request_timeout(pool_handle, REQUEST_JSON, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(_) => {
                assert!(false, "submit_request_timeout DID NOT time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "submit_request_timeout error code didn't match expected => {:?}", ec);
            },
        }
    }

}

#[cfg(test)]
mod test_submit_action {
    use super::*;

    const NODES : &str = "[\"Node1\", \"Node2\"]";

    // This test needs to be researched as a possible bug in libindy.  No errors are returned, it hangs forever,
    // ignoring the timeout
    #[test]
    #[ignore] // TODO: restore after IS-1027 will be fixed
    pub fn submit_action_this_hangs_indefinitely() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request = Ledger::sign_request(wallet.handle, &did, &validator_request).unwrap();

        let result = Ledger::submit_action(pool_handle, &signed_request, "[]", 5);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_success failed with {:?} extra {:?}", ec, signed_request);
            }
        }
    }

    #[test]
    pub fn submit_action_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request = Ledger::sign_request(wallet.handle, &did, &validator_request).unwrap();

        let result = Ledger::submit_action(pool_handle, &signed_request, NODES, 5);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_success failed with {:?} extra {:?}", ec, signed_request);
            }
        }
    }

    #[test]
    pub fn submit_action_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        Ledger::submit_action_async(pool_handle, &validator_request, NODES, 5, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "submit_action_async failed error_code {:?}", ec);
    }

    #[test]
    pub fn submit_action_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let result = Ledger::submit_action_timeout(pool_handle, &validator_request, NODES, 5, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_timeout failed with {:?} extra {:?}", ec, validator_request);
            }
        }
    }

    #[test]
    pub fn submit_action_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let result = Ledger::submit_action_timeout(pool_handle, &validator_request, NODES, 5, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {
                assert!(false, "submit_action_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "submit_action_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_sign_request {
    use super::*;

    #[test]
    pub fn sign_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request(wallet.handle, &did, &validator_request);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "sign_request returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn sign_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let signed_request_result = Ledger::sign_request_async(wallet.handle, &did, &validator_request, cb);

        assert_eq!(signed_request_result, ErrorCode::Success, "sign_request_async failed error_code {:?}", signed_request_result);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "sign_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn sign_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request_timeout(wallet.handle, &did, &validator_request, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "sign_request_timeout returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn sign_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request_timeout(wallet.handle, &did, &validator_request, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {
                assert!(false, "sign_request_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "sign_request_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_multi_sign_request {
    use super::*;

    #[test]
    pub fn multi_sign_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request(wallet.handle, &did, &validator_request);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "multi_sign_request returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn multi_sign_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let _ = Ledger::multi_sign_request_async(wallet.handle, &did, &validator_request, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "sign_request_async failed error_code {:?}", ec);
    }

    #[test]
    pub fn multi_sign_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request_timeout(wallet.handle, &did, &validator_request, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "multi_sign_request_timeout returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn multi_sign_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request_timeout(wallet.handle, &did, &validator_request, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {
                assert!(false, "multi_sign_request_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "multi_sign_request_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_nym_request {
    use super::*;

    use utils::did::NymRole;

    #[test]
    pub fn build_nym_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }

    }

    #[test]
    pub fn build_nym_request_with_no_verkey_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, None, None, NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }

    }

    #[test]
    pub fn build_nym_request_with_data_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, None, Some("some_data"), NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_nym_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let async_error_code = Ledger::build_nym_request_async(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), cb);

        assert_eq!(async_error_code, ErrorCode::Success, "build_nym_request_async failed error_code {:?}", async_error_code);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        assert_eq!(ec, ErrorCode::Success, "build_nym_request_async returned error_code {:?}", ec);

    }

    #[test]
    pub fn build_nym_request_timeout_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request_timeout(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), VALID_TIMEOUT);

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_nym_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request_timeout(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), INVALID_TIMEOUT);

        match nym_result {
            Ok(_) => {
                assert!(false, "build_nym_request_timeout did not time out as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_nym_request {
    use super::*;

    #[test]
    pub fn build_get_nym_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request(Some(&submitter_did), &did);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_no_submitter_did_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request(None, &did);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let get_async_result = Ledger::build_get_nym_request_async(Some(&submitter_did), &did, cb);

        assert_eq!(get_async_result, ErrorCode::Success, "build_get_nym_request_async failed {:?}", get_async_result);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_nym_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn build_get_nym_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request_timeout(Some(&submitter_did), &did, VALID_TIMEOUT);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request_timeout(Some(&submitter_did), &did, INVALID_TIMEOUT);

        match get_result {
            Ok(_) => {
                assert!(false, "build_get_nym_request_timeout DID NOT time out as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_attrib_request {
    use super::*;

    #[test]
    pub fn build_attrib_request_success() {

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_attrib_request(&submitter_did, &did, None, Some("{}"), None) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_attrib_request failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_attrib_request_async_success() {
        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let request_error_code =  Ledger::build_attrib_request_async(&submitter_did, &did, None, Some("{}"), None, cb);

        assert_eq!(request_error_code, ErrorCode::Success, "build_attrib_request_async returned {:?}", request_error_code);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_attrib_request_async returned error_code {:?}", ec);

    }

    #[test]
    pub fn build_attrib_request_timeout_success() {

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_attrib_request_timeout(&submitter_did, &did, None, Some("{}"), None, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_attrib_request_timeout failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_attrib_request_timeout_times_out() {
        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_attrib_request_timeout(&submitter_did, &did, None, Some("{}"), None, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_attrib_request_timeout did not time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_attrib_request_timeout failed with error {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_attrib_request {

    use super::*;

    #[test]
    pub fn build_get_attrib_request_success() {

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_get_attrib_request(Some(&submitter_did), &did, Some("{}"), None, None) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_attrib_request failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_attrib_request_async_success() {
        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let request_error_code =  Ledger::build_get_attrib_request_async(Some(&submitter_did), &did, Some("{}"), None, None, cb);

        assert_eq!(request_error_code, ErrorCode::Success, "build_get_attrib_request_async returned {:?}", request_error_code);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_attrib_request_async returned error_code {:?}", ec);

    }

    #[test]
    pub fn build_get_attrib_request_timeout_success() {

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_get_attrib_request_timeout(Some(&submitter_did), &did, Some("{}"), None, None, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_attrib_request_timeout failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_attrib_request_timeout_times_out() {
        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        match Ledger::build_get_attrib_request_timeout(Some(&submitter_did), &did, Some("{}"), None, None, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_attrib_request_timeout did not time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_attrib_request_timeout failed with error {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_schema_request {
    use super::*;

    const SCHEMA_DATA : &str = r#"{"id":"id","attrNames": ["name", "male"],"name":"gvt2","version":"3.1","ver":"1.0"}"#;

    #[test]
    pub fn build_schema_request_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_schema_request(&did, SCHEMA_DATA) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_schema_request failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_schema_request_async_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let async_ec = Ledger::build_schema_request_async(&did, SCHEMA_DATA, cb);
        assert_eq!(async_ec, ErrorCode::Success, "build_schema_request_async returned {:?}", async_ec);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_schema_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn build_schema_request_timeout_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_schema_request_timeout(&did, SCHEMA_DATA, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_schema_request_timeout failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_schema_request_timeout_times_out() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_schema_request_timeout(&did, SCHEMA_DATA, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_schema_request_timeout failed to TIME OUT");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_schema_request_timeout failed with error {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_schema_request {
use super::*;

    const SCHEMA_REQUEST : &str = "5LEV4bTAXntXqmtLFm7yCS:2:bob:1.0";

    #[test]
    pub fn build_get_schema_request_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();


        match Ledger::build_get_schema_request(Some(&did), SCHEMA_REQUEST) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_schema_request failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_schema_request_async_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let async_ec = Ledger::build_get_schema_request_async(Some(&did), SCHEMA_REQUEST, cb);
        assert_eq!(async_ec, ErrorCode::Success, "build_get_schema_request_async returned {:?}", async_ec);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_schema_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn build_get_schema_request_timeout_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_schema_request_timeout(Some(&did), SCHEMA_REQUEST, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_schema_request_timeout failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_schema_request_timeout_times_out() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_schema_request_timeout(Some(&did), SCHEMA_REQUEST, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_get_schema_request_timeout failed to TIME OUT");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_schema_request_timeout failed with error {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_parse_get_schema_response {

    use super::*;

    const SCHEMA_ID : &str = "schema_id1234";
    const SCHEMA_NAME : &str = "schema_1234";
    const SCHEMA_DATA : &str = r#"{"id":"schema_id1234","attrNames": ["name", "male"],"name":"schema_1234","version":"1.0","ver":"1.0"}"#;


    fn create_build_schema_request(did : &String) -> String {
        format!("{}:2:{}:1.0", did, SCHEMA_NAME)
    }

    fn build_schema(did: &String, pool_handle: i32) {
        let build_schema = Ledger::build_schema_request(&did, SCHEMA_DATA).unwrap();
        let submit_response = Ledger::submit_request(pool_handle, &build_schema).unwrap();
    }

    #[test]
    pub fn parse_get_schema_response_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        build_schema(&did, pool_handle);
        let schema_request = create_build_schema_request(&did);

        let schema_response = Ledger::build_get_schema_request(Some(&did), &schema_request).unwrap();
        let signed_response = Ledger::sign_request(wallet.handle, &did,&schema_response).unwrap();
        let submit_response = Ledger::submit_request(pool_handle, &signed_response).unwrap();

        let parse_response = Ledger::parse_get_schema_response(&submit_response);

        Pool::close(pool_handle).unwrap();

        match parse_response {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "parse_get_schema_response failed error_code {:?} \n\n using submit_response {:?} \n\n with signed_response {:?} \n\n from schema_response {:?} \n\n schema {:?}", ec, submit_response, signed_response, schema_response, schema_request);
            }
        }
    }

    #[test]
    pub fn parse_get_schema_response_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        build_schema(&did, pool_handle);
        let schema_request = create_build_schema_request(&did);

        let schema_response = Ledger::build_get_schema_request(Some(&did), &schema_request).unwrap();
        let signed_response = Ledger::sign_request(wallet.handle, &did,&schema_response).unwrap();
        let submit_response = Ledger::submit_request(pool_handle, &signed_response).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, s1, s2| {
            sender.send((ec, s1, s2)).unwrap();
        };

        let async_ec = Ledger::parse_get_schema_response_async(&submit_response, cb);

        let (ec, _, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        Pool::close(pool_handle).unwrap();

        assert_eq!(async_ec, ErrorCode::Success, "parse_get_schema_response_async returned {:?}", async_ec);
        assert_eq!(ec, ErrorCode::Success, "parse_get_schema_response_async failed error_code {:?}", ec);
    }

    #[test]
    pub fn parse_get_schema_response_timeout_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let response = Ledger::build_get_schema_request(Some(&did), SCHEMA_DATA).unwrap();

        match Ledger::parse_get_schema_response_timeout(&response, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "parse_get_schema_response_timeout failed with error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn parse_get_schema_response_timeout_times_out() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let response = Ledger::build_get_schema_request(Some(&did), SCHEMA_DATA).unwrap();

        match Ledger::parse_get_schema_response_timeout(&response, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "parse_get_schema_response_timeout failed to TIME OUT");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "parse_get_schema_response_timeout failed with error {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_ddo_request {

    use super::*;

    #[test]
    pub fn build_get_ddo_request_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_ddo_request(Some(&did), &did) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_ddo_request failed error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_ddo_request_async_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let async_ec = Ledger::build_get_ddo_request_async(Some(&did), &did, cb);
        assert_eq!(async_ec, ErrorCode::Success, "build_get_ddo_request_async returned {:?}", async_ec);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_ddo_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn build_get_ddo_request_timeout_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_ddo_request_timeout(Some(&did), &did, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_ddo_request_timeout failed error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_ddo_request_timeout_times_out() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_ddo_request_timeout(Some(&did), &did, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_get_ddo_request_timeout failed to timeout");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_ddo_request_timeout failed error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_txn_request {
    use super::*;

    const LEDGER_TYPE : &str = "DOMAIN";

    #[test]
    pub fn build_get_txn_request_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_txn_request(Some(&did), Some(LEDGER_TYPE), 1) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_txn_request failed error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_txn_request_async_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let async_ec =  Ledger::build_get_txn_request_async(Some(&did), Some(LEDGER_TYPE), 1, cb);

        assert_eq!(async_ec, ErrorCode::Success, "build_get_txn_request_async return error_code {:?}", async_ec);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_txn_request_async returned error_code {:?}", ec);

    }

    #[test]
    pub fn build_get_txn_request_timeout_success() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_txn_request_timeout(Some(&did), Some(LEDGER_TYPE), 1, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_txn_request_timeout failed error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_txn_request_timeout_times_out() {
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_get_txn_request_timeout(Some(&did), Some(LEDGER_TYPE), 1, INVALID_TIMEOUT) {
            Ok(_) => {
                assert!(false, "build_get_txn_request_timeout failed to timeout");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_txn_request_timeout failed error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_cred_def_request {
    use super::*;

    const CRED_DATA : &str = r#"{"ver":"1.0","id":"V4SGRU86Z58d6TV7PBUe6f:3:CL:17:oI6Iokv","schemaId":"17","type":"CL","tag":"oI6Iokv","value":{"primary":{"n":"87178281071734731437690387382922138711010162107879888888538848407132327215161696479014638246148780516059076502007170233816521638866237445955186196199106181664196333035350522256775772678749757516076687671733088043974750225859037634391059057871128952528114293385763258675546471992534732373945591487042489023109902330242980705545998552851661474245748466697559479508930710234503328250288511766352977719334252928597855882930620741923986586828547412389638821815758450532251881301327049927072714545264108360496728434366393519356711418047944068773770531352206244052618670493983767902201878934735288733850555036281883721724473","s":"66794590351311400173440340223561508784777853797981871904981559245334503567545616382611784848902543717386870008558289905316601662574754089209687052710438230437549560386636286514822680768065835610624750399055088116166226098175424830519537908801592274839622946402090491946787040058370552124732885892610142242847158959492000732292603755213902976259446731410240912872744210451254301242220759673686769861789834996124153811460984114732824978048021325148492655982695079333718710090836034672739682282204904856516947015563681443657793597047393731812247221167838278986153621564706820976058621996938916226023920421313258317181056","r":{"height":"37686658568948037814775431903843597441856100114754323955796133079330648476309192012260294209465266635551131504125646637359931844595436529982289207908218765877672222632310887737940054188921134584912244256324727048869497937750475441196124576035922245172355820888415660512858847440533214955712359488689065763607483137995713620001810321655685884305156101062519673602853819411046367019986397235673847881046529391589711229735614071805410066894389088951657447726215788267372471488185033424222037788505918934857840957649277458736101301203881379280675945440723899652144116975079241713669490809165909240425120649887001447597783","sex":"48901017446440182649799731593114947230876351500273905015595989118217119375071111218399500737051508041416910144890371937193478691958771514378058820388069120931504416289663010431000145369715463131882383259114210820041731832960312557792681552574471886139487662967612807830334835729846093444859302732007584479807271091676491277902271511164922767210557187133481955551837663018165430244652992048757580783775433571336475692686259720997931194126203237043117966211161878071070916577025579669942228991696500136399569383974985399786080235342264485395522119939857486145401501612186163491615961653478438596959371113747671419654818","master_secret":"25754344723836699563584283879786692153622691083042382144160949511089528018631287834606498465418239311334501386316618629687258527283908520406207259178795217648481719864528515688115872808761112818709464686844054961387398147908732686218740513751960844653382166618983380191016571483892249629309506399346596975262589381752411984820505602091163687287542251803844421163362364666975191601496467090517324300542321861313152630025880504086070664031524805153571288074723002683472372414034607393588926109015678216745625826790077479058525170476570603845174669212586627449339894597259739762350550126584394404404482135882343197379054","name":"64945144723199018124037264140277156942131666148001245998219662472757345342279533884405888431954009830746588251472121029944168008438815350643138701794229741155411122621890661138856631059298571458398258239896113210276837384872922411226059610000961503919325158321529528085516642820682380678880886510720705463915144189095545183388444662260696183777535832602518582169729325489244691039221691384084009024188000745680035168934609700642727769603625029893488551202843527601643121220149122355960460523112480070939364242895718918315978456710031746858656148388609050488969420517950113219916527876709626082332309036117494497571230","age":"32059832863983999153613274260157019698296212529496396734792926477728751870868732126531732944880026440901943732956875433636855727848522486073745001899386738358016223503298068118020520201025961660332519845724521320552467451946852925824035412812595344753770029091958417300642692814810865758776286263929051571009155820474717897179825570107678882180230319004359558595714472285100325554837250772201405847343231203624749918461130474867722155978675551762505078508248381791048831193422832357874770535478244636601382323151375446307087219224928662366021820679699538198192887109930714869161019271417169222414370495648047714662103"},"rctxt":"38527464755916130963069611882293312815641859720607042775748742527688895624917359948168832828223678535804570958646474457323858571801037955359525462917173252086033591899208879285429574561167189107147758287137082689831331351781369164690717667586513821072095969666206581907243540078811132148136508600170388059098530570400824663936962403223100603326027117401899329035603739144303108339956544437073624926208818126402005595194344120188160525632489014089138283290414616529527375527577666875823786710497945303252443476610222721607664991987281949777517734685590949741562190122640202895612444667451959072089271004850428610427341","z":"31160349078984317779569363785252606468286101126154161634595102825752576352018565401209247600497171866986884547654707390053445672860929599916189762737605262398652714436350679825010487409345252016639698884761154432723648619393558760904141612222413004613912305317054390982133492273484244661652402423836430130022761985374095739624351663212686292616011934960947889676162946869205272435766196882679460333258355511812361345778943009086137697548566706243827603133668933678120765799991107515465495261132740007129958253450651703438567276912235691326876396719017894799034243480316169679472944643292666194979984921170821328746189"}}}"#;

    #[test]
    pub fn test_build_cred_def_request_success(){

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_cred_def_request(&did, &CRED_DATA) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_cred_def_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn test_build_cred_def_request_async_success(){
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let async_ec = Ledger::build_cred_def_request_async(&did, &CRED_DATA, cb);
        assert_eq!(async_ec, ErrorCode::Success, "build_cred_def_request_async returned error_code {:?}", async_ec);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_cred_def_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn test_build_cred_def_request_timeout_success(){
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_cred_def_request_timeout(&did, &CRED_DATA, VALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_cred_def_request_timeout returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn test_build_cred_def_request_timeout_times_out(){
        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        match Ledger::build_cred_def_request_timeout(&did, &CRED_DATA, INVALID_TIMEOUT) {
            Ok(_) => {},
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_cred_def_request_timeout returned error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_cred_def_request {

}

#[cfg(test)]
mod test_build_node_request {

}

#[cfg(test)]
mod test_build_get_validator_info_request {

}

#[cfg(test)]
mod test_build_pool_config_request {

}

#[cfg(test)]
mod test_build_pool_restart_request {

}

#[cfg(test)]
mod test_build_pool_upgrade_request {

}

#[cfg(test)]
mod test_build_revoc_reg_def_request {

}

#[cfg(test)]
mod test_build_get_revoc_reg_def_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_def_response {

}

#[cfg(test)]
mod test_build_revoc_reg_entry_request {
}

#[cfg(test)]
mod test_build_get_revoc_reg_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_response {

}

#[cfg(test)]
mod test_build_get_revoc_reg_delta_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_delta_response {

}

#[cfg(test)]
mod test_register_transaction_parser_for_sp {

}
