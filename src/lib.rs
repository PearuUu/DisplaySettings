use std::ptr;
use winapi::um::winuser::{CDS_GLOBAL, CDS_UPDATEREGISTRY, DISP_CHANGE_SUCCESSFUL, EnumDisplaySettingsW};
use winapi::um::wingdi::{DEVMODEW};
use winapi::um::winnt::{LONG};
use std::fmt::Display;
use std::mem::zeroed;
use std::cmp::{PartialEq, Eq};

mod types {
    pub mod DisplaySettingsInput;
}

pub use types::DisplaySettingsInput::*;

struct MyDevMode(DEVMODEW);
impl PartialEq for MyDevMode {
    fn eq(&self, other: &Self) -> bool {
        self.0.dmPelsWidth == other.0.dmPelsWidth &&
            self.0.dmPelsHeight == other.0.dmPelsHeight
    }
}
impl Eq for MyDevMode {}

pub fn get_display_settings() -> Vec<DisplaySettingsInput>
{
    let mut display_settings_vec: Vec<DisplaySettingsInput> = Vec::new();

    get_display_settings_core().iter().for_each(|setting| {
        display_settings_vec.push(DisplaySettingsInput {
            width: setting.dmPelsWidth,
            height: setting.dmPelsHeight,
            refresh_rate: setting.dmDisplayFrequency
        });
    });

    return display_settings_vec;
}

fn get_display_settings_core() -> Vec<DEVMODEW>
{
    let mut display_settings_vec: Vec<DEVMODEW> = Vec::new();


    unsafe {
        let mut dev_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..zeroed()
        };

        let mut index = 0;
        let mut prev_setting: DEVMODEW = zeroed();

        loop {
            if EnumDisplaySettingsW(ptr::null(), index, &mut dev_mode) == 0 {
                break;
            }

            if MyDevMode(prev_setting) != MyDevMode(dev_mode) {
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

fn get_dev_mode_w() -> DEVMODEW
{
    unsafe{
        let mut dev_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..zeroed()
        };

        EnumDisplaySettingsW(ptr::null(), 0, &mut dev_mode);

        return dev_mode;
    }


}

pub fn set_display_settings(settings: DisplaySettingsInput) -> LONG
{
    let mut dev_mode = get_dev_mode_w();

    dev_mode.dmPelsWidth = settings.width;
    dev_mode.dmPelsHeight = settings.height;
    dev_mode.dmDisplayFrequency = settings.refresh_rate;

    let result = set_display_settings_core(&dev_mode);

    return result;
}

fn set_display_settings_core(settings: &DEVMODEW) -> LONG
{
    use winapi::um::winuser::{ChangeDisplaySettingsW};

    unsafe {
        let mut dev_mode: DEVMODEW = *settings;

        let result = ChangeDisplaySettingsW(&mut dev_mode, CDS_GLOBAL | CDS_UPDATEREGISTRY);
        if result == DISP_CHANGE_SUCCESSFUL
        {
            println!("Success");
            println!("{}", result)
        } else {
            println!("Success");
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_display_settings_test(){
        let settings = get_display_settings();
        assert_ne!(settings.len(), 0);
    }

    #[test]
    fn set_display_settings_test(){
        // let settings = DisplaySettings {
        //     width: 1920,
        //     height: 1080,
        //     refresh_rate: 60
        // };
        let settings = get_display_settings();
        let setting = settings[0].clone();
        let result = set_display_settings(setting);

        assert_eq!(result, 0);
    }
}
