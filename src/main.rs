mod lb;
use pingora::server::Server;
use std::sync::Arc;
use pingora::lb::LoadBalancer;
use pingora::proxy::http_proxy_service;
use crate::lb::LB;

fn main() {
    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();
    let lb = LB(Arc::new(upstreams));

    let mut lb_service = http_proxy_service(&server.configuration, lb);
    lb_service.add_tcp("0.0.0.0:6188");

    server.add_service(lb_service);
    server.run_forever();
}
