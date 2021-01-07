use std::{thread, time};
use std::time::{Duration, Instant};
extern crate futures;
extern crate tokio_core;
extern crate tokio_modbus;
extern crate tokio_serial;


#[cfg(feature = "rtu")]
use tokio_modbus::client::rtu::connect;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    use tokio_serial::{Serial, SerialPortSettings};
    use tokio_modbus::prelude::*;


    let tty_path = "COM6";
    let slave = Slave(17);

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 19200;
    let port = Serial::from_path(tty_path, &settings).unwrap();
   
    let ten_millis = time::Duration::from_millis(3000);
    
    loop{
        let mut now = Instant::now();
        thread::sleep(ten_millis);
        println!("{}", now.elapsed().as_secs());
    }
    
    let mut ctx = rtu::connect_slave(port, slave).await?;
    println!("Reading a sensor value");
    let rsp = ctx.read_holding_registers(45500, 2).await?;
    println!("Sensor value is: {:?}", rsp);
    Ok(())
}
