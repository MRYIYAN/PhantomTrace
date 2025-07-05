//---------------------------------------------------------//
// Instalar todas las dependencias necesarias
// Instalar Rust y sus herramientas
//--------------------------------------------------------// 

use libcap
use redis:: AsyncCommands;
use serde::{Serialize};
use std::error::Error;
use tokio; 

#[derive(Serialize)]
struct PacketData {
    source: String,
    destination: String,
    protocol: String,
    length: usize,
    payload: String,
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    // Elegir el primer dispositivo disponible en la red 
    let device = Device::lookup()?
        .ok_or("No se encontró ningún dispositivo de red")?;
    let mut cap = Capture::from_device(device)?.inmediate_mode(true).open()?;

    // Conectar a Redis
    let client = redis/