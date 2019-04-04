#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::{wallet, crypto, did};
use utils::constants::*;

use self::indy::ErrorCode;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11];

fn setup_with_key() -> (i32, String) {
    let wallet_handle = utils::setup_with_wallet();
    let verkey = crypto::create_key(wallet_handle, None).unwrap();
    (wallet_handle, verkey)
}

mod high_cases {
    use super::*;

    mod create_key {
        use super::*;
        use rust_base58::FromBase58;

        #[test]
        fn indy_create_key_works_for_seed() {
            let wallet_handle = utils::setup_with_wallet();

            let verkey = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_create_key_works_without_seed() {
            let wallet_handle = utils::setup_with_wallet();

            let verkey = crypto::create_key(wallet_handle, None).unwrap();
            assert_eq!(verkey.from_base58().unwrap().len(), 32);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_create_key_works_for_invalid_wallet_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::create_key(wallet_handle + 1, None);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod set_key_metadata {
        use super::*;

        #[test]
        fn indy_set_key_metadata_works() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_replace() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();
            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            let new_metadata = "updated metadata";
            crypto::set_key_metadata(wallet_handle, &verkey, new_metadata).unwrap();
            let updated_metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(new_metadata, updated_metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::set_key_metadata(wallet_handle + 1, &verkey, METADATA);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_set_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }


        #[test]
        fn indy_set_key_metadata_works_for_invalid_key() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::set_key_metadata(wallet_handle, INVALID_BASE58_VERKEY, METADATA);
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod get_key_metadata {
        use super::*;

        #[test]
        fn indy_get_key_metadata_works() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!(METADATA.to_string(), metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_empty_string() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, "").unwrap();

            let metadata = crypto::get_key_metadata(wallet_handle, &verkey).unwrap();
            assert_eq!("", metadata);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_no_metadata() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::get_key_metadata(wallet_handle, &verkey);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_key_metadata_works_for_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::set_key_metadata(wallet_handle, &verkey, METADATA).unwrap();

            let res = crypto::get_key_metadata(wallet_handle + 1, &verkey);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod crypto_sign {
        use super::*;

        #[test]
        fn indy_crypto_sign_works() {
            let wallet_handle = utils::setup_with_wallet();

            let my_vk = crypto::create_key(wallet_handle, Some(MY1_SEED)).unwrap();

            let signature = crypto::sign(wallet_handle, &my_vk, MESSAGE.as_bytes()).unwrap();
            assert_eq!(SIGNATURE.to_vec(), signature);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_sign_works_for_unknown_signer() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::sign(wallet_handle, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_sign_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::sign(wallet_handle + 1, &verkey, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod crypto_verify {
        use super::*;

        #[test]
        fn indy_crypto_verify_works() {
            let valid = crypto::verify(&VERKEY_MY1, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_correct_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":ed25519";
            let valid = crypto::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_verkey_with_invalid_crypto_type() {
            let verkey = VERKEY_MY1.to_owned() + ":unknown_crypto";
            let res = crypto::verify(&verkey, MESSAGE.as_bytes(), SIGNATURE);
            assert_code!(ErrorCode::UnknownCryptoTypeError, res);
        }


        #[test]
        fn indy_crypto_verify_works_for_other_signer() {
            let valid = crypto::verify(&VERKEY_MY2, MESSAGE.as_bytes(), SIGNATURE).unwrap();
            assert!(!valid);
        }

        #[test]
        fn indy_crypto_verify_works_for_invalid_signature_len() {
            let signature: Vec<u8> = vec![20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190];
            let res = crypto::verify(&VERKEY_MY1, MESSAGE.as_bytes(), &signature);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }
    }

    mod auth_crypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_key() {
            let (wallet_handle, verkey) = setup_with_key();

            crypto::auth_crypt(wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did() {
            let sender_wallet_handle = utils::setup_with_wallet();

            let (_, verkey) = did::create_and_store_my_did(sender_wallet_handle, Some(MY1_SEED)).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_created_did_as_cid() {
            let sender_wallet_handle = utils::setup_with_wallet();

            let (_, verkey) = did::create_my_did(sender_wallet_handle, &json!({ "seed": MY1_SEED, "cid": true }).to_string()).unwrap();

            crypto::auth_crypt(sender_wallet_handle, &verkey, VERKEY_MY2, MESSAGE.as_bytes()).unwrap();

            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_unknown_sender_verkey() {
            let wallet_handle = utils::setup_with_wallet();

            let res = crypto::auth_crypt(wallet_handle, VERKEY_MY2, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_wallet_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::auth_crypt(wallet_handle + 1, &verkey, VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_crypt_works_for_invalid_recipient_vk() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::auth_crypt(wallet_handle, &verkey, INVALID_BASE58_VERKEY, MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod auth_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_auth_decrypt_works() {
            let (sender_wallet_handle, sender_vk) = setup_with_key();

            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let (vk, msg) = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);
            assert_eq!(sender_vk, vk);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_invalid_msg() {
            let sender_wallet_handle = utils::setup_with_wallet();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();

            let (recipient_did, recipient_vk) = did::create_and_store_my_did(recipient_wallet_handle, Some(MY2_SEED)).unwrap();
            did::store_their_did_from_parts(sender_wallet_handle, &recipient_did, &recipient_vk).unwrap();

            let encrypted_msg = format!(r#"{{"nonce":"Th7MpTaRZVRYnPiabds81Y12","sender":"{:?}","msg":"{:?}"}}"#, VERKEY, ENCRYPTED_MESSAGE.to_vec());

            let res = crypto::auth_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_for_unknown_recipient_vk() {
            let (wallet_handle, sender_vk) = setup_with_key();

            let encrypted_msg = crypto::auth_crypt(wallet_handle, &sender_vk, &VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_auth_decrypt_works_invalid_handle() {
            let (sender_wallet_handle, sender_vk) = setup_with_key();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::auth_crypt(sender_wallet_handle, &sender_vk, &recipient_vk, MESSAGE.as_bytes()).unwrap();

            let res = crypto::auth_decrypt(recipient_wallet_handle + 1, &recipient_vk, &encrypted_msg);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }
    }

    mod anon_crypt {
        use super::*;

        #[test]
        fn indy_anon_crypt_works() {
            utils::setup();

            crypto::anon_crypt(VERKEY_MY2, &MESSAGE.as_bytes()).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_anon_crypt_works_for_invalid_their_vk() {
            utils::setup();

            let res = crypto::anon_crypt(INVALID_VERKEY_LENGTH, &MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            let res = crypto::anon_crypt(INVALID_BASE58_VERKEY, &MESSAGE.as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down();
        }
    }

    mod anon_decrypt {
        use super::*;

        #[test]
        fn indy_crypto_anon_decrypt_works() {
            let (sender_wallet_handle, _) = setup_with_key();
            let recipient_wallet_handle = wallet::create_and_open_default_wallet().unwrap();
            let recipient_vk = crypto::create_key(recipient_wallet_handle, None).unwrap();

            let encrypted_msg = crypto::anon_crypt(&recipient_vk, MESSAGE.as_bytes()).unwrap();

            let msg = crypto::anon_decrypt(recipient_wallet_handle, &recipient_vk, &encrypted_msg).unwrap();
            assert_eq!(MESSAGE.as_bytes().to_vec(), msg);

            wallet::close_wallet(recipient_wallet_handle).unwrap();
            utils::tear_down_with_wallet(sender_wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_invalid_msg() {
            let (wallet_handle, verkey) = setup_with_key();

            let res = crypto::anon_decrypt(wallet_handle, &verkey, &"unencrypted message".as_bytes());
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_for_unknown_verkey() {
            let wallet_handle = utils::setup_with_wallet();

            let encrypted_msg = crypto::anon_crypt(&VERKEY_TRUSTEE, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle, &VERKEY_TRUSTEE, &encrypted_msg);
            assert_code!(ErrorCode::WalletItemNotFound, res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_anon_decrypt_works_invalid_handle() {
            let (wallet_handle, verkey) = setup_with_key();

            let encrypted_msg = crypto::anon_crypt(&verkey, MESSAGE.as_bytes()).unwrap();

            let res = crypto::anon_decrypt(wallet_handle + 1, &verkey, &encrypted_msg);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod pack_message_authcrypt {
        use super::*;
        use serde_json::Value;

        #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
        pub struct Forward1 {
            #[serde(rename = "@type")]
            msg_type: String,
            #[serde(rename = "@fwd")]
            fwd: String,
            #[serde(rename = "@msg")]
            msg: Vec<u8>,
        }

        #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
        pub struct Forward2 {
            #[serde(rename = "@type")]
            msg_type: String,
            #[serde(rename = "@fwd")]
            fwd: String,
            #[serde(rename = "@msg")]
            msg: String,
        }

        #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
        pub struct UnpackMessage {
            pub message: String,
            pub recipient_verkey: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sender_verkey: Option<String>
        }

        fn extract_forward_msg(msg: &[u8]) -> Vec<u8> {
            let (M9_1, cts3) = crypto::remove_cts_from_msg(msg).unwrap();
            let mut M9_2_json: Value = serde_json::from_slice(&M9_1).unwrap();
            let M9_2_obj = M9_2_json.as_object_mut().unwrap();
            let M9_3_msg = M9_2_obj.remove("@msg").unwrap();
            let M9_3_str = str::replace(&M9_3_msg.to_string(), "\\", "");
            let M9_4_str = &M9_3_str.trim_matches('"');
            let M9_3_json: Value = serde_json::from_str(&M9_4_str).unwrap();
            let M9_4 = serde_json::to_vec(&M9_3_json).unwrap();
            crypto::add_cts_to_msg(&M9_4, &cts3).unwrap()
        }

        #[test]
        fn indy_pack_message_authcrypt_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert!(res.is_ok());
            let x = res.unwrap();
            println!("x_len={:?}", x.len());
            println!("x={:?}", &x);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_authcrypt_repeated_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys_1 = serde_json::to_string(&rec_key_vec[..1]).unwrap();
            let receiver_keys_2 = serde_json::to_string(&rec_key_vec[1..2]).unwrap();
            let receiver_keys_3 = serde_json::to_string(&rec_key_vec[2..3]).unwrap();
            let m0 = "Hello World".as_bytes().to_vec();
            let mut m00 = vec![];
            for _ in 0..10000 {
                m00.extend_from_slice(&m0);
            }
            let message = &m00;
            let res_1 = crypto::pack_message(wallet_handle, message, &receiver_keys_1, Some(&verkey));
            assert!(res_1.is_ok());
            let m2 = res_1.unwrap();
            let res_2 = crypto::pack_message(wallet_handle, &m2, &receiver_keys_2, Some(&verkey));
            assert!(res_2.is_ok());
            let m3 = res_2.unwrap();
            let res_3 = crypto::pack_message(wallet_handle, &m3, &receiver_keys_3, Some(&verkey));
            assert!(res_3.is_ok());
            let m4 = res_3.unwrap();
            println!("message={}", message.len());
            println!("m2={}", m2.len());
            println!("m3={}", m3.len());
            println!("m4={}", m4.len());
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_pack_fwd_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys_1 = serde_json::to_string(&rec_key_vec[..1]).unwrap();
            let res_1 = crypto::pack_message(wallet_handle, MESSAGE.as_bytes(), &receiver_keys_1, Some(&verkey)).unwrap();
            println!("res_1_len={:?}", res_1.len());
            //println!("res_1={:?}", &res_1);
            let y = serde_json::to_string(&res_1).unwrap();
            println!("y_len={:?}", y.len());
            //println!("y={:?}", &y);

            let fwd1 = Forward1 {
                msg_type: "f".to_string(),
                fwd: "a".to_string(),
                msg: res_1.clone()
            };
            let f1 = serde_json::to_string(&fwd1).unwrap();
            //println!("f1_len={:?}", f1.len());
            //println!("f1={:?}", &f1);

            let fwd2 = Forward2 {
                msg_type: "f".to_string(),
                fwd: "a".to_string(),
                msg: String::from_utf8(res_1).unwrap()
            };
            let f2 = serde_json::to_string(&fwd2).unwrap();
            //println!("f2_len={:?}", f2.len());
            //println!("f2={:?}", &f2);


            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_collapse_ciphertext_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let verkey_1 = crypto::create_key(wallet_handle, None).unwrap();
            let receiver_key = serde_json::to_string(&vec![verkey_1]).unwrap();
            let res_1 = crypto::pack_message(wallet_handle, MESSAGE.as_bytes(), &receiver_key, Some(&verkey)).unwrap();
            let p1 = String::from_utf8(res_1.clone()).unwrap();
            let res_2 = crypto::collapse_ciphertext(&res_1).unwrap();
            let p2 = String::from_utf8(res_2.clone()).unwrap();
            let res_3 = crypto::collapse_ciphertext(&res_2).unwrap();
            let p3 = String::from_utf8(res_3.clone()).unwrap();
            println!("p1={}", &p1);
            println!("p2={}", &p2);
            println!("p3={}", &p3);
            assert_eq!(res_2, res_3);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_add_remove_cts_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let verkey_1 = crypto::create_key(wallet_handle, None).unwrap();
            let receiver_key = serde_json::to_string(&vec![verkey_1]).unwrap();
            let res_1 = crypto::pack_message(wallet_handle, MESSAGE.as_bytes(), &receiver_key, Some(&verkey)).unwrap();
            let p1 = String::from_utf8(res_1.clone()).unwrap();
            let res_2 = crypto::collapse_ciphertext(&res_1).unwrap();
            let p2 = String::from_utf8(res_2.clone()).unwrap();
            let (res_3, res_4) = crypto::remove_cts_from_msg(&res_2).unwrap();
            let p3 = String::from_utf8(res_3.clone()).unwrap();
            let p4 = String::from_utf8(res_4.clone()).unwrap();
            //println!("p1={}", &p1);
            println!("p2={}", &p2);
            //println!("p3={}", &p3);
            //println!("p4={}", &p4);
            // Adding cts to a message already containing cts gives erros
            assert!(crypto::add_cts_to_msg(&res_2, &res_4).is_err());
            let res_5 = crypto::add_cts_to_msg(&res_3, &res_4).unwrap();
            let p5 = String::from_utf8(res_5.clone()).unwrap();
            println!("p5={}", &p5);
            // TODO: Need to compare JSON without considering order of keys
            //assert_eq!(res_2, res_5)
        }

        #[test]
        fn indy_crypto_forward_msg_with_cd_works() {
            let (wallet_handle, verkey) = setup_with_key();
            let verkey_1 = crypto::create_key(wallet_handle, None).unwrap();
            let receiver_key = serde_json::to_string(&vec![verkey_1]).unwrap();
            let (typ, to) = (String::from("forward"), String::from("someone"));
            let res_1 = crypto::pack_message(wallet_handle, MESSAGE.as_bytes(), &receiver_key, Some(&verkey)).unwrap();
            let res_2 = crypto::collapse_ciphertext(&res_1).unwrap();
            let res_3 = crypto::forward_msg_with_cd(&typ, &to, &res_2).unwrap();
            let p1 = String::from_utf8(res_1.clone()).unwrap();
            let p2 = String::from_utf8(res_2.clone()).unwrap();
            let p3 = String::from_utf8(res_3.clone()).unwrap();
            println!("p1={}", &p1);
            println!("p2={}", &p2);
            println!("p3={}", &p3);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_crypto_pack_fwd_repeat_works() {
            let (wallet_handle, verkey_A1) = setup_with_key();
            let verkey_B3 = crypto::create_key(wallet_handle, None).unwrap();
            let verkey_B4 = crypto::create_key(wallet_handle, None).unwrap();
            let verkey_B9 = crypto::create_key(wallet_handle, None).unwrap();
            let receiver_key_B3 = serde_json::to_string(&vec![verkey_B3]).unwrap();
            let receiver_key_B4 = serde_json::to_string(&vec![verkey_B4]).unwrap();
            let receiver_key_B9 = serde_json::to_string(&vec![verkey_B9]).unwrap();

            let (typ, to) = (String::from("forward"), String::from("someone"));

            let mut large_message = MESSAGE.to_string();
            for _ in 0..1000 {
                large_message.push_str(&MESSAGE);
            }

            println!("originla message len={}", &large_message.len());

            let m = &large_message.as_bytes();

            // A.1 packs m for B.4
            let M1 = crypto::pack_message(wallet_handle,
                                          m,
                                          &receiver_key_B4, Some(&verkey_A1)).unwrap();
            println!("M1_len={}", &M1.len());

            let M2 = crypto::collapse_ciphertext(&M1).unwrap();

            // A.1 creates forward message for B.4
            let M3 = crypto::forward_msg_with_cd(&typ, &to, &M2).unwrap();
            println!("M3_len={}", &M3.len());
            let p3 = String::from_utf8(M3.clone()).unwrap();
            //println!("p3={}", &p3);

            let (M4_1, cts) = crypto::remove_cts_from_msg(&M3).unwrap();

            // A.1 packs the above forward message for B.3
            let M4_2 = crypto::pack_message(wallet_handle,
                                          &M4_1,
                                          &receiver_key_B3, Some(&verkey_A1)).unwrap();
            let M4 = crypto::add_cts_to_msg(&M4_2, &cts).unwrap();
            println!("M4_len={}", &M4.len());
            let p4 = String::from_utf8(M4.clone()).unwrap();
            //println!("p4={}", &p4);

            let M5 = crypto::collapse_ciphertext(&M4).unwrap();
            println!("M5_len={}", &M5.len());
            let p5 = String::from_utf8(M5.clone()).unwrap();
            //println!("p5={}", &p5);

            // A.1 creates forward message for B.3
            let M6 = crypto::forward_msg_with_cd(&typ, &to, &M5).unwrap();
            println!("M6_len={}", &M6.len());
            let p6 = String::from_utf8(M6.clone()).unwrap();
            //println!("p6={}", &p6);

            let (M7_1, cts1) = crypto::remove_cts_from_msg(&M6).unwrap();

            let M7_2 = crypto::pack_message(wallet_handle,
                                            &M7_1,
                                            &receiver_key_B9, Some(&verkey_A1)).unwrap();
            let M7 = crypto::add_cts_to_msg(&M7_2, &cts1).unwrap();
            println!("M7_len={}", &M7.len());
            let p7 = String::from_utf8(M7.clone()).unwrap();
            //println!("p7={}", &p7);

            let (M8_1, cts2) = crypto::remove_cts_from_msg(&M7).unwrap();

            let M8_2 = crypto::unpack_message(wallet_handle, &M8_1).unwrap();
            let M8_2_unpacked : UnpackMessage = serde_json::from_slice(&M8_2).unwrap();
            let M8_3 = M8_2_unpacked.message.as_bytes().to_vec();
            let p8_3 = String::from_utf8(M8_3.clone()).unwrap();
            //println!("p8_3={}", &p8_3);

            let M8 = crypto::add_cts_to_msg(&M8_3, &cts2).unwrap();
            let p8 = String::from_utf8(M8.clone()).unwrap();
            //println!("p8={}", &p8);

            let M9 = extract_forward_msg(&M8);
            let p9 = String::from_utf8(M9.clone()).unwrap();
            //println!("p9={}", &p9);

            let M10 = crypto::expand_ciphertext(&M9).unwrap();
            let p10 = String::from_utf8(M10.clone()).unwrap();
            //println!("p10={}", &p10);

            let (M11_1, cts4) = crypto::remove_cts_from_msg(&M10).unwrap();

            let M11_2 = crypto::unpack_message(wallet_handle, &M11_1).unwrap();
            let M11_2_unpacked : UnpackMessage = serde_json::from_slice(&M11_2).unwrap();
            let M11_3 = M11_2_unpacked.message.as_bytes().to_vec();
            let p11_3 = String::from_utf8(M11_3.clone()).unwrap();
            //println!("p8_3={}", &p8_3);

            let M11 = crypto::add_cts_to_msg(&M11_3, &cts4).unwrap();
            let p11 = String::from_utf8(M11.clone()).unwrap();
            //println!("p11={}", &p11);

            let M12 = extract_forward_msg(&M11);
            let p12 = String::from_utf8(M12.clone()).unwrap();
            //println!("p12={}", &p12);

            let M13 = crypto::expand_ciphertext(&M12).unwrap();
            let p13 = String::from_utf8(M13.clone()).unwrap();
            //println!("p13={}", &p13);

            let M14 = crypto::unpack_message(wallet_handle, &M13).unwrap();
            let M14_unpacked : UnpackMessage = serde_json::from_slice(&M14).unwrap();
            let M15 = M14_unpacked.message;

            assert_eq!(M15, large_message);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_empty_message() {
            let (wallet_handle, verkey) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::CommonInvalidParam3, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_no_receivers() {
            let (wallet_handle, verkey) = setup_with_key();
            let receiver_keys = "[]";
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_bad_wallet_handle() {
            let (wallet_handle, verkey) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle + 1, message, &receiver_keys, Some(&verkey));
            assert_code!(ErrorCode::WalletInvalidHandle, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_authcrypt_fails_invalid_verkey() {
            let (wallet_handle, _) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, Some(INVALID_BASE58_VERKEY));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

    }

    mod pack_message_anoncrypt {
        use super::*;

        #[test]
        fn indy_pack_message_anon_works() {
            let (wallet_handle, _ ) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert!(res.is_ok());
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_anoncrypt_fails_empty_message() {
            let (wallet_handle, _ ) = setup_with_key();
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert_code!(ErrorCode::CommonInvalidParam3, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_anoncrypt_fails_no_receivers() {
            let (wallet_handle, _ ) = setup_with_key();
            let receiver_keys = "[]";
            let message = "Hello World".as_bytes();
            let res = crypto::pack_message(wallet_handle, message, &receiver_keys, None);
            assert_code!(ErrorCode::CommonInvalidStructure, res);
            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_pack_message_anoncrypt_passes_bad_wallet_handle() {
            let rec_key_vec = vec![VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            //The wallet_handle and sender aren't used in this case, so any wallet_handle whether inited or not will work
            let res = crypto::pack_message(1, message, &receiver_keys, None);
            assert!(res.is_ok());
        }

    }

    mod unpack_message_authcrypt {
        use super::*;

        #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
        pub struct UnpackMessage {
            pub message: String,
            pub sender_verkey: String,
            pub recipient_verkey: String
        }

        #[test]
        fn indy_unpack_message_authcrypt_works() {
            //Test setup
            let (wallet_handle_sender, sender_verkey) = setup_with_key();
            let wallet_handle_receiver = wallet::create_and_open_default_wallet().unwrap();
            let receiver_verkey = crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE, &receiver_verkey];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let pack_message = crypto::pack_message(wallet_handle_sender, AGENT_MESSAGE.as_bytes(), &receiver_keys, Some(&sender_verkey)).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice()).unwrap();
            let res_serialized : UnpackMessage = serde_json::from_slice(res.as_slice()).unwrap();

            //verify unpack ran correctly
            assert_eq!(res_serialized.message, AGENT_MESSAGE.to_string());
            assert_eq!(res_serialized.sender_verkey, sender_verkey);
            assert_eq!(res_serialized.recipient_verkey, receiver_verkey);

            //teardown
            wallet::close_wallet(wallet_handle_sender).unwrap();
            utils::tear_down_with_wallet(wallet_handle_receiver);
        }

        #[test]
        fn indy_unpack_message_authcrypt_fails_no_matching_key() {
            //Test Setup
            let (wallet_handle_sender, sender_verkey) = setup_with_key();
            let wallet_handle_receiver = wallet::create_and_open_default_wallet().unwrap();
            crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let pack_message = crypto::pack_message(wallet_handle_sender, message, &receiver_keys, Some(&sender_verkey)).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice());

            assert_code!(ErrorCode::WalletItemNotFound, res);

            wallet::close_wallet(wallet_handle_sender).unwrap();
            utils::tear_down_with_wallet(wallet_handle_receiver);
        }

    }

    mod unpack_message_anoncrypt {
        use super::*;

        #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
        pub struct UnpackMessage {
            pub message: String,
            pub recipient_verkey: String
        }

        #[test]
        fn indy_unpack_message_anoncrypt_works() {
            let (wallet_handle_sender, _) = setup_with_key();
            let wallet_handle_receiver = wallet::create_and_open_default_wallet().unwrap();
            let receiver_verkey = crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE, &receiver_verkey];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let pack_message = crypto::pack_message(wallet_handle_sender, AGENT_MESSAGE.as_bytes(), &receiver_keys, None).unwrap();
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice()).unwrap();
            let res_serialized : UnpackMessage = serde_json::from_slice(res.as_slice()).unwrap();

            assert_eq!(res_serialized.message, AGENT_MESSAGE.to_string());
            assert_eq!(res_serialized.recipient_verkey, receiver_verkey);

            wallet::close_wallet(wallet_handle_sender).unwrap();
            utils::tear_down_with_wallet(wallet_handle_receiver);
        }

        #[test]
        fn indy_unpack_message_anoncrypt_fails_no_matching_key() {
            //Test Setup
            let (wallet_handle_sender, _) = setup_with_key();
            let wallet_handle_receiver = wallet::create_and_open_default_wallet().unwrap();
            crypto::create_key(wallet_handle_receiver, None).unwrap();
            let rec_key_vec = vec![VERKEY_TRUSTEE];
            let receiver_keys = serde_json::to_string(&rec_key_vec).unwrap();
            let message = "Hello World".as_bytes();
            let pack_message = crypto::pack_message(wallet_handle_sender, message, &receiver_keys, None).unwrap();

            //execute function
            let res = crypto::unpack_message(wallet_handle_receiver, pack_message.as_slice());

            assert_code!(ErrorCode::WalletItemNotFound, res);

            wallet::close_wallet(wallet_handle_sender).unwrap();
            utils::tear_down_with_wallet(wallet_handle_receiver);
        }

    }
}

mod load {
    extern crate rand;

    use super::*;

    use self::rand::{Rng, OsRng};

    use std::cmp::max;
    use std::thread;
    use std::time::{Duration, SystemTime};

    const AGENT_CNT: usize = 10;
    const DATA_SZ: usize = 10 * 1024;
    const OPERATIONS_CNT: usize = 10;

    /**
     Environment variables can be used for tuning this test:
     - AGENTS_CNT - count of parallel agents
     - OPERATIONS_CNT - operations per agent (consequence in same agent)
     - DATA_SZ - data size for encryption
    */
    #[test]
    fn parallel_auth_encrypt() {
        utils::setup();

        let agent_cnt = std::env::var("AGENTS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(AGENT_CNT);
        let data_sz = std::env::var("DATA_SZ").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(DATA_SZ);
        let operations_cnt = std::env::var("OPERATIONS_CNT").ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(OPERATIONS_CNT);

        let mut agents = Vec::new();
        let mut os_rng = OsRng::new().unwrap();
        for _ in 0..agent_cnt {
            let wallet = wallet::create_and_open_default_wallet().unwrap();
            let (_did, verkey) = did::create_and_store_my_did(wallet, None).unwrap();
            let mut data = vec![0u8; data_sz];
            os_rng.fill_bytes(&mut data.as_mut_slice());
            agents.push((wallet, verkey, data));
        }

        let start_time = SystemTime::now();

        let mut results = Vec::new();

        for (wallet, verkey, data) in agents {
            let thread = thread::spawn(move || {
                let mut time_diffs = Vec::new();
                for _ in 0..operations_cnt {
                    let time = SystemTime::now();
                    let _encrypted = crypto::auth_crypt(wallet, &verkey, &verkey, data.as_slice()).unwrap();
                    let time_diff = SystemTime::now().duration_since(time).unwrap();
                    time_diffs.push(time_diff);
                }

                wallet::close_wallet(wallet).unwrap();
                time_diffs
            });
            results.push(thread);
        }

        let mut all_diffs = Vec::new();
        for result in results {
            all_diffs.push(result.join().unwrap());
        }
        let total_duration = SystemTime::now().duration_since(start_time).unwrap();

        let mut time_diff_max = Duration::from_secs(0);
        let mut time_sum_diff = Duration::from_secs(0);
        for time_diffs in all_diffs {
            warn!("{:?}", time_diffs);
            time_diff_max = time_diffs.iter().fold(time_diff_max, |acc, cur| max(acc, *cur));
            time_sum_diff = time_diffs.iter().fold(time_sum_diff, |acc, cur| acc + *cur);
        }

        warn!("================= Settings =================\n\
        Agent cnt:               \t{:?}\n\
        Operations per agent cnt:\t{:?}\n\
        Data size:               \t{:?}",
              agent_cnt, operations_cnt, data_sz);

        warn!("================= Summary =================\n\
        Max pending:   \t{:?}\n\
        Total ops cnt: \t{:?}\n\
        Sum pending:   \t{:?}\n\
        Total duration:\t{:?}",
              time_diff_max, agent_cnt * operations_cnt, time_sum_diff, total_duration);

        utils::tear_down();
    }
}
