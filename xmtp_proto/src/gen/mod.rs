// @generated
pub mod xmtp {
    #[cfg(feature = "xmtp-device_sync")]
    // @@protoc_insertion_point(attribute:xmtp.device_sync)
    pub mod device_sync {
        include!("xmtp/device_sync/xmtp.device_sync.rs");
        // @@protoc_insertion_point(xmtp.device_sync)
        #[cfg(feature = "xmtp-device_sync-consent_backup")]
        // @@protoc_insertion_point(attribute:xmtp.device_sync.consent_backup)
        pub mod consent_backup {
            include!("xmtp/device_sync/consent_backup/xmtp.device_sync.consent_backup.rs");
            // @@protoc_insertion_point(xmtp.device_sync.consent_backup)
        }
        #[cfg(feature = "xmtp-device_sync-content")]
        // @@protoc_insertion_point(attribute:xmtp.device_sync.content)
        pub mod content {
            include!("xmtp/device_sync/content/xmtp.device_sync.content.rs");
            // @@protoc_insertion_point(xmtp.device_sync.content)
        }
        #[cfg(feature = "xmtp-device_sync-event_backup")]
        // @@protoc_insertion_point(attribute:xmtp.device_sync.event_backup)
        pub mod event_backup {
            include!("xmtp/device_sync/event_backup/xmtp.device_sync.event_backup.rs");
            // @@protoc_insertion_point(xmtp.device_sync.event_backup)
        }
        #[cfg(feature = "xmtp-device_sync-group_backup")]
        // @@protoc_insertion_point(attribute:xmtp.device_sync.group_backup)
        pub mod group_backup {
            include!("xmtp/device_sync/group_backup/xmtp.device_sync.group_backup.rs");
            // @@protoc_insertion_point(xmtp.device_sync.group_backup)
        }
        #[cfg(feature = "xmtp-device_sync-message_backup")]
        // @@protoc_insertion_point(attribute:xmtp.device_sync.message_backup)
        pub mod message_backup {
            include!("xmtp/device_sync/message_backup/xmtp.device_sync.message_backup.rs");
            // @@protoc_insertion_point(xmtp.device_sync.message_backup)
        }
    }
    #[cfg(feature = "xmtp-identity")]
    // @@protoc_insertion_point(attribute:xmtp.identity)
    pub mod identity {
        include!("xmtp/identity/xmtp.identity.rs");
        // @@protoc_insertion_point(xmtp.identity)
        pub mod api {
            #[cfg(feature = "xmtp-identity-api-v1")]
            // @@protoc_insertion_point(attribute:xmtp.identity.api.v1)
            pub mod v1 {
                include!("xmtp/identity/api/v1/xmtp.identity.api.v1.rs");
                // @@protoc_insertion_point(xmtp.identity.api.v1)
            }
        }
        #[cfg(feature = "xmtp-identity-associations")]
        // @@protoc_insertion_point(attribute:xmtp.identity.associations)
        pub mod associations {
            include!("xmtp/identity/associations/xmtp.identity.associations.rs");
            // @@protoc_insertion_point(xmtp.identity.associations)
        }
    }
    pub mod keystore_api {
        #[cfg(feature = "xmtp-keystore_api-v1")]
        // @@protoc_insertion_point(attribute:xmtp.keystore_api.v1)
        pub mod v1 {
            include!("xmtp/keystore_api/v1/xmtp.keystore_api.v1.rs");
            // @@protoc_insertion_point(xmtp.keystore_api.v1)
        }
    }
    pub mod message_api {
        #[cfg(feature = "xmtp-message_api-v1")]
        // @@protoc_insertion_point(attribute:xmtp.message_api.v1)
        pub mod v1 {
            include!("xmtp/message_api/v1/xmtp.message_api.v1.rs");
            // @@protoc_insertion_point(xmtp.message_api.v1)
        }
    }
    #[cfg(feature = "xmtp-message_contents")]
    // @@protoc_insertion_point(attribute:xmtp.message_contents)
    pub mod message_contents {
        include!("xmtp/message_contents/xmtp.message_contents.rs");
        // @@protoc_insertion_point(xmtp.message_contents)
    }
    pub mod migration {
        pub mod api {
            #[cfg(feature = "xmtp-migration-api-v1")]
            // @@protoc_insertion_point(attribute:xmtp.migration.api.v1)
            pub mod v1 {
                include!("xmtp/migration/api/v1/xmtp.migration.api.v1.rs");
                // @@protoc_insertion_point(xmtp.migration.api.v1)
            }
        }
    }
    pub mod mls {
        pub mod api {
            #[cfg(feature = "xmtp-mls-api-v1")]
            // @@protoc_insertion_point(attribute:xmtp.mls.api.v1)
            pub mod v1 {
                include!("xmtp/mls/api/v1/xmtp.mls.api.v1.rs");
                // @@protoc_insertion_point(xmtp.mls.api.v1)
            }
        }
        #[cfg(feature = "xmtp-mls-database")]
        // @@protoc_insertion_point(attribute:xmtp.mls.database)
        pub mod database {
            include!("xmtp/mls/database/xmtp.mls.database.rs");
            // @@protoc_insertion_point(xmtp.mls.database)
        }
        #[cfg(feature = "xmtp-mls-message_contents")]
        // @@protoc_insertion_point(attribute:xmtp.mls.message_contents)
        pub mod message_contents {
            include!("xmtp/mls/message_contents/xmtp.mls.message_contents.rs");
            // @@protoc_insertion_point(xmtp.mls.message_contents)
            #[cfg(feature = "xmtp-mls-message_contents-content_types")]
            // @@protoc_insertion_point(attribute:xmtp.mls.message_contents.content_types)
            pub mod content_types {
                include!("xmtp/mls/message_contents/content_types/xmtp.mls.message_contents.content_types.rs");
                // @@protoc_insertion_point(xmtp.mls.message_contents.content_types)
            }
        }
    }
    pub mod mls_validation {
        #[cfg(feature = "xmtp-mls_validation-v1")]
        // @@protoc_insertion_point(attribute:xmtp.mls_validation.v1)
        pub mod v1 {
            include!("xmtp/mls_validation/v1/xmtp.mls_validation.v1.rs");
            // @@protoc_insertion_point(xmtp.mls_validation.v1)
        }
    }
    pub mod xmtpv4 {
        #[cfg(feature = "xmtp-xmtpv4-envelopes")]
        // @@protoc_insertion_point(attribute:xmtp.xmtpv4.envelopes)
        pub mod envelopes {
            include!("xmtp/xmtpv4/envelopes/xmtp.xmtpv4.envelopes.rs");
            // @@protoc_insertion_point(xmtp.xmtpv4.envelopes)
        }
        #[cfg(feature = "xmtp-xmtpv4-gateway_api")]
        // @@protoc_insertion_point(attribute:xmtp.xmtpv4.gateway_api)
        pub mod gateway_api {
            include!("xmtp/xmtpv4/gateway_api/xmtp.xmtpv4.gateway_api.rs");
            // @@protoc_insertion_point(xmtp.xmtpv4.gateway_api)
        }
        #[cfg(feature = "xmtp-xmtpv4-message_api")]
        // @@protoc_insertion_point(attribute:xmtp.xmtpv4.message_api)
        pub mod message_api {
            include!("xmtp/xmtpv4/message_api/xmtp.xmtpv4.message_api.rs");
            // @@protoc_insertion_point(xmtp.xmtpv4.message_api)
        }
        #[cfg(feature = "xmtp-xmtpv4-metadata_api")]
        // @@protoc_insertion_point(attribute:xmtp.xmtpv4.metadata_api)
        pub mod metadata_api {
            include!("xmtp/xmtpv4/metadata_api/xmtp.xmtpv4.metadata_api.rs");
            // @@protoc_insertion_point(xmtp.xmtpv4.metadata_api)
        }
        #[cfg(feature = "xmtp-xmtpv4-payer_api")]
        // @@protoc_insertion_point(attribute:xmtp.xmtpv4.payer_api)
        pub mod payer_api {
            include!("xmtp/xmtpv4/payer_api/xmtp.xmtpv4.payer_api.rs");
            // @@protoc_insertion_point(xmtp.xmtpv4.payer_api)
        }
    }
}