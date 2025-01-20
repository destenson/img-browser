
use std::ops::DerefMut;

use windows::{
    core::*, 
    Data::Xml::Dom::*, 
    Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Storage::FileSystem::*,
};

fn main() {
    let volumes = enumerate_volumes();
    for volume in volumes {
        println!("Found volume: {}", volume);
        match get_volume_info(volume) {
            Some(info) => {
                println!("  Volume name: {}", info.volume_name);
                println!("  File system name: {}", info.file_system_name);
                println!("  Volume serial number: {}", info.volume_serial_number);
                println!("  Max component length: {}", info.max_component_length);
                println!("  File system flags: {}", info.file_system_flags);
            },
            None => {
                eprintln!("  Failed to get volume info");
            }
        }
    }
}

fn enumerate_volumes() -> Vec<String> {
    let mut volume_name = [0u16; MAX_PATH as usize];
    let mut volumes = Vec::new();

    let hfindvolume = unsafe {FindFirstVolumeW(&mut volume_name)}.expect("FindFirstVolumeW failed");
    if hfindvolume == INVALID_HANDLE_VALUE {
        return vec![];
    }

    loop {
        // Process the volume name
        volumes.push(String::from_utf16_lossy(&volume_name));

        // Find next volume
        if unsafe {FindNextVolumeW(hfindvolume, &mut volume_name)}.is_err() {
            break;
        }
    }

    volumes
}

fn enumerate_volume_mount_points(volume: String) -> Vec<String> {
    let mut volume_name = [0u16; MAX_PATH as usize];
    let mut volume = volume.encode_utf16().collect::<Vec<u16>>();
    let mut volumes = Vec::new();

    let hfindvolume = unsafe {FindFirstVolumeMountPointW(PCWSTR(volume.as_mut_ptr()), volume_name.as_mut_slice())}.expect("FindFirstVolumeMountPointW failed");
    if hfindvolume == INVALID_HANDLE_VALUE {
        return vec![];
    }

    loop {
        // Process the volume name
        volumes.push(String::from_utf16_lossy(&volume_name));

        // Find next volume
        if unsafe {FindNextVolumeMountPointW(hfindvolume, volume_name.as_mut_slice())}.is_err() {
            break;
        }
    }

    volumes
}

fn get_volume_info(vol: String) -> Option<VolumeInfo> {
    let mut volume_name_buffer = Some([0u16; MAX_PATH as usize]);
    let mut file_system_name_buffer = Some([0u16; MAX_PATH as usize]);
    let mut volume_serial_number = Some(0u32);
    let mut max_component_length = Some(0u32);
    let mut file_system_flags = Some(0u32);


    let vol = vol + r"\";

    unsafe {
        // use windows_strings::*;
        GetVolumeInformationW(
            PCWSTR(vol.encode_utf16().collect::<Vec<u16>>().as_ptr()),
            volume_name_buffer.as_mut().map(|x|x.as_mut_slice()),
            volume_serial_number.as_mut().map(|x| x as *mut u32),
            max_component_length.as_mut().map(|x|x as *mut u32),
            file_system_flags.as_mut().map(|x|x as *mut u32),
            file_system_name_buffer.as_mut().map(|x|x.as_mut_slice()),
        )
    }.expect("GetVolumeInformationW failed");

    Some(VolumeInfo {
        volume_name: String::from_utf16_lossy(&volume_name_buffer.expect("volume_name_buffer is None")),
        file_system_name: String::from_utf16_lossy(&file_system_name_buffer.expect("file_system_name_buffer is None")),
        volume_serial_number: volume_serial_number.expect("volume_serial_number is None"),
        max_component_length: max_component_length.expect("max_component_length is None"),
        file_system_flags: file_system_flags.expect("file_system_flags is None"),
    })
}

struct VolumeInfo {
    volume_name: String,
    file_system_name: String,
    volume_serial_number: u32,
    max_component_length: u32,
    file_system_flags: u32,
}

// let mut volume_name = [0u16; MAX_PATH];
// let mut handle = FindFirstVolume(volume_name.as_mut_ptr(), volume_name.len() as u32);
// 
// if handle != INVALID_HANDLE_VALUE {
//     loop {
//         // Process the volume name
//         println!("Found volume: {}", String::from_utf16_lossy(&volume_name));
// 
//         // Find next volume
//         if FindNextVolume(handle, volume_name.as_mut_ptr(), volume_name.len() as u32) == 0 {
//             break;
//         }
//     }
// 
//     FindVolumeClose(handle);
// }

// let mut volume_name_buffer = [0u16; MAX_PATH];
// let mut file_system_name_buffer = [0u16; MAX_PATH];
// let mut volume_serial_number = 0;
// let mut max_component_length = 0;
// let mut file_system_flags = 0;

// GetVolumeInformation(
//     volume_path.as_ptr(),
//     volume_name_buffer.as_mut_ptr(),
//     volume_name_buffer.len() as u32,
//     &mut volume_serial_number,
//     &mut max_component_length,
//     &mut file_system_flags,
//     file_system_name_buffer.as_mut_ptr(),
//     file_system_name_buffer.len() as u32
// );
