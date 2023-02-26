use crate::cli::arguments;
use crate::service::Service;
use coin_shuffle_protos::v1::shuffle_service_client::ShuffleServiceClient;
use ethers::types::U256;
use eyre::Result;

pub async fn shuffle(args: arguments::Shuffle) -> Result<()> {
    let mut service = Service::new(
        args.rpc_url,
        args.utxo_address,
        ShuffleServiceClient::connect(args.service_url).await?,
    )?;

    service
        .init_shuffle_room(
            U256::from(args.utxo_id.as_bytes()),
            args.output_address,
            args.rsa_priv_path,
            args.ecdsa_priv_path,
        )
        .await?;

    service.join_shuffle_room().await?;

    service.wait_shuffle().await?;

    service
        .shuffling(service.clone().connect_room().await?)
        .await?;

    Ok(())
}