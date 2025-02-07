use crate::utils::env_utils::get_env_var;
use anyhow::Error;
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;

pub async fn post_data_to_bundler(envelope_data: Vec<u8>) -> Result<String, Error> {
    let private_key = get_env_var("WVM_AWS_S3_PK")?;

    let mut envelopes: Vec<Envelope> = vec![];
    let tags = vec![Tag::new(
        "Protocol".to_string(),
        "wvm-aws-sdk-s3".to_string(),
    )];

    let envelope: Envelope = Envelope::new()
        .data(Some(envelope_data))
        .target(None)
        .tags(Some(tags.clone()))
        .build()
        .unwrap();
    envelopes.push(envelope);

    let bundle_tx = Bundle::new()
        .private_key(private_key)
        .envelopes(envelopes)
        .build()
        .expect("ERROR SENDING BUNDLE")
        .propagate()
        .await
        .unwrap();

    Ok(bundle_tx)
}
