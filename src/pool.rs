use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;
use std::time::Duration;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

use native::pool;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseI32CB};

pub struct Pool {}

impl Pool {
    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// {
    ///     "genesis_txn": string (required), A path to genesis transaction file.
    /// }
    pub fn create_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// {
    ///     "genesis_txn": string (required), A path to genesis transaction file.
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_ledger_config_timeout(pool_name: &str, pool_config: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (required)- Pool configuration json. Example:
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_ledger_config_async<F: 'static>(pool_name: &str, pool_config: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_create_ledger_config(command_handle, pool_name, pool_config, cb)
    }

    fn _create_ledger_config(command_handle: IndyHandle, pool_name: &str, pool_config: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);
        let pool_config_str = opt_c_str!(pool_config);

        ErrorCode::from(unsafe { pool::indy_create_pool_ledger_config(command_handle, pool_name.as_ptr(), opt_c_ptr!(pool_config, pool_config_str), cb) })
    }

    /// Opens pool ledger and performs connecting to pool nodes.
    ///
    /// Pool ledger configuration with corresponded name must be previously created
    /// with indy_create_pool_ledger_config method.
    /// It is impossible to open pool with the same name more than once.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (optional)- Runtime pool configuration json.
    ///                         if NULL, then default config will be used. Example:
    /// {
    ///     "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
    ///                      Defaults to true.
    ///     "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
    ///                        Use 0 to disable automatic refresh. Defaults to 24*60.
    ///     "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
    ///                       Defaults to 20000.
    /// }
    ///
    /// # Returns
    /// Handle to opened pool to use in methods that require pool connection.
    pub fn open_ledger(pool_name: &str, config: Option<&str>) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Pool::_open_ledger(command_handle, pool_name, config, cb);

        ResultHandler::one(err, receiver)
    }

    /// Opens pool ledger and performs connecting to pool nodes.
    ///
    /// Pool ledger configuration with corresponded name must be previously created
    /// with indy_create_pool_ledger_config method.
    /// It is impossible to open pool with the same name more than once.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (optional)- Runtime pool configuration json.
    ///                         if NULL, then default config will be used. Example:
    /// {
    ///     "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
    ///                      Defaults to true.
    ///     "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
    ///                        Use 0 to disable automatic refresh. Defaults to 24*60.
    ///     "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
    ///                       Defaults to 20000.
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Handle to opened pool to use in methods that require pool connection.
    pub fn open_ledger_timeout(pool_name: &str, config: Option<&str>, timeout: Duration) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Pool::_open_ledger(command_handle, pool_name, config, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Opens pool ledger and performs connecting to pool nodes.
    ///
    /// Pool ledger configuration with corresponded name must be previously created
    /// with indy_create_pool_ledger_config method.
    /// It is impossible to open pool with the same name more than once.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration.
    /// * `config`  (optional)- Runtime pool configuration json.
    ///                         if NULL, then default config will be used. Example:
    /// {
    ///     "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
    ///                      Defaults to true.
    ///     "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
    ///                        Use 0 to disable automatic refresh. Defaults to 24*60.
    ///     "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
    ///                       Defaults to 20000.
    /// }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn open_ledger_async<F: 'static>(pool_name: &str, config: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, IndyHandle) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Pool::_open_ledger(command_handle, pool_name, config, cb)
    }

    fn _open_ledger(command_handle: IndyHandle, pool_name: &str, config: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);
        let config_str = opt_c_str!(config);

        ErrorCode::from(unsafe { pool::indy_open_pool_ledger(command_handle, pool_name.as_ptr(), opt_c_ptr!(config, config_str), cb) })
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    pub fn refresh(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_refresh(command_handle, pool_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    /// * `timeout` - the maximum time this function waits for a response
    pub fn refresh_timeout(pool_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_refresh(command_handle, pool_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn refresh_async<F: 'static>(pool_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_refresh(command_handle, pool_handle, cb)
    }

    fn _refresh(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_refresh_pool_ledger(command_handle, pool_handle, cb) })
    }

    /// Lists names of created pool ledgers
    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pool::_list(command_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// Lists names of created pool ledgers
    /// * `timeout` - the maximum time this function waits for a response
    pub fn list_timeout(timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pool::_list(command_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Lists names of created pool ledgers
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_async<F: 'static>(closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Pool::_list(command_handle, cb)
    }

    fn _list(command_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_list_pools(command_handle, cb) })
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    pub fn close(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_close(command_handle, pool_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn close_timeout(pool_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_close(command_handle, pool_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// # Arguments
    /// * `handle` - pool handle returned by Pool::open_ledger.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn close_async<F: 'static>(pool_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_close(command_handle, pool_handle, cb)
    }

    fn _close(command_handle: IndyHandle, pool_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        ErrorCode::from(unsafe { pool::indy_close_pool_ledger(command_handle, pool_handle, cb) })
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_delete(command_handle, pool_name, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn delete_timeout(pool_name: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_delete(command_handle, pool_name, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Deletes created pool ledger configuration.
    ///
    /// # Arguments
    /// * `config_name` - Name of the pool ledger configuration to delete.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn delete_async<F: 'static>(pool_name: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_delete(command_handle, pool_name, cb)
    }

    fn _delete(command_handle: IndyHandle, pool_name: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let pool_name = c_str!(pool_name);

        ErrorCode::from(unsafe { pool::indy_delete_pool_ledger_config(command_handle, pool_name.as_ptr(), cb) })
    }

    /// Set PROTOCOL_VERSION to specific version.
    ///
    /// There is a global property PROTOCOL_VERSION that used in every request to the pool and
    /// specified version of Indy Node which Libindy works.
    ///
    /// By default PROTOCOL_VERSION=1.
    ///
    /// # Arguments
    /// * `protocol_version` - Protocol version will be used:
    ///     1 - for Indy Node 1.3
    ///     2 - for Indy Node 1.4
    pub fn set_protocol_version(protocol_version: usize) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_set_protocol_version(command_handle, protocol_version, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Set PROTOCOL_VERSION to specific version.
    ///
    /// There is a global property PROTOCOL_VERSION that used in every request to the pool and
    /// specified version of Indy Node which Libindy works.
    ///
    /// By default PROTOCOL_VERSION=1.
    ///
    /// # Arguments
    /// * `protocol_version` - Protocol version will be used:
    ///     1 - for Indy Node 1.3
    ///     2 - for Indy Node 1.4
    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_protocol_version_timeout(protocol_version: usize, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pool::_set_protocol_version(command_handle, protocol_version, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Set PROTOCOL_VERSION to specific version.
    ///
    /// There is a global property PROTOCOL_VERSION that used in every request to the pool and
    /// specified version of Indy Node which Libindy works.
    ///
    /// By default PROTOCOL_VERSION=1.
    ///
    /// # Arguments
    /// * `protocol_version` - Protocol version will be used:
    ///     1 - for Indy Node 1.3
    ///     2 - for Indy Node 1.4
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn set_protocol_version_async<F: 'static>(protocol_version: usize, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pool::_set_protocol_version(command_handle, protocol_version, cb)
    }

    fn _set_protocol_version(command_handle: IndyHandle, protocol_version: usize, cb: Option<ResponseEmptyCB>) -> ErrorCode {

        ErrorCode::from(unsafe {
          pool::indy_set_protocol_version(command_handle, protocol_version, cb)
        })
    }
}


#[cfg(test)]
mod testing_helpers {
    use super::*;

    use std::env;
    use utils::test::rand;
    use utils::test::pool::PoolList;

    pub fn pool_name() -> String {
        format!("TestPool{}", rand::random_string(10))
    }

    pub fn sample_genesis_config() -> String {
        let mut sample_file = env::current_dir().unwrap();
        sample_file.push("storage/sample_genesis_txn.txn");
        assert!(sample_file.exists());

        json!({"genesis_txn": sample_file}).to_string()
    }

    pub fn create_default_pool_config() -> String {
        let name = pool_name();
        let config = sample_genesis_config();
        Pool::create_ledger_config(&name, Some(&config)).unwrap();

        name
    }

    pub fn assert_pool_exists(name: &str) {
        assert!(PoolList::new().pool_exists(name));
    }

    pub fn assert_pool_not_exists(name: &str) {
        assert!(! PoolList::new().pool_exists(name));
    }
}


#[cfg(test)]
mod test_pool_create_config {
    use super::*;
    use super::testing_helpers::*;

    use std::fs;
    use std::time::Duration;
    use std::sync::mpsc::channel;
    use utils::test::file::TempFile;

    const VALID_TIMEOUT: Duration = Duration::from_millis(250);

    /*
    Returns the file, otherwise the file would be deleted
    when it goes out of scope.rustc_lsan
    */
    fn invalid_temporary_genesis_config() -> (String, TempFile) {
        let file = TempFile::new(None).unwrap();
        fs::write(&file, b"Some nonsensical data").unwrap();
        let config = json!({"genesis_txn": file.as_ref()}).to_string();

        (config, file)
    }

    #[test]
    /* Create a valid config with custom genesis txn. */
    fn config_with_genesis_txn() {
        let name = pool_name();
        let config = sample_genesis_config();
        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!((), result.unwrap());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Config options missing genesis_txn */
    fn config_missing_genesis_txn() {
        let name = pool_name();
        let result = Pool::create_ledger_config(&name, Some("{}"));

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* A path which doesn't exists results in error. */
    fn config_with_unknown_path_genesis_txn() {
        let name = pool_name();
        let config = json!({"genesis_txn": "/nonexist15794345"}).to_string();
        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Error with an incorrectly formed gensis txn. */
    fn config_with_bad_genesis_txn() {
        let name = pool_name();
        let (config, _file) = invalid_temporary_genesis_config();

        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Must specify pool name when config is created. */
    fn config_with_empty_pool_name() {
        let name = pool_name();
        let config = sample_genesis_config();
        let result = Pool::create_ledger_config("", Some(&config));
    
        assert_eq!(ErrorCode::CommonInvalidParam2, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Error when creating a pool that already exists */
    fn config_already_exists() {
        let name = pool_name();
        let config = sample_genesis_config();

        let result = Pool::create_ledger_config(&name, Some(&config));
        assert_eq!((), result.unwrap());
    
        let result = Pool::create_ledger_config(&name, Some(&config));
        assert_eq!(ErrorCode::PoolLedgerConfigAlreadyExistsError, result.unwrap_err());
    
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config async. */
    fn config_async() {
        let name = pool_name();
        let config = sample_genesis_config();

        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some(&config),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::Success, result);

        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config async resulting in an early error: callback isn't called. */
    fn config_async_with_early_error() {
        let name = pool_name();
        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some("{}"),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result);
        assert!(receiver.recv_timeout(VALID_TIMEOUT).is_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Create a config async resulting in a late error: callback is called. */
    fn config_async_with_late_error() {
        let name = pool_name();
        let (config, _file) = invalid_temporary_genesis_config();
        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some(&config),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::CommonInvalidStructure, result);

        assert_pool_not_exists(&name);
    }

    #[test]
    /* Create a config with timeout. */
    fn config_timeout() {
        let name = pool_name();
        let config = sample_genesis_config();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some(&config),
            VALID_TIMEOUT
        );

        assert_eq!((), result.unwrap());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config timeout resulting in an error. */
    fn config_timeout_with_error() {
        let name = pool_name();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some("{}"),
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Timeout occurs while creating config. Pool is still created. */
    fn config_timeout_timeouts() {
        let name = pool_name();
        let config = sample_genesis_config();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some(&config), 
            Duration::from_micros(1)
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }
}


#[cfg(test)]
mod test_delete_config {
    use super::*;
    use super::testing_helpers::*;

    use std::sync::mpsc::channel;

    const VALID_TIMEOUT: Duration = Duration::from_millis(250);
    const NON_EXISTENT_NAME: &str = "a_pool_name_which_does_not_exist";

    #[test]
    /* Delete a pool_config. */
    fn delete_pool() {
        let pool_name = create_default_pool_config();
        assert_pool_exists(&pool_name);

        let result = Pool::delete(&pool_name);
        assert_eq!((), result.unwrap());

        assert_pool_not_exists(&pool_name);
    }

    #[test]
    /* Error deleting non existent pool_config. */
    fn delete_pool_not_exist() {
        let result = Pool::delete(NON_EXISTENT_NAME);
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }

    #[test]
    /* Error deleting an open pool_config. */
    fn delete_pool_open() {
        let pool_name = create_default_pool_config();
        let config = json!({
            "refresh_on_open": false,
            "auto_refresh_time": 0,
        }).to_string();

        Pool::set_protocol_version(2).unwrap();
        let pool_handle = Pool::open_ledger(&pool_name, Some(&config)).unwrap();

        let result = Pool::delete(&pool_name);
        assert_eq!(ErrorCode::CommonInvalidState, result.unwrap_err());
        assert_pool_exists(&pool_name);
        
        Pool::close(pool_handle).unwrap();
        Pool::delete(&pool_name).unwrap();
    }

    #[test]
    /* Delete pool async. */
    fn delete_pool_async() {
        let pool_name = create_default_pool_config();
        let (sender, receiver) = channel();

        let result = Pool::delete_async(&pool_name, move |ec| sender.send(ec).unwrap());
        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::Success, result);

        assert_pool_not_exists(&pool_name);
    }


    #[test]
    /* Delete pool async resuting in a late error: callback is called. */
    fn delete_pool_async_late_error() {
        let (sender, receiver) = channel();

        let result = Pool::delete_async(
            NON_EXISTENT_NAME,
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::CommonIOError, result);
    }

    #[test]
    /* Delete a pool with a timeout. */ 
    fn delete_pool_timeout() {
        let pool_name = create_default_pool_config();

        let result = Pool::delete_timeout(&pool_name, VALID_TIMEOUT);
        assert_eq!((), result.unwrap());

        assert_pool_not_exists(&pool_name)
    }

    #[test]
    /* Error deleting a pool with a timeout. */
    fn delete_pool_timeout_error() {
        let result = Pool::delete_timeout(
            NON_EXISTENT_NAME,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }

    #[test]
    /* Delete a pool with timeout that timeouts. */
    fn delete_pool_timeout_timeouts() {
        let pool_name = create_default_pool_config();

        let result = Pool::delete_timeout(
            &pool_name,
            Duration::from_micros(1)
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());

        assert_pool_not_exists(&pool_name)
    }
}

#[cfg(test)]
mod test_set_protocol_version {
    use super::*;

    use ledger::Ledger;
    use serde_json;

    const VALID_VERSIONS: [usize; 2] = [1, 2];

    fn assert_protocol_version_set(version: usize) {
        let did = "5UBVMdSADMjGzuJMQwJ6yyzYV1krTcKRp6EqRAz8tiDP";
        let request = Ledger::build_get_nym_request(did, did).unwrap();
        let request: serde_json::Value = serde_json::from_str(&request).unwrap();
        assert_eq!(json!(version), *request.get("protocolVersion").unwrap());
    }

    #[test]
    /* Set all available protocol versions. */
    fn set_all_valid_versions() {
        for &version in VALID_VERSIONS.into_iter() {
            let result = Pool::set_protocol_version(version);
            assert_eq!((), result.unwrap());
            assert_protocol_version_set(version);
        }
    }

    #[test]
    /* Error setting invalid protocol version. */
    fn set_invalid_versions() {
        Pool::set_protocol_version(1).unwrap();
        
        let result = Pool::set_protocol_version(0);
        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result.unwrap_err());
        assert_protocol_version_set(1);

        let next_protocol_version = *VALID_VERSIONS.last().unwrap() + 1;
        let result = Pool::set_protocol_version(next_protocol_version);
        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result.unwrap_err());
        assert_protocol_version_set(1);
    }
}