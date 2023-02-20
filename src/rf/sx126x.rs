use std::time::Duration;
use std::io::{self, Write};
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use serialport;
use serialport::SerialPort;

const M0: u8 = 22;
const M1: u8 = 27;
const GET_REG: [u8; 1] = (12 as u8).to_ne_bytes();
const RSSI: bool = false;
pub const ADDR: i32 = 65535;
pub const START_FREQ: i32 = 850;
pub const OFFSET_FREQ: i32 = 18;

pub const UART_BAUDRATE_1200: i32 = 0x00;
pub const UART_BAUDRATE_2400: i32 = 0x20;
pub const UART_BAUDRATE_4800: i32 = 0x40;
pub const UART_BAUDRATE_9600: i32 = 0x60;
pub const UART_BAUDRATE_19200: i32 = 0x80;
pub const UART_BAUDRATE_38400: i32 = 0xA0;
pub const UART_BAUDRATE_57600: i32 = 0xC0;
pub const UART_BAUDRATE_115200: i32 = 0xE0;

pub const PACKAGE_SIZE_240_BYTE: i32 = 0x00;
pub const PACKAGE_SIZE_128_BYTE: i32 = 0x40;
pub const PACKAGE_SIZE_64_BYTE: i32 = 0x80;
pub const PACKAGE_SIZE_32_BYTE: i32 = 0xC0;

pub const POWER_22: i32 = 0x00;
pub const POWER_17: i32 = 0x01;
pub const POWER_13: i32 = 0x02;
pub const POWER_10: i32 = 0x03;

pub const LORA_AIR_SPEED_DIC_1200: i32 = 0x01;
pub const LORA_AIR_SPEED_DIC_2400: i32 = 0x02;
pub const LORA_AIR_SPEED_DIC_4800: i32 = 0x03;
pub const LORA_AIR_SPEED_DIC_9600: i32 = 0x04;
pub const LORA_AIR_SPEED_DIC_19200: i32 = 0x05;
pub const LORA_AIR_SPEED_DIC_38400: i32 = 0x06;
pub const LORA_AIR_SPEED_DIC_62500: i32 = 0x07;

pub struct Sx126x {
    rssi: bool,
    addr: u8,
    freq: u16,
    serial_n: String,
    power: u8,
    gpio: Gpio,
    cfg_reg: Vec<u8>,
    port: Box<dyn SerialPort>,
    relay: bool,
}

impl Sx126x {
    pub fn new(rssi: bool, freq: u16, serial_port: &str, power: u8) -> Self {
        // if the header is 0xC0, then the LoRa register settings is not lost when it poweroff, and 0xC2 will be lost.
        //let cfg_reg: Vec<u8> = vec![0xC0,0x00,0x09,0x00,0x00,0x00,0x62,0x00,0x17,0x43,0x00,0x00];
        let cfg_reg: Vec<u8> = vec![0xc2, 0x00, 0x09, 0x00, 0x00, 0x00, 0x62, 0x00, 0x12, 0x43, 0x00, 0x00];
        let gpio = match Gpio::new() {
            Err(e) => panic!("{}", e),
            Ok(g) => g,
        };

        let mut found = false;
        let ports = serialport::available_ports().expect("No ports");
        for p in ports {
            if p.port_name.eq(serial_port) {
                found = true;
            }
        }
        if !found {
            panic!("Serial port not found: {}", serial_port);
        }
        let port = serialport::new(serial_port, 9600)
            .timeout(Duration::from_millis(1000))
            .open().expect("Failed to open port");


        Self {
            rssi,
            addr: ADDR as u8,
            freq,
            serial_n: serial_port.to_owned(),
            power,
            gpio,
            cfg_reg,
            port,
            relay: false,
        }
    }

    pub fn init(&mut self) {
        // get GPIO pins to enter config mode
        let mut m0_pin: OutputPin = match self.gpio.get(M0) {
            Err(e) => panic!("{}", e),
            Ok(pin) => pin
        }.into_output();
        let mut m1_pin: OutputPin = match self.gpio.get(M1) {
            Err(e) => panic!("{}", e),
            Ok(pin) => pin
        }.into_output();
        // enter config mode
        m0_pin.set_low();
        m1_pin.set_high();


        // these should work just fine but rust complains about overflow on bitshift. Commenting out until I find a solution for this
        // let low_addr: u8 = self.addr & 0xff;
        // let high_addr:u32  = u32::from(self.addr) >> (8 as u8) & 0xff;

        let net_id_temmp: u8 = 0 & 0xff; // set net id as 0 for now because we don;t have a network of boards
        let channel_temp: u8 = (self.freq as u32 - 0) as u8; // 0 <= channel < 84; Actual freq is calculated based on channel to avoid interference. Leaving it as 0 for now
        let buffer_size: u8 = PACKAGE_SIZE_240_BYTE as u8;
        let rssi: u8 = if self.rssi { 0x80 } else { 0x00 };

        // https://www.waveshare.com/wiki/LoRa-HAT-Reg
        //[0xc2, 0x00, 0x09, 0x00, 0x00, 0x00, 0x62, 0x00, 0x12, 0x43, 0x00, 0x00];
        // setting the address as FFFF to disable message address filtering. We want it ALL!
        self.cfg_reg[3] = 0xFF; //00H - addr low
        self.cfg_reg[4] = 0xFF; //01H - addr high
        self.cfg_reg[5] = net_id_temmp; //02H
        self.cfg_reg[6] = (UART_BAUDRATE_9600 + LORA_AIR_SPEED_DIC_2400) as u8; //03H
        self.cfg_reg[7] = buffer_size + self.power + 0x20; //04H
        self.cfg_reg[8] = channel_temp; // 05H
        self.cfg_reg[9] = 0x43 + rssi; // 06H
        // send config command to the serial port
        match self.port.write_all(&(self.cfg_reg)) {
            Ok(..) => println!("message sent"),
            Err(..) => println!("we could not send message"),
        }

        // read response from the board on the serial port
        let mut serial_buffer: Vec<u8> = vec![];
        std::thread::sleep(Duration::from_millis(1000));
        match self.port.read(serial_buffer.as_mut_slice()) {
            Ok(t) => println!("we have contact"),
            Err(e) => eprintln!("{:?}", e)
        };

        // disable config mode
        m0_pin.set_low();
        m1_pin.set_low();
    }

    pub fn read(&mut self) -> ! {
        let mut serial_buf: Vec<u8> = vec![0, 255];
        loop {
            match self.port.read(serial_buf.as_mut_slice()) {
                Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e)
            }
        }
    }
}
