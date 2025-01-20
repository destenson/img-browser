
use windows::{
    core::*, 
    // Data::Xml::Dom::*, 
    Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Storage::FileSystem::*,
};

fn main() {
    // unsafe {
    //     // let event = CreateEventW(None, true, false, None).expect("CreateEventW failed");
    //     // SetEvent(event);
    //     // WaitForSingleObject(event, 0);
    //     // CloseHandle(event);

    //     // let r1 = MessageBoxA(None, s!("Ansi"), "Caption 1", MB_OK);
    //     // println!("MessageBoxA returned {:?}", r1);
    //     // let r2 = MessageBoxW(None, w!("Wide"), "Caption 2", MB_OK);
    //     // println!("MessageBoxW returned {:?}", r2);
    // }

    unsafe {
        let file_h = CreateFileW(w!(r"C:\tmp\lmstudio-server-log.txt"), GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        ).expect("CreateFileW failed");
        if file_h == INVALID_HANDLE_VALUE {
            eprintln!("Failed to open file");
            return;
        }

        let mut file_info = BY_HANDLE_FILE_INFORMATION::default();
        match GetFileInformationByHandle(file_h, &mut file_info) {
            Ok(_) => {
                println!("File attributes: {}", file_info.dwFileAttributes);
                println!("Creation time: {}", file_info.ftCreationTime.dwLowDateTime);
                println!("Last access time: {}", file_info.ftLastAccessTime.dwLowDateTime);
                println!("Last write time: {}", file_info.ftLastWriteTime.dwLowDateTime);
                println!("File size: {}", (file_info.nFileSizeHigh as u64) << 32 | file_info.nFileSizeLow as u64);
            },
            Err(e) => {
                eprintln!("Failed to get file information: {}", e);
                CloseHandle(file_h).expect("CloseHandle failed");
                return;
            }
        }

    }
}

// #include <windows.h>
// #include <iostream>

// int main() {
//     HANDLE hFile = CreateFile(L"C:\\path\\to\\file", GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
//     if (hFile == INVALID_HANDLE_VALUE) {
//         std::cerr << "Failed to open file" << std::endl;
//         return 1;
//     }

//     BY_HANDLE_FILE_INFORMATION fileInfo;
//     if (!GetFileInformationByHandle(hFile, &fileInfo)) {
//         std::cerr << "Failed to get file information" << std::endl;
//         CloseHandle(hFile);
//         return 1;
//     }

//     std::cout << "File attributes: " << fileInfo.dwFileAttributes << std::endl;
//     std::cout << "Creation time: " << fileInfo.ftCreationTime.dwLowDateTime << std::endl;
//     std::cout << "Last access time: " << fileInfo.ftLastAccessTime.dwLowDateTime << std::endl;
//     std::cout << "Last write time: " << fileInfo.ftLastWriteTime.dwLowDateTime << std::endl;
//     std::cout << "File size: " << (static_cast<ULONGLONG>(fileInfo.nFileSizeHigh) << 32) | fileInfo.nFileSizeLow << std::endl;

//     CloseHandle(hFile);
//     return 0;
// }
