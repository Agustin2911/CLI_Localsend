use std::io::{Read, Result};
use std::net::TcpListener;

fn main() -> Result<()> {
    let direccion = "0.0.0.0:8080";
    let listener = TcpListener::bind(direccion)?;
    
    println!("👂 Servidor de prueba escuchando en {}...", direccion);
    println!("Esperando que tu emisor se conecte...\n");

    // Acepta la primera conexión que llegue
    let (mut stream, cliente_ip) = listener.accept()?;
    println!("🔗 ¡Conexión recibida desde: {}!\n", cliente_ip);

    let mut buffer = [0u8; 65536]; // Buffer de 64 KB
    let mut total_bytes: u64 = 0;

    loop {
        let bytes_leidos = stream.read(&mut buffer)?;

        // Si devuelve 0, el emisor cerró el stream (final de la transferencia)
        if bytes_leidos == 0 {
            break;
        }

        total_bytes += bytes_leidos as u64;

        // Convierte el fragmento a texto para ver qué está llegando
        let texto_recibido = String::from_utf8_lossy(&buffer[..bytes_leidos]);

        println!(
            "📥 Paquete leal: {} bytes | Total recibido: {} bytes",
            bytes_leidos, total_bytes
        );
        
        // Muestra las primeras líneas del buffer (útil si mandás metadatos o texto)
        if let Some(primera_linea) = texto_recibido.lines().next() {
            println!("   📄 Vista previa del contenido: \"{}\"", primera_linea);
        }
    }

    println!("\n✅ El emisor cerró la conexión.");
    println!("📊 RESUMEN: Se recibieron {} bytes en total sin errores.", total_bytes);

    Ok(())
}