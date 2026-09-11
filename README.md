# domain-controller















# Hybrid Domain Controller Core (C# & Rust)

Ein experimenteller, hochperformanter Domain Controller Kernel. Er kombiniert die Low-Level-Performance und Speichersicherheit von **Rust** für Netzwerk- und Authentifizierungsprotokolle (Kerberos / LDAP) mit der Flexibilität von **C# / .NET 8** für das Verzeichnis-Management.

## Architecture

* **Rust (`src-rust`)**: UDP/TCP-Listener für Port 88 (Kerberos) und Port 389 (LDAP). Verarbeitet Paket-Parser auf Byte-Ebene.
* **C# (`src-csharp`)**: Management-Engine, Business-Logik und gRPC-Server.
* **gRPC (`proto/auth.proto`)**: Schnelle Inter-Prozess-Kommunikation zwischen Rust und C#.

## Quickstart

### Prerequisites
* .NET 8.0 SDK
* Rust & Cargo

### 1. C# Management Service starten
```bash
cd src-csharp
dotnet run











cd src-rust
cargo run
echo "TEST_AS_REQ" | nc -u -w1 127.0.0.1 88
