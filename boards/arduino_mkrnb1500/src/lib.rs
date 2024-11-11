#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
use hal::clock::GenericClockController;
pub use hal::pac;
use hal::sercom::uart::{self, BaudMode, Oversampling};
use hal::time::Hertz;

hal::bsp_peripherals!(
    // SERCOM0 { UartSercom }
    SERCOM2 { GsmSercom } // SERCOM3 { I2cSercom }
                          // SERCOM4 { SpiSercom }
);

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    // Use the `bsp_pins!` macro to define all the pins
    hal::bsp_pins!(
        PA22 {
            /// Digital 0: PWM, TC
            name: d0
        }
        PA23 {
            /// Digital 1: PWM, TC
            name: d1
        }
        PA10 {
            /// Digital 2: PWM, TCC, ADC
            name: d2
        }
        PA11 {
            /// Digital 3: PWM, TCC, ADC
            name: d3
        }
        PB10 {
            /// Digital 4: PWM, TC
            name: d4
        }
        PB11 {
            /// Digital 5: PWM, TC
            name: d5
        }
        PA20 {
            /// Digital 6: PWM, TCC
            name: d6
        }
        PA21 {
            /// Digital 7: PWM, TCC
            name: d7
        }
        PA16 {
            /// SPI MOSI: PWM, TCC
            name: mosi
        }
        PA17 {
            /// SPI SCK
            name: sck
        }
        PA19 {
            /// SPI MISO: PWM, TC
            name: miso
        }
        PA08 {
            /// I2C SDA
            name: sda
            aliases: {
                AlternateC: Sda
            }
        }
        PA09 {
            /// I2C SCL
            name: scl
            aliases: {
                AlternateC: Scl
            }
        }
        PB23 {
            /// RX
            name: rx
            aliases: {
                AlternateC: UartRx
            }
        }
        PB22 {
            /// TX
            name: tx
            aliases: {
                AlternateC: UartTx
            }
        }
        PA02 {
            /// Analog 0: DAC
            name: a0
        }
        PB02 {
            /// Analog 1
            name: a1
        }
        PB03 {
            /// Analog 2
            name: a2
        }
        PA04 {
            /// Analog 3: PWM, TCC
            name: a3
        }
        PA05 {
            /// Analog 4: PWM, TCC
            name: a4
        }
        PA06 {
            /// Analog 5
            name: a5
        }
        PA07 {
            /// Analog 6
            name: a6
        }
        PA24 {
            /// USB D-
            name: usb_n
            aliases: {
                AlternateG: UsbDm
            }
        }
        PA25 {
            /// USB D+
            name: usb_p
            aliases: {
                AlternateG: UsbDp
            }
        }
        PA18 {
            /// USB ID
            name: usb_id
        }
               // Note: you had a comment about this pin also connecting to usbid.
        // Consider adjusting or validating this based on your needs.
        // PA18 {
        //     /// PMIC OTG (also connects to USB ID)
        //     name: pmic_otg
        // }
        PA03 {
            /// AREF
            name: aref
        }
        PA12 {
            /// GSM TX
            name: gsm_tx
            aliases: {
                AlternateC: GsmUartTx
            }
        }
        PA13 {
            /// GSM RX
            name: gsm_rx
            aliases: {
                AlternateC: GsmUartRx
            }
        }
        PA14 {
            /// GSM RTS
            name: gsm_rts
        }
        PA15 {
            /// GSM CTS
            name: gsm_cts
        }
        PA27 {
            /// PMIC IRQ
            name: pmic_irq
        }
        PB08 {
            /// GSM Reset (active low)
            name: gsm_reset_n
        }
        PB09 {
            /// PMIC BAT
            name: adc_battery
        }
        PA28 {
            /// GSM DTR
            name: gsm_dtr
        }
        PA00 {
            /// 32kHz crystal input
            name: xin32
        }
        PA01 {
            /// 32kHz crystal output
            name: xout32
        }

    );
}

pub type GsmUartPads = uart::Pads<GsmSercom, GsmUartRx, GsmUartTx>;
pub type GsmUart = uart::Uart<uart::Config<GsmUartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn gsm_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: GsmSercom,
    pm: &mut pac::PM,
    uart_rx: impl Into<GsmUartRx>,
    uart_tx: impl Into<GsmUartTx>,
) -> GsmUart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

pub use pins::*;
