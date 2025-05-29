use crate::s3::S3_CONFIG;
use crate::utils::env_utils::get_env_var;
use anyhow::{anyhow, Error};
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;

pub async fn post_data_to_bundler(
    envelope_data: Vec<u8>,
    external_tags: Option<Vec<Tag>>,
) -> Result<String, Error> {
    let conf = S3_CONFIG.get().unwrap();
    let private_key = conf.private_key.clone();
    let load0_api_key = Some(std::env::var("API_INTERNAL_KEY").unwrap_or("".to_string()));

    let mut envelopes: Vec<Envelope> = vec![];
    let mut tags: Vec<Tag> = vec![Tag::new(
        "Protocol".to_string(),
        "wvm-aws-sdk-s3".to_string(),
    )];

    if let Some(external_tags) = external_tags {
        tags.extend(external_tags);
    }

    let envelope: Envelope = match Envelope::new()
        .data(Some(envelope_data))
        .target(None)
        .tags(Some(tags.clone()))
        .build() {
        Ok(envelope) => envelope,
        Err(e) => {
            eprintln!("post_data_to_bundler error {:?}", e);
            return Err(anyhow!("Error creating bundle request"));
        }
    };

    envelopes.push(envelope);

    let bundle_tx = Bundle::new()
        .private_key(private_key)
        .envelopes(envelopes)
        .build()
        .expect("ERROR SENDING BUNDLE")
        .propagate_to_load0(load0_api_key)
        .await;

    let bundle_tx = match bundle_tx {
        Ok(bundle_tx) => bundle_tx,
        Err(e) => {
            eprintln!("post_data_to_bundler error {:?}", e);
            return Err(anyhow!("Error creating bundle request"));
        }
    };

    Ok(bundle_tx)
}
