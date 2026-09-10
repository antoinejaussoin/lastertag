//! Debug flag in RAM plus last flash sector. Default is off.

use embassy_rp::flash::{Blocking, ERASE_SIZE, Error as FlashError, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

/// Pico 2 W onboard flash size.
pub const FLASH_SIZE: usize = 4 * 1024 * 1024;
/// Last 4 KiB sector, well above the firmware image.
const CONFIG_OFFSET: u32 = (FLASH_SIZE - ERASE_SIZE) as u32;

/// "SNDB" — not the temperature/capture Wi-Fi blob.
const MAGIC: u32 = 0x534E_4442;

pub type ConfigFlash = Flash<'static, FLASH, Blocking, FLASH_SIZE>;

static SETTINGS: Mutex<CriticalSectionRawMutex, Config> = Mutex::new(Config::empty());
static CHANGED: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[derive(Clone, Copy)]
pub struct Config {
    /// USB `debug on` + `save`. Button then toggles DC / 1 s NEC.
    pub debug: bool,
}

impl Config {
    pub const fn empty() -> Self {
        Self { debug: false }
    }
}

pub async fn snapshot() -> Config {
    *SETTINGS.lock().await
}

pub async fn replace(cfg: Config) {
    *SETTINGS.lock().await = cfg;
}

pub async fn update(f: impl FnOnce(&mut Config)) {
    f(&mut *SETTINGS.lock().await);
}

pub fn notify() {
    CHANGED.signal(());
}

pub async fn wait_change() {
    CHANGED.wait().await;
}

pub fn load_flash(flash: &mut ConfigFlash) -> Config {
    let mut buf = [0u8; 256];
    if flash.blocking_read(CONFIG_OFFSET, &mut buf).is_err() {
        return Config::empty();
    }
    decode(&buf).unwrap_or_else(Config::empty)
}

pub fn save_flash(flash: &mut ConfigFlash, cfg: &Config) -> Result<(), FlashError> {
    let buf = encode(cfg);
    flash.blocking_erase(CONFIG_OFFSET, CONFIG_OFFSET + ERASE_SIZE as u32)?;
    flash.blocking_write(CONFIG_OFFSET, &buf)
}

pub fn erase_flash(flash: &mut ConfigFlash) -> Result<(), FlashError> {
    flash.blocking_erase(CONFIG_OFFSET, CONFIG_OFFSET + ERASE_SIZE as u32)
}

fn encode(cfg: &Config) -> [u8; 256] {
    let mut buf = [0u8; 256];
    buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    buf[8] = u8::from(cfg.debug);
    let crc = checksum(&buf[8..]);
    buf[4..8].copy_from_slice(&crc.to_le_bytes());
    buf
}

fn decode(buf: &[u8; 256]) -> Option<Config> {
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    if magic != MAGIC {
        return None;
    }
    let crc = u32::from_le_bytes(buf[4..8].try_into().ok()?);
    if crc != checksum(&buf[8..]) {
        return None;
    }
    Some(Config {
        debug: buf[8] != 0,
    })
}

fn checksum(data: &[u8]) -> u32 {
    let mut h = 2166136261u32;
    for b in data {
        h ^= u32::from(*b);
        h = h.wrapping_mul(16777619);
    }
    h
}
