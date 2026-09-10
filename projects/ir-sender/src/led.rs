//! Pico 2 W onboard LED (CYW43 WL_GPIO0). No Wi-Fi join; LED only.

use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_rp::Peri;
use embassy_rp::dma;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::Pio;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use static_cell::StaticCell;

use crate::board::Irqs;

static FLASH: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Pulse the onboard LED for 500 ms. Retriggers if called again while on.
pub fn flash() {
    FLASH.signal(());
}

pub async fn start(
    spawner: Spawner,
    pin_pwr: Peri<'static, PIN_23>,
    pin_cs: Peri<'static, PIN_25>,
    pin_dio: Peri<'static, PIN_24>,
    pin_clk: Peri<'static, PIN_29>,
    pio0: Peri<'static, PIO0>,
    dma_ch0: Peri<'static, DMA_CH0>,
) {
    let fw = cyw43::aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = cyw43::aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = cyw43::aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    let pwr = Output::new(pin_pwr, Level::Low);
    let cs = Output::new(pin_cs, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        pin_dio,
        pin_clk,
        dma::Channel::new(dma_ch0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let (_net_device, mut control, runner) =
        cyw43::new(STATE.init(cyw43::State::new()), pwr, spi, fw, nvram).await;
    spawner.spawn(cyw43_task(runner).unwrap());
    control.init(clm.as_ref()).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;
    control.gpio_set(0, false).await;
    spawner.spawn(led_task(control).unwrap());
}

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn led_task(mut control: cyw43::Control<'static>) -> ! {
    loop {
        FLASH.wait().await;
        control.gpio_set(0, true).await;
        loop {
            match select(FLASH.wait(), Timer::after(Duration::from_millis(500))).await {
                Either::First(()) => {}
                Either::Second(()) => break,
            }
        }
        control.gpio_set(0, false).await;
    }
}
