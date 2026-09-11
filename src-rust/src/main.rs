use tokio::net::UdpSocket;
use tonic::Request;

pub mod domain {
    tonic::include_proto!("domain");
}

use domain::auth_engine_client::AuthEngineClient;
use domain::UserRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Rust KDC] Starte Kerberos Core auf UDP 0.0.0.0:88...");

    let socket = UdpSocket::bind("0.0.0.0:88").await?;
    let mut buf = [0u8; 1024];

    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        println!("[Rust KDC] {} Bytes empfangen von {}", len, src);

        // Verbindung zum C# Management-Dienst aufbauen
        if let Ok(mut client) = AuthEngineClient::connect("http://127.0.0.1:50051").await {
            let req = Request::new(UserRequest {
                username: "admin".into(),
                realm: "CORP.LOCAL".into(),
            });

            if let Ok(res) = client.validate_user(req).await {
                let user = res.into_inner();
                println!("[Rust KDC] C# Validierung: Exists={}, Active={}", user.exists, user.is_active);
            }
        }

        let dummy_response = b"KERBEROS_AS_REP_DUMMY";
        socket.send_to(dummy_response, &src).await?;
    }
}
