use std::io::{self, Read};

use chrono::{DateTime, Timelike, Utc};
use hidapi::{DeviceInfo, HidApi};
use image::{GrayImage, Luma, Pixel};
use imageproc::{drawing::draw_text_mut};
use rusttype::{Font, Scale};
use crate::content::content::DisplayContent;

mod content;

const VENDOR_ID: u16 = 0xfeed;
const PRODUCT_ID: u16 = 0x6060;
const USAGE_PAGE: u16 = 0xff60;

fn is_my_device(device: &DeviceInfo) -> bool {
    device.vendor_id() == VENDOR_ID
        && device.product_id() == PRODUCT_ID
        && device.usage_page() == USAGE_PAGE
}

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut buffer).unwrap();
    let buffer = buffer.trim();
    let content: DisplayContent = serde_json::from_str(&buffer).unwrap();

    let mut image = GrayImage::new(128,64);

    let icon_font = Vec::from(include_bytes!("../icons/fonts/w-icon.ttf") as &[u8]);
    let icon_font = Font::try_from_vec(icon_font).unwrap();

    let text_font = Vec::from(include_bytes!("../iA-Fonts/iA Writer Mono/Static/iAWriterMonoS-Regular.ttf") as &[u8]);
    let text_font = Font::try_from_vec(text_font).unwrap();

    let utc: DateTime<Utc> = Utc::now();  
    let padding = ((utc.hour() + utc.minute()) % 10 + 1) as f32;
    let padding = if padding < 2.0 { 2 as f32 } else { padding };
    let height = 13.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    if let Some(icon) = content.icon_top_left {
        draw_text_mut(&mut image, Luma([255]), padding as u32, padding as u32, scale, &icon_font, &icon.to_string());
    }

    if let Some(text) = content.text_top_left {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + height as u32 + padding as u32, padding as u32, scale, &text_font, &text);
    }

    if let Some(icon) = content.icon_top_right {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + 62, padding as u32, scale, &icon_font, &icon.to_string());
    }

    if let Some(text) = content.text_top_right {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + height as u32 + padding as u32 + 62, padding as u32, scale, &text_font, &text);
    }

    if let Some(icon) = content.icon_bottom_left {
        draw_text_mut(&mut image, Luma([255]), padding as u32, padding as u32 + 32, scale, &icon_font, &icon.to_string());
    }

    if let Some(text) = content.text_bottom_left {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + height as u32 + padding as u32, padding as u32 + 32, scale, &text_font, &text);
    }

    if let Some(icon) = content.icon_bottom_right {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + 62, padding as u32 + 32, scale, &icon_font, &icon.to_string());
    }

    if let Some(text) = content.text_bottom_right {
        draw_text_mut(&mut image, Luma([255]), padding as u32 + height as u32 + padding as u32 + 62, padding as u32 + 32, scale, &text_font, &text);
    }

    let mut data : Vec<u8> = vec![];

    for p in image.pixels() {
        data.push(p.channels()[0]);
    }

    let api = HidApi::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let device = api
    .device_list()
    .find(|device| is_my_device(device))
    .unwrap_or_else(|| {
      eprintln!("Could not find keyboard");
      std::process::exit(1);
    })
    .open_device(&api).unwrap_or_else(|_| {
      eprintln!("Could not connect to keyboard");
      std::process::exit(1);
    });

    let _ = device.write(&data);
}