use aliyun_openapi_core_rust_sdk::ROAClient;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // create rpc style api client.
    let aliyun_openapi_client = ROAClient::new(
        env::var("ACCESS_KEY_ID")?,
        env::var("ACCESS_KEY_SECRET")?,
        String::from("https://ros.aliyuncs.com"),
        String::from("2015-09-01"),
    );

    // call `DescribeRegions` with empty queries.
    let response = aliyun_openapi_client.get("/regions").send()?;
    println!("DescribeRegions response: {}", response);

    Ok(())
}
