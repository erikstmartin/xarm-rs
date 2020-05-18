use serialport::SerialPort;

pub enum Error {
    InvalidServo(u8),
    InvalidPosition { servo: u8, position: u16 },
    SerialPortFailure(std::io::Error),
}

pub struct Arm {
    port: Box<dyn SerialPort>,
}

// TODO: Implement additional commands
//  - read servo position
//  - group commands
//  - servo off
//  - get battery voltage
//  -
// TODO: Inverse Kinematics

// TODO: Testing and docs
impl Arm {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Arm { port }
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        let speed = 1000;
        self.set_position([500, 500, 500, 500, 500, 500], speed)
    }

    pub fn set_position(&mut self, pos: [u16; 6], speed: u16) -> Result<(), Error> {
        for (i, v) in pos.iter().enumerate() {
            self.set_servo_position((i as u8) + 1, v.to_owned(), speed)?;
        }
        Ok(())
    }

    pub fn set_servo_position(&mut self, servo: u8, pos: u16, speed: u16) -> Result<(), Error> {
        if servo > 6 {
            return Err(Error::InvalidServo(servo));
        }

        if pos > 1000 {
            return Err(Error::InvalidPosition {
                servo: servo,
                position: pos,
            });
        }

        let mut buf = [0u8; 10];
        let pos_bytes = pos.to_le_bytes();
        let speed_bytes = speed.to_le_bytes();

        buf[0] = 0x55;
        buf[1] = 0x55;
        buf[2] = 0x08; // len
        buf[3] = 0x03; // cmd
        buf[4] = 0x01; // num
        buf[5] = speed_bytes[0]; // time lsb
        buf[6] = speed_bytes[1]; // time msb
        buf[7] = servo.to_le_bytes()[0]; // servo id
        buf[8] = pos_bytes[0]; // pos lsb
        buf[9] = pos_bytes[1]; // pos msb

        let result = self.port.write_all(&buf);
        match result {
            Err(e) => Err(Error::SerialPortFailure(e)),
            Ok(_) => Ok(()),
        }
    }
}
