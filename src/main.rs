#![no_main]
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

use alloc::boxed::Box;
use core::{error::Error, time::Duration};

use vexide::prelude::*;

use crate::drivetrain::Drivetrain;

mod drivetrain;

struct Robot {
    drivetrain: Drivetrain,
    controller: Controller,
}

impl Robot {
    fn new(peripherals: Peripherals) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            drivetrain: Drivetrain::new(
                Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward)?,
                Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward)?,
            ),
            controller: peripherals.primary_controller,
        })
    }
}

impl CompetitionRobot for Robot {
    type Error = Box<dyn Error>;

    async fn driver(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let drive = self.controller.left_stick.y()? as f64;
            let rotate = self.controller.right_stick.x()? as f64;

            self.drivetrain.arcade_drive(drive, rotate)?;

            sleep(Duration::from_millis(20)).await;
        }
    }

    async fn autonomous(&mut self) -> Result<(), Box<dyn Error>> {
        self.drivetrain.tank_drive(0.5, 0.5)?;
        sleep(Duration::from_secs(2)).await;
        self.drivetrain.tank_drive(0.0, 0.0)?;
        Ok(())
    }
}

#[vexide_startup::main]
async fn main(peripherals: Peripherals) {
    if let Err(e) = start(peripherals).await {
        println!("Error: {:?}", e);
    }

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

async fn start(peripherals: Peripherals) -> Result<(), Box<dyn Error>> {
    let robot = Robot::new(peripherals)?;
    robot.compete().await?;
    Ok(())
}
