#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//Basic Identity
use identity_iota::core::ToJson;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::storage::Storage;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use iota_sdk::client::api::GetAddressesOptions;
use iota_sdk::client::secret::stronghold;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::crypto::keys::bip39;
use iota_sdk::types::block::address::Bech32Address;
use iota_sdk::types::block::output::dto::AliasOutputDto;
use iota_sdk::types::block::output::AliasOutput;
use rand::Rng;
use tokio::io::AsyncReadExt;

//NFT Issuing
use identity_iota::iota::block::output::feature::MetadataFeature;
use identity_iota::iota::IotaDID;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::address::AliasAddress;
use iota_sdk::types::block::output::feature::IssuerFeature;
use iota_sdk::types::block::output::unlock_condition::AddressUnlockCondition;
use iota_sdk::types::block::output::AliasId;
use iota_sdk::types::block::output::Feature;
use iota_sdk::types::block::output::NftId;
use iota_sdk::types::block::output::NftOutput;
use iota_sdk::types::block::output::NftOutputBuilder;
use iota_sdk::types::block::output::Output;
use iota_sdk::types::block::output::OutputId;
use iota_sdk::types::block::output::RentStructure;
use iota_sdk::types::block::output::UnlockCondition;
use iota_sdk::types::block::payload::transaction::TransactionEssence;
use iota_sdk::types::block::payload::Payload;
use iota_sdk::types::block::Block;

//Load DotEnv
use dotenv::dotenv;
use std::env;

// The endpoint of the IOTA node to use.
static API_ENDPOINT: &str = "https://vienna.dlt.green";
static NODE: &str = "https://api.testnet.shimmer.network";
static FAUCET: &str = "https://faucet.testnet.shimmer.network";
static FAUCET_API: &str = "https://faucet.testnet.shimmer.network/api/enqueue";
static EXPLORER: &str = "https://explorer.shimmer.network/testnet";

// Mock Sensor with Data
fn get_temperature() -> f64 {
    //Generate random number betweeen 0 and 100
    let mut rng = rand::thread_rng();
    let temperature: f64 = rng.gen_range(0.0..100.0);
    temperature
}


/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Connecting to node: {}", NODE);
    // Create a new client to interact with the IOTA ledger.
    let client: Client = Client::builder()
        .with_primary_node(NODE, None)?
        .finish()
        .await?;
    let node_info = client.get_info().await?.node_info;
    let network_name: NetworkName = client.network_name().await?;
    let node_name = node_info.name;
    let healthy = node_info.status.is_healthy.to_string();
    let milestone = node_info.status.latest_milestone.index.to_string();
    println!(
        "Connected to node: {node_name} {healthy} {milestone} {}",
        network_name.to_string()
    );

    // Load the environment variables from the .env file.
    dotenv().ok();
    let passphrase = env::var("PASSPHRASE").expect("PASSPHRASE must be set");
    let stronghold_filename = "./stronghold_datastore.data";
    // Create a new Stronghold file
    println!("Creating a new Stronghold file: {}", stronghold_filename);
    let stronghold = StrongholdSecretManager::builder()
        .password(passphrase.to_owned())
        .build(stronghold_filename)?;

    // Generate a mnemonic and store it in the Stronghold.
    let random: [u8; 32] = rand::random();
    let mnemonic = bip39::wordlist::encode(random.as_ref(), &bip39::wordlist::ENGLISH)
        .map_err(|err| anyhow::anyhow!("{err:?}"))?;

    //Print the mnemonic
    let mnemonic_string = mnemonic.to_string();
    println!(
        "Your mnemonic is: ({})\nThis has been stored in stronghold!",
        mnemonic_string
    );

    //Store Mnemonic; If the stonghold already has the mnemonic just continue.
    stronghold
        .store_mnemonic(mnemonic)
        .await
        .unwrap_or_else(|err| {
            println!("Error storing mnemonic: {err:?}");
            println!("Mnemonic probably already exists in stronghold.")
        });

    // Create a new secret manager backed by the Stronghold.
    let secret_manager: SecretManager = SecretManager::Stronghold(stronghold);

    // Get an address from the secret manager.
    let address: Bech32Address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_range(0..1)
                .with_bech32_hrp((&network_name).try_into()?),
        )
        .await?[0];

    println!("Your wallet address is: {}", address);
    println!(
        "Please request funds from {FAUCET}, wait for a couple of seconds and then press Enter."
    );
    println!("To check for funds use the explorer: {EXPLORER}");
    tokio::io::stdin().read_u8().await?;

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    let mut document: IotaDocument = IotaDocument::new(&network_name);

    // Insert a new Ed25519 verification method in the DID document.
    let storage: Storage<JwkMemStore, KeyIdMemstore> =
        Storage::new(JwkMemStore::new(), KeyIdMemstore::new());
    document
        .generate_method(
            &storage,
            JwkMemStore::ED25519_KEY_TYPE,
            JwsAlgorithm::EdDSA,
            None,
            MethodScope::VerificationMethod,
        )
        .await?;

    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    let alias_output: AliasOutput = client
        .new_did_output(address.into(), document, None)
        .await?;
    println!(
        "Alias Output: {}",
        AliasOutputDto::from(&alias_output).to_json_pretty()?
    );

    // Publish the Alias Output and get the published DID document.
    let document: IotaDocument = client
        .publish_did_output(&secret_manager, alias_output)
        .await?;
    println!("Published DID document: {:#}", document);

    //Continue to create an NFT with data
    create_ntf(client, document, secret_manager).await
}



