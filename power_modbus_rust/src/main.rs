use mysql_async::prelude::*;

extern crate futures;
extern crate tokio_core;
extern crate tokio_modbus;
extern crate tokio_serial;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
    struct Power {
        name: String,
        voltage: f32,
    }

#[cfg(feature = "rtu")]
use tokio_modbus::client::rtu::connect;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_serial::{Serial, SerialPortSettings};
    use tokio_modbus::prelude::*;

    let url = "mysql://root:v?577ZX@localhost:3306/test";
    let pool = mysql_async::Pool::new(url);
    let mut conn = pool.get_conn().await?;

    let powers = vec![
        Power { name: String::from("power meter 2"), voltage: 30.2},
        Power { name: String::from("power meter 2"), voltage: 30.2},
    ];

    // conn.exec_batch(
    //     r"INSERT INTO power_meter (power_meter_name, power_meter_voltage)
    //       VALUES (:power_meter_name, :power_meter_voltage)",
    //       powers.iter().map(|p| params! {
    //         "power_meter_name" => p.name,
    //         "power_meter_voltage" => p.voltage,
    //     })
    // )?;

    // Save payments
    let params = powers.clone().into_iter().map(|power| {
        params! {
            "customer_id" => power.name,
            "amount" => power.voltage,
        }
    });

    conn.exec_batch(
        r"INSERT INTO power_meter (power_meter_name, power_meter_voltage)
        VALUES (:power_meter_name, :power_meter_voltage)",
        params,
    ).await?;

    drop(conn);

    // Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;

    // let tty_path = "COM6";
    // let addresses = vec![1,2,3,4,5];
    
    // loop {
    //     for i in 0..addresses.len(){
            
    //         let slave = Slave(addresses[i]);

    //         let mut settings = SerialPortSettings::default();
    //         settings.baud_rate = 19200;
    //         let port = Serial::from_path(tty_path, &settings).unwrap();

    //         let mut ctx = rtu::connect_slave(port, slave).await?;
    //         println!("Reading a sensor value");
    //         let rsp = ctx.read_holding_registers(0x082B, 2).await?;
    //         println!("Sensor value is: {:?}", rsp);
    //     }
    // }
    Ok(())
}
