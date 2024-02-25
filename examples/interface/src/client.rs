use dubbo::codegen::ClientBuilder;
use dubbo::registry::n_registry::ArcRegistry;
use example_interface::{DemoServiceRpc, ReqDto};
use registry_zookeeper::ZookeeperRegistry;

#[tokio::main(worker_threads = 512)]
async fn main() {
    let builder = ClientBuilder::new()
    .with_registry(ArcRegistry::new(ZookeeperRegistry::new("127.0.0.1:2181")));
    let mut client = DemoServiceRpc::new(builder);
    let res = client.sayHello("world".to_string()).await;
    println!("{:?}", res);
    let res = client
        .sayHelloV2(ReqDto {
            str: "world".to_string(),
        })
        .await;
    println!("{:?}", res);
}