// After you have a DID issue an NFT with data.
async fn create_ntf(client: Client, manufacturer_document: IotaDocument, secret_manager: SecretManager) -> anyhow::Result<()> {
    let temperature = get_temperature(); // Mock sensor data.
    println!("Temperature: {}", temperature);

    let manufacturer_did = manufacturer_document.id().clone();

    // Get the current byte cost.
    let rent_structure: RentStructure = client.get_rent_structure().await?;

    //Temperature Metadata Feature for the NFT
    let temperature_metadata: MetadataFeature = MetadataFeature::new(
        format!("Temperature: {}°C", temperature).as_bytes().to_vec(),
    )?;


    // Create a Digital Product Passport NFT issued by the manufacturer.
    let product_passport_nft: NftOutput =
        NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, NftId::null())
            // The NFT will initially be owned by the manufacturer.
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                Address::Alias(AliasAddress::new(AliasId::from(&manufacturer_did))),
            )))
            // Set the manufacturer as the immutable issuer.
            .add_immutable_feature(Feature::Issuer(IssuerFeature::new(Address::Alias(
                AliasAddress::new(AliasId::from(&manufacturer_did)),
            ))))
            // A proper DPP would hold its metadata here.
            //.add_immutable_feature(Feature::Metadata(MetadataFeature::new(
            //    b"Digital Product Passport Metadata".to_vec(),
            //)?))
            .add_immutable_feature(temperature_metadata) //Adding Sensor Data
            .finish()?;

    // Publish the NFT.
    let block: Block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(vec![product_passport_nft.into()])?
        .finish()
        .await?;
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    // ========================================================
    // Resolve the Digital Product Passport NFT and its issuer.
    // ========================================================

    // Extract the identifier of the NFT from the published block.
    let nft_id: NftId =
        NftId::from(&get_nft_output_id(block.payload().ok_or_else(|| {
            anyhow::anyhow!("expected block to contain a payload")
        })?)?);

    // Fetch the NFT Output.
    let nft_output_id: OutputId = client.nft_output_id(nft_id).await?;
    let output: Output = client.get_output(&nft_output_id).await?.into_output();

    // Extract the issuer of the NFT.
    let nft_output: NftOutput = if let Output::Nft(nft_output) = output {
        nft_output
    } else {
        anyhow::bail!("expected NFT output")
    };

    let issuer_address: Address =
        if let Some(Feature::Issuer(issuer)) = nft_output.immutable_features().iter().next() {
            *issuer.address()
        } else {
            anyhow::bail!("expected an issuer feature")
        };

    let manufacturer_alias_id: AliasId = if let Address::Alias(alias_address) = issuer_address {
        *alias_address.alias_id()
    } else {
        anyhow::bail!("expected an Alias Address")
    };

    // Reconstruct the manufacturer's DID from the Alias Id.
    let network: NetworkName = client.network_name().await?;
    let manufacturer_did: IotaDID = IotaDID::new(&manufacturer_alias_id, &network);

    // Resolve the issuer of the NFT.
    let manufacturer_document: IotaDocument = client.resolve_did(&manufacturer_did).await?;

    println!("The issuer of the Digital Product Passport NFT is: {manufacturer_document:#}\n");

    println!("To view the DID go to {EXPLORER}/search/{manufacturer_did}");
    println!("To view the NFT go to {EXPLORER}/search/{nft_id}");
    Ok(())
}


// Helper function to get the output id for the first NFT output in a Block.
fn get_nft_output_id(payload: &Payload) -> anyhow::Result<OutputId> {
    match payload {
      Payload::Transaction(tx_payload) => {
        let TransactionEssence::Regular(regular) = tx_payload.essence();
        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Nft(_nft_output) = output {
            return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
          }
        }
        anyhow::bail!("no NFT output in transaction essence")
      }
      _ => anyhow::bail!("No transaction payload"),
    }
  }