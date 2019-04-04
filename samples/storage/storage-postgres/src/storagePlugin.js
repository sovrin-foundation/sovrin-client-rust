var ffi = require('ffi');
var ref = require('ref');
var int = ref.types.int;

var storagePlugin = ffi.Library(
    '/indy-sdk/experimental/plugins/postgres_storage/target/debug/libindystrgpostgres.so', 
    {
        "postgresstorage_init": [int, []]
    }
);
module.exports = storagePlugin;