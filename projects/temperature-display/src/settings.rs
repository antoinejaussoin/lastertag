//! Provisioned Wi-Fi and POST target: RAM copy plus last flash sector.

use embassy_rp::flash::{Blocking, ERASE_SIZE, Error as FlashError, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use heapless::String;

/// Pico 2 W onboard flash size.
pub const FLASH_SIZE: usize = 4 * 1024 * 1024;
/// Last 4 KiB sector, well above the firmware image.
const CONFIG_OFFSET: u32 = (FLASH_SIZE - ERASE_SIZE) as u32;

const MAGIC: u32 = 0x5445_4D50; // "TEMP"
const SSID_MAX: usize = 32;
const PSK_MAX: usize = 64;
const SERVER_MAX: usize = 128;

pub type ConfigFlash = Flash<'static, FLASH, Blocking, FLASH_SIZE>;

static SETTINGS: Mutex<CriticalSectionRawMutex, NetConfig> = Mutex::new(NetConfig::empty());
static JOIN: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// In-RAM copy of the provisioned settings.
#[derive(Clone)]
pub struct NetConfig {
    pub ssid: String<SSID_MAX>,
    pub psk: String<PSK_MAX>,
    pub server: String<SERVER_MAX>,
}

impl NetConfig {
    pub const fn empty() -> Self {
        Self {
            ssid: String::new(),
            psk: String::new(),
            server: String::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        !self.ssid.is_empty() && !self.server.is_empty()
    }
}

/// Host, TCP port, and URL path parsed from the `server` setting.
pub struct ServerTarget {
    pub host: String<64>,
    pub port: u16,
    pub path: String<64>,
}

pub async fn snapshot() -> NetConfig {
    SETTINGS.lock().await.clone()
}

pub async fn replace(cfg: NetConfig) {
    *SETTINGS.lock().await = cfg;
}

pub async fn update(f: impl FnOnce(&mut NetConfig)) {
    f(&mut *SETTINGS.lock().await);
}

pub fn request_rejoin() {
    JOIN.signal(());
}

pub async fn wait_rejoin() {
    JOIN.wait().await;
}

/// Parse `host:port/path`, `http://host:port/path`, or `host/path` (port 80).
pub fn parse_server(raw: &str) -> Option<ServerTarget> {
    let mut s = raw.trim();
    if let Some(rest) = s.strip_prefix("http://") {
        s = rest;
    }
    if s.is_empty() || s.len() > 128 {
        return None;
    }

    let (hostport, path) = match s.find('/') {
        Some(i) => (&s[..i], &s[i..]),
        None => (s, "/"),
    };
    if hostport.is_empty() || path.is_empty() {
        return None;
    }

    let (host, port) = split_host_port(hostport)?;
    let mut host_s = String::new();
    let mut path_s = String::new();
    host_s.push_str(host).ok()?;
    path_s.push_str(path).ok()?;
    Some(ServerTarget {
        host: host_s,
        port,
        path: path_s,
    })
}

fn split_host_port(hostport: &str) -> Option<(&str, u16)> {
    if let Some(colon) = hostport.rfind(':') {
        let host = &hostport[..colon];
        let port_s = &hostport[colon + 1..];
        if host.is_empty() {
            return None;
        }
        if !port_s.is_empty() && port_s.bytes().all(|b| b.is_ascii_digit()) {
            let port: u16 = port_s.parse().ok()?;
            return Some((host, port));
        }
    }
    Some((hostport, 80))
}

pub fn load_flash(flash: &mut ConfigFlash) -> NetConfig {
    let mut buf = [0u8; 256];
    if flash.blocking_read(CONFIG_OFFSET, &mut buf).is_err() {
        return NetConfig::empty();
    }
    decode(&buf).unwrap_or_else(NetConfig::empty)
}

pub fn save_flash(flash: &mut ConfigFlash, cfg: &NetConfig) -> Result<(), FlashError> {
    let buf = encode(cfg);
    flash.blocking_erase(CONFIG_OFFSET, CONFIG_OFFSET + ERASE_SIZE as u32)?;
    flash.blocking_write(CONFIG_OFFSET, &buf)
}

pub fn erase_flash(flash: &mut ConfigFlash) -> Result<(), FlashError> {
    flash.blocking_erase(CONFIG_OFFSET, CONFIG_OFFSET + ERASE_SIZE as u32)
}

fn encode(cfg: &NetConfig) -> [u8; 256] {
    let mut buf = [0u8; 256];
    buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    write_field(&mut buf, 8, cfg.ssid.as_bytes());
    write_field(&mut buf, 8 + 1 + SSID_MAX, cfg.psk.as_bytes());
    write_field(
        &mut buf,
        8 + 1 + SSID_MAX + 1 + PSK_MAX,
        cfg.server.as_bytes(),
    );
    let crc = checksum(&buf[8..]);
    buf[4..8].copy_from_slice(&crc.to_le_bytes());
    buf
}

fn decode(buf: &[u8; 256]) -> Option<NetConfig> {
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    if magic != MAGIC {
        return None;
    }
    let crc = u32::from_le_bytes(buf[4..8].try_into().ok()?);
    if crc != checksum(&buf[8..]) {
        return None;
    }
    let mut cfg = NetConfig::empty();
    cfg.ssid = read_field(buf, 8, SSID_MAX)?;
    cfg.psk = read_field(buf, 8 + 1 + SSID_MAX, PSK_MAX)?;
    cfg.server = read_field(buf, 8 + 1 + SSID_MAX + 1 + PSK_MAX, SERVER_MAX)?;
    Some(cfg)
}

fn write_field(buf: &mut [u8], offset: usize, bytes: &[u8]) {
    buf[offset] = bytes.len() as u8;
    buf[offset + 1..offset + 1 + bytes.len()].copy_from_slice(bytes);
}

fn read_field<const N: usize>(buf: &[u8], offset: usize, max: usize) -> Option<String<N>> {
    let len = buf[offset] as usize;
    if len > max || offset + 1 + len > buf.len() {
        return None;
    }
    let s = core::str::from_utf8(&buf[offset + 1..offset + 1 + len]).ok()?;
    let mut out = String::new();
    out.push_str(s).ok()?;
    Some(out)
}

fn checksum(data: &[u8]) -> u32 {
    let mut h = 2166136261u32;
    for b in data {
        h ^= u32::from(*b);
        h = h.wrapping_mul(16777619);
    }
    h
}
