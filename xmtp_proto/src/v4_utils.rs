use crate::types::TopicKind;
use crate::xmtp::mls::api::v1::KeyPackageUpload;
use crate::xmtp::xmtpv4::envelopes::{
    ClientEnvelope, OriginatorEnvelope, PayerEnvelope, UnsignedOriginatorEnvelope,
};
use crate::ConversionError;
use openmls::key_packages::KeyPackageIn;
use openmls::prelude::tls_codec::Deserialize;
use openmls::prelude::{MlsMessageIn, ProtocolMessage, ProtocolVersion};
use openmls_rust_crypto::RustCrypto;
use prost::Message;

pub const MLS_PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion::Mls10;

pub trait Extract {
    fn client_envelope(&self) -> Result<ClientEnvelope, ConversionError>;
    fn unsigned_originator_envelope(&self) -> Result<UnsignedOriginatorEnvelope, ConversionError>;
}

impl Extract for OriginatorEnvelope {
    fn client_envelope(&self) -> Result<ClientEnvelope, ConversionError> {
        let unsigned = self.unsigned_originator_envelope()?;
        let payer_envelope: PayerEnvelope =
            prost::Message::decode(unsigned.payer_envelope_bytes.as_slice())?;
        // TODO: Verify payer envelope
        Ok(ClientEnvelope::decode(
            &mut payer_envelope.unsigned_client_envelope.as_slice(),
        )?)
    }

    fn unsigned_originator_envelope(&self) -> Result<UnsignedOriginatorEnvelope, ConversionError> {
        let mut unsigned_bytes = self.unsigned_originator_envelope.as_slice();
        Ok(UnsignedOriginatorEnvelope::decode(&mut unsigned_bytes)?)
    }
}

pub fn build_key_package_topic(installation_id: &[u8]) -> Vec<u8> {
    let mut topic = Vec::with_capacity(1 + installation_id.len());
    topic.push(TopicKind::KeyPackagesV1 as u8);
    topic.extend_from_slice(installation_id);
    topic
}

pub fn build_identity_update_topic(inbox_id: &[u8]) -> Vec<u8> {
    let mut topic = Vec::with_capacity(1 + inbox_id.len());
    topic.push(TopicKind::IdentityUpdatesV1 as u8);
    topic.extend_from_slice(inbox_id);
    topic
}

pub fn build_group_message_topic(group_id: &[u8]) -> Vec<u8> {
    let mut topic = Vec::with_capacity(1 + group_id.len());
    topic.push(TopicKind::GroupMessagesV1 as u8);
    topic.extend_from_slice(group_id);
    topic
}

pub fn build_welcome_message_topic(installation_id: &[u8]) -> Vec<u8> {
    let mut topic = Vec::with_capacity(1 + installation_id.len());
    topic.push(TopicKind::WelcomeMessagesV1 as u8);
    topic.extend_from_slice(installation_id);
    topic
}

pub fn build_identity_topic_from_hex_encoded(
    hex_encoded_inbox_id: &String,
) -> Result<Vec<u8>, crate::ProtoError> {
    let decoded_inbox_id = hex::decode(hex_encoded_inbox_id)?;
    Ok(build_identity_update_topic(&decoded_inbox_id))
}

pub fn extract_unsigned_originator_envelope(
    req: &OriginatorEnvelope,
) -> Result<UnsignedOriginatorEnvelope, crate::ProtoError> {
    let mut unsigned_bytes = req.unsigned_originator_envelope.as_slice();
    Ok(UnsignedOriginatorEnvelope::decode(&mut unsigned_bytes)?)
}

pub fn get_group_message_topic(message: Vec<u8>) -> Result<Vec<u8>, crate::ProtoError> {
    let msg_result = MlsMessageIn::tls_deserialize(&mut message.as_slice())?;
    let protocol_message: ProtocolMessage = msg_result.try_into_protocol_message()?;

    Ok(build_group_message_topic(
        protocol_message.group_id().as_slice(),
    ))
}

pub fn get_key_package_topic(key_package: &KeyPackageUpload) -> Result<Vec<u8>, crate::ProtoError> {
    let kp_in: KeyPackageIn =
        KeyPackageIn::tls_deserialize_exact(key_package.key_package_tls_serialized.as_slice())?;
    let rust_crypto = RustCrypto::default();
    let kp = kp_in.validate(&rust_crypto, MLS_PROTOCOL_VERSION)?;
    let installation_key = kp.leaf_node().signature_key().as_slice();
    Ok(build_key_package_topic(installation_key))
}
