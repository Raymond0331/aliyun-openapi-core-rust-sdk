use aliyun_openapi_core_rust_sdk::RPClient;
use std::env;
use std::error::Error;

#[tokio::test]
async fn rpc_client() -> Result<(), Box<dyn Error>> {
    // create rpc style api client.
    let aliyun_openapi_client = RPClient::new(
        env::var("ACCESS_KEY_ID")?,
        env::var("ACCESS_KEY_SECRET")?,
        String::from("https://ecs.aliyuncs.com/"),
        String::from("2014-05-26"),
    );

    // call `DescribeRegions` with empty queries.
    let response = aliyun_openapi_client.get("DescribeRegions").send().await?;
    println!("DescribeRegions{}",response);
    //assert!(response.is_ok());

    Ok(())
}

#[tokio::test]
async fn rpc_client_with_queries() -> Result<(), Box<dyn Error>> {
    // create rpc style api client.
    let aliyun_openapi_client = RPClient::new(
        env::var("ACCESS_KEY_ID")?,
        env::var("ACCESS_KEY_SECRET")?,
        String::from("https://ecs.aliyuncs.com/"),
        String::from("2014-05-26"),
    );

    // call `DescribeInstances` with queries.
    let response = aliyun_openapi_client
        .get("DescribeInstances")
        .query(&[("RegionId", "cn-hangzhou")])
        .send().await?;
    
    println!("{}",response);
    //assert!(response.is_ok);

    Ok(())
}
