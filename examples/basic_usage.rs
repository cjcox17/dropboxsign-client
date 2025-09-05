use dotenvy::dotenv;
use dropboxsign_client::{
    DropboxSignClient,
    signature_request::{SendSignatureRequest, SubCustomField, SubSignatureRequestTemplateSigner},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let signer_email = std::env::var("SIGNER_EMAIL").expect("SIGNER_EMAIL must be set");
    let template_id = std::env::var("TEMPLATE_ID").expect("TEMPLATE_ID must be set");

    println!("Using API_KEY: {api_key}");

    //Role should match the template role name
    let signer = SubSignatureRequestTemplateSigner::new(
        "Client".to_string(),
        "test name".to_string(),
        signer_email,
    );

    let custom_fields = vec![
        SubCustomField::new("test_field_one".to_string())
            .value("This is test field one!".to_string()),
        SubCustomField::new("test_field_two".to_string())
            .value("This is test field two!".to_string()),
    ];

    let client = DropboxSignClient::new(&api_key);

    let signature_request =
        SendSignatureRequest::new(vec![signer], vec![template_id]).custom_fields(custom_fields);

    let (response, warnings) = client.send_with_template(signature_request).await?;

    println!("Dropbox response: {response:?}");
    println!("Dropbox warnings: {warnings:?}");

    let signature_request_id = response.signature_request_id.clone();

    let (get_response, get_warnings) = client.get_signature_request(&signature_request_id).await?;

    println!("Dropbox get_response: {get_response:?}");
    println!("Dropbox get_warnings: {get_warnings:?}");

    Ok(())
}
