//! [DELETE /_matrix/client/r0/room_keys/keys/{roomId}/{sessionId}](https://matrix.org/docs/spec/client_server/unstable#delete-matrix-client-r0-room-keys-keys-roomid-sessionid)

use ruma_api::ruma_api;
use ruma_identifiers::RoomId;
use js_int::UInt;

ruma_api! {
    metadata: {
        description: "Delete a key from the backup",
        method: GET,
        name: "delete_backup_key_session",
        path: "/_matrix/client/r0/room_keys/keys/:room_id/:session_id",
        rate_limited: true,
        requires_authentication: true,
    }

    request: {
        /// The backup version. Must be the current backup.
        #[ruma_api(query)]
        pub version: String,

        /// Room ID.
        #[ruma_api(path)]
        pub room_id: RoomId,
        /// Session ID.
        #[ruma_api(path)]
        pub session_id: String,
    }

    response: {
        /// An opaque string representing stored keys in the backup. Clients can compare it with
        /// the etag value they received in the request of their last key storage request.
        pub etag: String,

        /// The number of keys stored in the backup.
        pub count: UInt,
    }

    error: crate::Error
}