#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal; // Hardware Abstraction Layer
use hal::clock::GenericClockController;
pub use hal::pac; // Peripheral Access Crate
use hal::sercom::uart::{self, BaudMode, Oversampling};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    Sercom0 { I2cOptSercom }
    Sercom1 { SpiSercom }
    Sercom2 { GsmUartSercom }
    Sercom3 { UartSercom }
);

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    // Use the `bsp_pins!` macro to define all the pins
    hal::bsp_pins!(
        PA22 {
            name: d0
        }
        PA23 {
            name: d1
        }
        PA10 {
            name: d2
        }
        PA11 {
            name: d3
        }
        PB10 {
            name: d4
        }
        PB11 {
            name: d5
        }
        PA20 {
            name: d6
        }
        PA21 {
            name: d7
        }
        PA16 {
            name: d8
            aliases: {
                AlternateC: COPI
            }
        }
        PA17 {
            name: d9
            aliases: {
                AlternateC: Sck
            }
        }
        PA19 {
            name: d10
            aliases: {
                AlternateC: CIPO
            }
        }
        PA08 {
            name: d11
            aliases: {
                AlternateC: Sda
            }
        }
        PA09 {
            name: d12
            aliases: {
                AlternateC: Scl
            }
        }
        PB23 {
            name: d13
            aliases: {
                AlternateC: UartRx
            }
        }
        PB22 {
            name: d14
            aliases: {
                AlternateC: UartTx
            }
        }
        PA02 {
            name: a0
        }
        PB02 {
            name: a1
        }
        PB03 {
            name: a2
        }
        PA04 {
            name: a3
        }
        PA05 {
            name: a4
        }
        PA06 {
            name: a5
        }
        PA07 {
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

pub type GsmUartPads = uart::Pads<GsmUartSercom, GsmUartRx, GsmUartTx>;
pub type GsmUart = uart::Uart<uart::Config<GsmUartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn gsm_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: GsmUartSercom,
    pm: &mut pac::Pm,
    uart_rx: impl Into<GsmUartRx>,
    uart_tx: impl Into<GsmUartTx>,
) -> GsmUart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

pub use pins::*;

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    pm: &mut pac::Pm,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
