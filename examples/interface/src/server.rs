use std::env;
use dubbo::Dubbo;
use dubbo::registry::n_registry::ArcRegistry;
use dubbo_base::Url;
use dubbo_config::RootConfig;
use dubbo_macro::rpc_server;
use example_interface::{DemoService, ReqDto, ResDto};
use registry_nacos::NacosRegistry;
use registry_zookeeper::ZookeeperRegistry;

#[derive(Clone)]
struct DemoServiceImpl {
    _db: String,
}

#[rpc_server(package = "org.apache.dubbo.springboot.demo")]
impl DemoService for DemoServiceImpl {
    async fn sayHello(&self, req: String) -> Result<String, dubbo::status::Status> {
        println!("res : {:?}", req);
        return Ok("Hello ".to_owned() + &req);
    }
    async fn sayHelloV2(&self, req: ReqDto) -> Result<ResDto, dubbo::status::Status> {
        println!("res : {:?}", req);
        return Ok(ResDto {
            str: "Hello ".to_owned() + &req.str + " V2",
        });
    }
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    dubbo_logger::init();
    env::set_var("DUBBO_CONFIG_PATH", "/Users/kwsc98/Desktop/mySpace/dubbo-rust/examples/interface/application.yaml");
    let r = RootConfig::new();
    let r = match r.load() {
        Ok(config) => config,
        Err(_err) => panic!("err: {:?}", _err), // response was droped
    };
    let server = DemoServiceImpl { _db: "i am db".to_string() };
    let zookeeper_registry = ZookeeperRegistry::new("127.0.0.1:2181");
    let mut f = Dubbo::new()
        .with_config(r)
        .add_registry("zookeeper-registry", ArcRegistry::new(zookeeper_registry))
        .register_server(server);
    f.start().await;
}
