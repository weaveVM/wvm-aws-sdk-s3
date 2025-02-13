use crate::utils::env_utils::get_env_var;
use anyhow::Error;
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;

pub async fn post_data_to_bundler(
    envelope_data: Vec<u8>,
    external_tags: Option<Vec<Tag>>,
) -> Result<String, Error> {
    let private_key = get_env_var("WVM_AWS_S3_PK")?;

    let mut envelopes: Vec<Envelope> = vec![];
    let mut tags: Vec<Tag> = vec![Tag::new(
        "Protocol".to_string(),
        "wvm-aws-sdk-s3".to_string(),
    )];

    if let Some(external_tags) = external_tags {
        tags.extend(external_tags);
    }

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
