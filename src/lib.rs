use std::ptr;
use winapi::um::winuser::{CDS_FULLSCREEN, CDS_GLOBAL, CDS_UPDATEREGISTRY, DISP_CHANGE_SUCCESSFUL, EnumDisplaySettingsW};
use winapi::um::wingdi::{DEVMODEW};
use winapi::um::winnt::{LONG, WCHAR};
use std::fmt::Display;
use std::mem::zeroed;
use std::cmp::{PartialEq, Eq};

struct MyDevMode(DEVMODEW);

impl PartialEq for MyDevMode {
    fn eq(&self, other: &Self) -> bool {
        self.0.dmPelsWidth == other.0.dmPelsWidth &&
            self.0.dmPelsHeight == other.0.dmPelsHeight
    }
}

impl Eq for MyDevMode {}

fn GetDisplaySettings() -> Vec<DEVMODEW>
{
    let mut display_settings_vec: Vec<DEVMODEW> = Vec::new();


    unsafe {
        let mut dev_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..std::mem::zeroed()
        };

        let mut index = 0;
        let mut prev_setting: DEVMODEW = zeroed();

        loop {
            if EnumDisplaySettingsW(ptr::null(), index, &mut dev_mode) == 0 {
                break;
            }

            if (MyDevMode(prev_setting) != MyDevMode(dev_mode)) {
                prev_setting = dev_mode.clone();
                display_settings_vec.push(dev_mode);
            }

            index += 1;
        }
    }

    for (index, settings) in display_settings_vec.iter().enumerate() {
        println!("Display Settings #{}", index);
        println!("Width: {}", settings.dmPelsWidth);
        println!("Height: {}", settings.dmPelsHeight);
        println!("Bits Per Pixel: {}", settings.dmBitsPerPel);
        println!("Refresh Rate: {}", settings.dmDisplayFrequency);
        println!();
    }

    return display_settings_vec;
}

fn SetDisplaySettings(settings: &DEVMODEW) -> LONG
{
    use winapi::um::winuser::{ChangeDisplaySettingsW};

    unsafe {
        let mut dev_mode: DEVMODEW = *settings;

        let result = ChangeDisplaySettingsW(&mut dev_mode, CDS_GLOBAL | CDS_UPDATEREGISTRY);
        if (result == DISP_CHANGE_SUCCESSFUL)
        {
            println!("Success");
            println!("{}", result)
        } else {
            println!("Success");
        }

        return result;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn GetDisplaySettingsTest(){
        let settings = GetDisplaySettings();
        assert_ne!(settings.len(), 0);
    }

    #[test]
    fn SetDisplaySettingsTest(){
        let settings = GetDisplaySettings();
        let result = SetDisplaySettings(&settings[0]);

        assert_eq!(result, 0);
    }
}
