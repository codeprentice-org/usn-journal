#[cfg(test)]
mod tests {
    
    use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::System::IO::*,
    Win32::System::Ioctl::*,
    Win32::System::Threading::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::Storage::FileSystem::*,
    };
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn test()
    {
        let vol = iwin::create_file("\\\\.\\C:", GENERIC_READ, FILE_SHARE_READ  | FILE_SHARE_WRITE, OPEN_ALWAYS, FILE_FLAG_BACKUP_SEMANTICS).unwrap();

        let mut journal_data = USN_JOURNAL_DATA_V1{
            UsnJournalID: 0,
            FirstUsn: 0,
            NextUsn: 0,
            LowestValidUsn: 0,
            MaxUsn: 0,
            MaximumSize: 0,
            AllocationDelta: 0,
            MinSupportedMajorVersion: 0,
            MaxSupportedMajorVersion: 0,

        };

        let mut read_data = READ_USN_JOURNAL_DATA_V0 {
            StartUsn: 0,
            ReasonMask: 0xFFFF_FFFF,
            ReturnOnlyOnClose: 0,
            Timeout: 0,
            BytesToWaitFor: 0,
            UsnJournalID: 0,    
        };

        let mut usn_record = USN_RECORD_V2
        {
            RecordLength: 0,
            MajorVersion: 0,
            MinorVersion: 0,
            FileReferenceNumber: 0,
            ParentFileReferenceNumber: 0,
            Usn: 0,
            TimeStamp: 0,
            Reason: 0,
            SourceInfo: 0,
            SecurityId: 0,
            FileAttributes: 0,
            FileNameOffset: 0,
            FileNameLength: 0,
            FileName: [0],
        };

        let mut dw_bytes = 0;
        let mut buffer = [0u8; 4096];


        let res = unsafe { DeviceIoControl( vol, 
            FSCTL_QUERY_USN_JOURNAL, 
            std::ptr::null(),
            0,
            std::ptr::addr_of_mut!(journal_data).cast::<std::ffi::c_void>(),
            std::mem::size_of::<USN_JOURNAL_DATA_V0>().try_into().unwrap(),
            & mut dw_bytes,
            std::ptr::null_mut()) };


        println!("DeviceIoControl Returned {res}");
        println!("Journal ID: {}", journal_data.UsnJournalID);
        println!("FirstUsn: {}", journal_data.FirstUsn);

        read_data.UsnJournalID = journal_data.UsnJournalID;


        buffer.fill(0);

        let res = unsafe {
            DeviceIoControl( vol, 
                FSCTL_READ_USN_JOURNAL, 
                std::ptr::addr_of!(read_data).cast::<std::ffi::c_void>(),
                std::mem::size_of::<READ_USN_JOURNAL_DATA_V0>().try_into().unwrap(),
                buffer.as_mut_ptr().cast::<std::ffi::c_void>(),
                buffer.len().try_into().unwrap(),
                & mut dw_bytes,
                std::ptr::null_mut())
        };



        assert!(res != 0, "Error DeviceIOControl: {}", iwin::get_last_error());

        // let usn_record = unsafe { buffer[8..].as_ptr().cast::<USN_RECORD_V2>().as_ref().unwrap() };
        println!("dw_bytes = {dw_bytes}");
        let mut bytes_remaining =  dw_bytes - 8;
        println!("bytes remaining: {bytes_remaining}");

        let mut usn_start = 8usize;    

        while bytes_remaining > 0
        {
            let usn_record: &USN_RECORD_V2 = {
                let usn_end = usn_start + std::mem::size_of::<USN_RECORD_V2>();

                unsafe { buffer[usn_start..usn_end].as_ptr().cast::<USN_RECORD_V2>().as_ref().unwrap() }
            }; 

            println!("FilenameOffset: {}", usn_record.FileNameOffset);
            println!("FilenameLength: {}", usn_record.FileNameLength);
            println!("Id {}", usn_record.Usn);
            let name = unsafe { std::slice::from_raw_parts(usn_record.FileName.as_ptr() as *const _, (usn_record.FileNameLength / 2).try_into().unwrap()) };
            let name = String::from_utf16(name).unwrap();

            println!("name: {name}");

            bytes_remaining -= usn_record.RecordLength;
            usn_start += usn_record.RecordLength as usize;
        } 
    }
}
