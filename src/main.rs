use aws_sdk_dynamodb as dynamodb;

#[tokio::main]
async fn main() -> Result<(), dynamodb::Error> {
    let client = dynamodb::Client::from_env();
    println!("cl");
    create_table(&client).await?;
    Ok(())
}

use dynamodb::model::AttributeValue;

async fn create_table(client: &dynamodb::Client) -> Result<(), dynamodb::Error> {
    client
        .create_table()
        .table_name("hello-rust-sdk")
        .attribute_definitions(
            dynamodb::model::attribute_definition::Builder::default()
                .attribute_name("title")
                .attribute_type(dynamodb::model::ScalarAttributeType::S)
                .build(),
        )
        .key_schema(
            dynamodb::model::key_schema_element::Builder::default()
                .attribute_name("title")
                .key_type(dynamodb::model::KeyType::Hash)
                .build(),
        )
        .billing_mode(dynamodb::model::BillingMode::PayPerRequest)
        .send()
        .await?;

    // NOTE:
    // 本来はDescribeTableで `TableStatus` が `ACTIVE` になるまで待機するべきです。
    // 今回は実装を簡素にするためsleepで代用しています。
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
