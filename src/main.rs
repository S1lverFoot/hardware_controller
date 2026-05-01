use hardware_controller::{AuthProxy, HttpRequest, ApiKeyStrategy, JwtStrategy};

fn main() {
    println!("ТЕСТ АВТОРИЗАЦІЙНОГО ПРОКСІ (TASK 8)\n");
    let request1 = HttpRequest::new("https://cloud-server.com/api/telemetry", "{ temp: 45, fan: 1200 }");
    let mut api_proxy = AuthProxy::new(Box::new(ApiKeyStrategy {
        api_key: "super_secret_hardware_key_999".to_string(),
    }));

    let response1 = api_proxy.send_request(request1.clone());
    println!("Результат сервера: HTTP {}\n", response1.status);
    api_proxy.set_strategy(Box::new(JwtStrategy {
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
    }));

    let response2 = api_proxy.send_request(request1.clone());
    println!("Результат сервера: HTTP {}\n", response2.status);
    api_proxy.set_strategy(Box::new(JwtStrategy {
        token: "".to_string(),
    }));

    let response3 = api_proxy.send_request(request1);
    println!("Результат сервера: HTTP {}\n", response3.status);
}