//  _  _         ___ ___
// | \| |___ ___| _ \ __|  Header-only and lightweight PE utility library.
// | .` / -_) _ \  _/ _|   version 0.0.0
// |_|\_\___\___/_| |___|  https://github.com/pagefaultcc/NeoPE
//
// SPDX-FileCopyrightText. 2026 - 2027 pagefault.cc. <https://github.com/pagefaultcc>
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// #pragma region WINDOWS_DEFINITIONS

pub type BYTE = u8;
pub type CHAR = i8;
pub type UCHAR = u8;
pub type SHORT = i16;
pub type USHORT = u16;
pub type WORD = u16;
pub type INT = i32;
pub type UINT = u32;
pub type LONG = i32;
pub type ULONG = u32;
pub type DWORD = u32;
pub type LONGLONG = i64;
pub type ULONGLONG = u64;
pub type DWORDLONG = u64;
pub type DWORD32 = u32;
pub type DWORD64 = u64;

pub type INT8 = i8;
pub type INT16 = i16;
pub type INT32 = i32;
pub type INT64 = i64;

pub type UINT8 = u8;
pub type UINT16 = u16;
pub type UINT32 = u32;
pub type UINT64 = u64;

pub type LONG32 = i32;
pub type LONG64 = i64;
pub type ULONG32 = u32;
pub type ULONG64 = u64;

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;

pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6;
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize = 13;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;

//0x40 bytes (sizeof)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: USHORT,                                                         //0x0
    pub e_cblp: USHORT,                                                          //0x2
    pub e_cp: USHORT,                                                            //0x4
    pub e_crlc: USHORT,                                                          //0x6
    pub e_cparhdr: USHORT,                                                       //0x8
    pub e_minalloc: USHORT,                                                      //0xa
    pub e_maxalloc: USHORT,                                                      //0xc
    pub e_ss: USHORT,                                                            //0xe
    pub e_sp: USHORT,                                                            //0x10
    pub e_csum: USHORT,                                                          //0x12
    pub e_ip: USHORT,                                                            //0x14
    pub e_cs: USHORT,                                                            //0x16
    pub e_lfarlc: USHORT,                                                        //0x18
    pub e_ovno: USHORT,                                                          //0x1a
    pub e_res: [USHORT; 4],                                                      //0x1c
    pub e_oemid: USHORT,                                                         //0x24
    pub e_oeminfo: USHORT,                                                       //0x26
    pub e_res2: [USHORT; 10],                                                     //0x28
    pub e_lfanew: LONG,                                                          //0x3c
}
pub type PIMAGE_DOS_HEADER = *const IMAGE_DOS_HEADER;

#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_SECTION_HEADER_MISC {
    pub PhysicalAddress: ULONG,                                              //0x8
    pub VirtualSize: ULONG,                                                  //0x8
}

impl std::fmt::Debug for IMAGE_SECTION_HEADER_MISC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Misc")
            .field("VirtualSize", unsafe { &self.VirtualSize })
            .finish()
    }
}

//0x28 bytes (sizeof)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [UCHAR; 8],                                                   //0x0
    pub Misc: IMAGE_SECTION_HEADER_MISC,                                    //0x8
    pub VirtualAddress: ULONG,                                                   //0xc
    pub SizeOfRawData: ULONG,                                                    //0x10
    pub PointerToRawData: ULONG,                                                 //0x14
    pub PointerToRelocations: ULONG,                                             //0x18
    pub PointerToLinenumbers: ULONG,                                             //0x1c
    pub NumberOfRelocations: USHORT,                                             //0x20
    pub NumberOfLinenumbers: USHORT,                                             //0x22
    pub Characteristics: ULONG,                                                  //0x24
}
pub type PIMAGE_SECTION_HEADER = *const IMAGE_SECTION_HEADER;

//0x14 bytes (sizeof)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: USHORT,                                                         //0x0
    pub NumberOfSections: USHORT,                                                //0x2
    pub TimeDateStamp: ULONG,                                                    //0x4
    pub PointerToSymbolTable: ULONG,                                             //0x8
    pub NumberOfSymbols: ULONG,                                                  //0xc
    pub SizeOfOptionalHeader: USHORT,                                            //0x10
    pub Characteristics: USHORT,                                                 //0x12
}
pub type PIMAGE_FILE_HEADER = *const IMAGE_FILE_HEADER;

//0x8 bytes (sizeof)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: ULONG,                                                   //0x0
    pub Size: ULONG,                                                             //0x4
}

//0xf0 bytes (sizeof)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: USHORT,                                                           //0x0
    pub MajorLinkerVersion: UCHAR,                                               //0x2
    pub MinorLinkerVersion: UCHAR,                                               //0x3
    pub SizeOfCode: ULONG,                                                       //0x4
    pub SizeOfInitializedData: ULONG,                                            //0x8
    pub SizeOfUninitializedData: ULONG,                                          //0xc
    pub AddressOfEntryPoint: ULONG,                                              //0x10
    pub BaseOfCode: ULONG,                                                       //0x14
    pub ImageBase: ULONGLONG,                                                    //0x18
    pub SectionAlignment: ULONG,                                                 //0x20
    pub FileAlignment: ULONG,                                                    //0x24
    pub MajorOperatingSystemVersion: USHORT,                                     //0x28
    pub MinorOperatingSystemVersion: USHORT,                                     //0x2a
    pub MajorImageVersion: USHORT,                                               //0x2c
    pub MinorImageVersion: USHORT,                                               //0x2e
    pub MajorSubsystemVersion: USHORT,                                           //0x30
    pub MinorSubsystemVersion: USHORT,                                           //0x32
    pub Win32VersionValue: ULONG,                                                //0x34
    pub SizeOfImage: ULONG,                                                      //0x38
    pub SizeOfHeaders: ULONG,                                                    //0x3c
    pub CheckSum: ULONG,                                                         //0x40
    pub Subsystem: USHORT,                                                       //0x44
    pub DllCharacteristics: USHORT,                                              //0x46
    pub SizeOfStackReserve: ULONGLONG,                                           //0x48
    pub SizeOfStackCommit: ULONGLONG,                                            //0x50
    pub SizeOfHeapReserve: ULONGLONG,                                            //0x58
    pub SizeOfHeapCommit: ULONGLONG,                                             //0x60
    pub LoaderFlags: ULONG,                                                      //0x68
    pub NumberOfRvaAndSizes: ULONG,                                              //0x6c
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],                              //0x70
}
pub type PIMAGE_OPTIONAL_HEADER64 = *const IMAGE_OPTIONAL_HEADER64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: ULONG,                                                        //0x0
    pub FileHeader: IMAGE_FILE_HEADER,                                           //0x4
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,                                 //0x18
}
pub type PIMAGE_NT_HEADERS64 = *const IMAGE_NT_HEADERS64;

pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS64;
pub type PIMAGE_NT_HEADERS = PIMAGE_NT_HEADERS64;

pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER64;
pub type PIMAGE_OPTIONAL_HEADER = PIMAGE_OPTIONAL_HEADER64;

pub const IMAGE_DOS_SIGNATURE: USHORT = 0x5A4D;      // MZ
pub const IMAGE_NT_SIGNATURE: ULONG = 0x00004550;    // PE00

// Directory types
#[repr(C)]
pub struct IMAGE_EXPORT_DIRECTORY { _opaque: [u8; 0] }
pub type PIMAGE_EXPORT_DIRECTORY = *const IMAGE_EXPORT_DIRECTORY;

#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR { _opaque: [u8; 0] }
pub type PIMAGE_IMPORT_DESCRIPTOR = *const IMAGE_IMPORT_DESCRIPTOR;

#[repr(C)]
pub struct IMAGE_RESOURCE_DIRECTORY { _opaque: [u8; 0] }
pub type PIMAGE_RESOURCE_DIRECTORY = *const IMAGE_RESOURCE_DIRECTORY;

#[repr(C)]
pub struct IMAGE_RUNTIME_FUNCTION_ENTRY { _opaque: [u8; 0] }
pub type PIMAGE_RUNTIME_FUNCTION_ENTRY = *const IMAGE_RUNTIME_FUNCTION_ENTRY;

#[repr(C)]
pub struct WIN_CERTIFICATE { _opaque: [u8; 0] }
pub type LPWIN_CERTIFICATE = *const WIN_CERTIFICATE;

#[repr(C)]
pub struct IMAGE_BASE_RELOCATION { _opaque: [u8; 0] }
pub type PIMAGE_BASE_RELOCATION = *const IMAGE_BASE_RELOCATION;

#[repr(C)]
pub struct IMAGE_DEBUG_DIRECTORY { _opaque: [u8; 0] }
pub type PIMAGE_DEBUG_DIRECTORY = *const IMAGE_DEBUG_DIRECTORY;

#[repr(C)]
pub struct IMAGE_ARCHITECTURE_HEADER { _opaque: [u8; 0] }
pub type PIMAGE_ARCHITECTURE_HEADER = *const IMAGE_ARCHITECTURE_HEADER;

pub type PVOID = *const std::ffi::c_void;

#[repr(C)]
pub struct IMAGE_TLS_DIRECTORY { _opaque: [u8; 0] }
pub type PIMAGE_TLS_DIRECTORY = *const IMAGE_TLS_DIRECTORY;

#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY { _opaque: [u8; 0] }
pub type PIMAGE_LOAD_CONFIG_DIRECTORY = *const IMAGE_LOAD_CONFIG_DIRECTORY;

#[repr(C)]
pub struct IMAGE_BOUND_IMPORT_DESCRIPTOR { _opaque: [u8; 0] }
pub type PIMAGE_BOUND_IMPORT_DESCRIPTOR = *const IMAGE_BOUND_IMPORT_DESCRIPTOR;

#[repr(C)]
pub struct IMAGE_THUNK_DATA { _opaque: [u8; 0] }
pub type PIMAGE_THUNK_DATA = *const IMAGE_THUNK_DATA;

#[repr(C)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR { _opaque: [u8; 0] }
pub type PIMAGE_DELAYLOAD_DESCRIPTOR = *const IMAGE_DELAYLOAD_DESCRIPTOR;

#[repr(C)]
pub struct IMAGE_COR20_HEADER { _opaque: [u8; 0] }
pub type PIMAGE_COR20_HEADER = *const IMAGE_COR20_HEADER;

// #pragma endregion

#[inline]
pub fn PE_FAILED(error: EError) -> bool {
    error != EError::E_SUCCESS
}

#[inline]
pub fn pe_failed(error: EError) -> bool {
    PE_FAILED(error)
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EError {
    E_NONE = 0,

    E_SUCCESS,
    E_TOO_SMALL_BINARY,
    E_INVALID_BINARY,
    E_INVALID_DOS_SIGNATURE,
    E_INVALID_HEADER_SIGNATURE,

    E_MAX,
}

pub fn ErrorToStr(error: EError) -> &'static str {
    match error {
        EError::E_SUCCESS => "E_SUCCESS",
        EError::E_TOO_SMALL_BINARY => "E_TOO_SMALL_BINARY",
        EError::E_INVALID_DOS_SIGNATURE => "E_INVALID_DOS_SIGNATURE",
        EError::E_INVALID_HEADER_SIGNATURE => "E_INVALID_HEADER_SIGNATURE",
        _ => "INVALID_ERROR_CODE",
    }
}

pub fn error_to_str(error: EError) -> &'static str {
    ErrorToStr(error)
}

#[derive(Debug, Clone)]
pub struct PESection {
    m_imgSectionHeader: IMAGE_SECTION_HEADER,
}

impl PESection {
    pub fn new(section_header: IMAGE_SECTION_HEADER) -> Self {
        Self {
            m_imgSectionHeader: section_header,
        }
    }

    pub fn GetName(&self) -> &str {
        let raw_name = &self.m_imgSectionHeader.Name;
        let len = raw_name.iter().position(|&c| c == 0).unwrap_or(IMAGE_SIZEOF_SHORT_NAME);
        if len == 0 {
            return "INVALID";
        }
        std::str::from_utf8(&raw_name[..len]).unwrap_or("INVALID")
    }

    pub fn get_name(&self) -> &str {
        self.GetName()
    }

    pub fn HasCharacteristic(&self, characteristic: DWORD) -> bool {
        (self.m_imgSectionHeader.Characteristics & characteristic) == characteristic
    }

    pub fn has_characteristic(&self, characteristic: DWORD) -> bool {
        self.HasCharacteristic(characteristic)
    }

    pub fn AddCharacteristic(&mut self, characteristic: DWORD) {
        self.m_imgSectionHeader.Characteristics |= characteristic;
    }

    pub fn add_characteristic(&mut self, characteristic: DWORD) {
        self.AddCharacteristic(characteristic);
    }

    pub fn RemoveCharacteristic(&mut self, characteristic: DWORD) {
        self.m_imgSectionHeader.Characteristics &= !characteristic;
    }

    pub fn remove_characteristic(&mut self, characteristic: DWORD) {
        self.RemoveCharacteristic(characteristic);
    }

    pub fn GetSectionHeader(&self) -> PIMAGE_SECTION_HEADER {
        &self.m_imgSectionHeader as PIMAGE_SECTION_HEADER
    }

    pub fn get_section_header(&self) -> PIMAGE_SECTION_HEADER {
        self.GetSectionHeader()
    }

    pub fn section_header(&self) -> &IMAGE_SECTION_HEADER {
        &self.m_imgSectionHeader
    }
}

#[derive(Debug, Clone)]
pub struct PEExportedFunction<'a> {
    m_szName: &'a str,
    m_iOrdinal: u32,
    m_iRva: u32,
    m_bCouldBeBadExportedFunction: bool,
}

impl<'a> PEExportedFunction<'a> {
    pub fn new(
        sz_name: &'a str,
        i_ordinal: u32,
        i_rva: u32,
        b_could_be_bad_exported_function: bool,
    ) -> Self {
        Self {
            m_szName: sz_name,
            m_iOrdinal: i_ordinal,
            m_iRva: i_rva,
            m_bCouldBeBadExportedFunction: b_could_be_bad_exported_function,
        }
    }

    pub fn new_forwarded(
        sz_name: &'a str,
        i_ordinal: u32,
        i_rva: u32,
        _c_forwarded: PEExportedFunction<'a>,
        b_could_be_bad_exported_function: bool,
    ) -> Self {
        Self {
            m_szName: sz_name,
            m_iOrdinal: i_ordinal,
            m_iRva: i_rva,
            m_bCouldBeBadExportedFunction: b_could_be_bad_exported_function,
        }
    }

    pub fn name(&self) -> &'a str {
        self.m_szName
    }

    pub fn ordinal(&self) -> u32 {
        self.m_iOrdinal
    }

    pub fn rva(&self) -> u32 {
        self.m_iRva
    }

    pub fn could_be_bad_exported_function(&self) -> bool {
        self.m_bCouldBeBadExportedFunction
    }
}

pub struct PE {
    // PE
    m_imgDosHeader: PIMAGE_DOS_HEADER,
    m_imgNtHeaders: PIMAGE_NT_HEADERS,
    m_imgOptionalHeader: PIMAGE_OPTIONAL_HEADER,
    m_vecSections: Vec<PESection>,

    m_pExportDir: PIMAGE_EXPORT_DIRECTORY,
    m_pImportDir: PIMAGE_IMPORT_DESCRIPTOR,
    m_pResourceDir: PIMAGE_RESOURCE_DIRECTORY,
    m_pExceptionDir: PIMAGE_RUNTIME_FUNCTION_ENTRY,
    m_pSecurityDir: LPWIN_CERTIFICATE,
    m_pBaseRelocDir: PIMAGE_BASE_RELOCATION,
    m_pDebugDir: PIMAGE_DEBUG_DIRECTORY,
    m_pArchitectureDir: PIMAGE_ARCHITECTURE_HEADER,
    m_pGlobalPtrDir: PVOID,
    m_pTlsDir: PIMAGE_TLS_DIRECTORY,
    m_pConfigDir: PIMAGE_LOAD_CONFIG_DIRECTORY,
    m_pBoundImportDir: PIMAGE_BOUND_IMPORT_DESCRIPTOR,
    m_pThunkDataDir: PIMAGE_THUNK_DATA,
    m_pDelayLoadDir: PIMAGE_DELAYLOAD_DESCRIPTOR,
    m_pCor20Dir: PIMAGE_COR20_HEADER,

    // Internal
    m_pData: *const u8,
    m_iSize: usize,

    m_eLastError: EError,
}

impl PE {
    pub fn new(data: &[u8]) -> Self {
        let mut pe = Self {
            m_imgDosHeader: std::ptr::null(),
            m_imgNtHeaders: std::ptr::null(),
            m_imgOptionalHeader: std::ptr::null(),
            m_vecSections: Vec::new(),

            m_pExportDir: std::ptr::null(),
            m_pImportDir: std::ptr::null(),
            m_pResourceDir: std::ptr::null(),
            m_pExceptionDir: std::ptr::null(),
            m_pSecurityDir: std::ptr::null(),
            m_pBaseRelocDir: std::ptr::null(),
            m_pDebugDir: std::ptr::null(),
            m_pArchitectureDir: std::ptr::null(),
            m_pGlobalPtrDir: std::ptr::null(),
            m_pTlsDir: std::ptr::null(),
            m_pConfigDir: std::ptr::null(),
            m_pBoundImportDir: std::ptr::null(),
            m_pThunkDataDir: std::ptr::null(),
            m_pDelayLoadDir: std::ptr::null(),
            m_pCor20Dir: std::ptr::null(),

            m_pData: data.as_ptr(),
            m_iSize: data.len(),

            m_eLastError: EError::E_NONE,
        };
        pe.m_eLastError = pe.Load(data.as_ptr(), data.len());
        pe
    }

    pub fn new_raw(data: *const u8, size: usize) -> Self {
        let mut pe = Self {
            m_imgDosHeader: std::ptr::null(),
            m_imgNtHeaders: std::ptr::null(),
            m_imgOptionalHeader: std::ptr::null(),
            m_vecSections: Vec::new(),

            m_pExportDir: std::ptr::null(),
            m_pImportDir: std::ptr::null(),
            m_pResourceDir: std::ptr::null(),
            m_pExceptionDir: std::ptr::null(),
            m_pSecurityDir: std::ptr::null(),
            m_pBaseRelocDir: std::ptr::null(),
            m_pDebugDir: std::ptr::null(),
            m_pArchitectureDir: std::ptr::null(),
            m_pGlobalPtrDir: std::ptr::null(),
            m_pTlsDir: std::ptr::null(),
            m_pConfigDir: std::ptr::null(),
            m_pBoundImportDir: std::ptr::null(),
            m_pThunkDataDir: std::ptr::null(),
            m_pDelayLoadDir: std::ptr::null(),
            m_pCor20Dir: std::ptr::null(),

            m_pData: data,
            m_iSize: size,

            m_eLastError: EError::E_NONE,
        };
        pe.m_eLastError = pe.Load(data, size);
        pe
    }

    pub fn GetDosHeader(&self) -> Option<&IMAGE_DOS_HEADER> {
        if self.m_imgDosHeader.is_null() {
            None
        } else {
            unsafe { Some(&*self.m_imgDosHeader) }
        }
    }

    pub fn get_dos_header(&self) -> Option<&IMAGE_DOS_HEADER> {
        self.GetDosHeader()
    }

    pub fn GetDosHeaderRaw(&self) -> PIMAGE_DOS_HEADER {
        self.m_imgDosHeader
    }

    pub fn GetNtHeaders(&self) -> Option<&IMAGE_NT_HEADERS> {
        if self.m_imgNtHeaders.is_null() {
            None
        } else {
            unsafe { Some(&*self.m_imgNtHeaders) }
        }
    }

    pub fn get_nt_headers(&self) -> Option<&IMAGE_NT_HEADERS> {
        self.GetNtHeaders()
    }

    pub fn GetNtHeadersRaw(&self) -> PIMAGE_NT_HEADERS {
        self.m_imgNtHeaders
    }

    pub fn GetOptionalHeader(&self) -> Option<&IMAGE_OPTIONAL_HEADER> {
        if self.m_imgOptionalHeader.is_null() {
            None
        } else {
            unsafe { Some(&*self.m_imgOptionalHeader) }
        }
    }

    pub fn get_optional_header(&self) -> Option<&IMAGE_OPTIONAL_HEADER> {
        self.GetOptionalHeader()
    }

    pub fn GetOptionalHeaderRaw(&self) -> PIMAGE_OPTIONAL_HEADER {
        self.m_imgOptionalHeader
    }

    pub fn GetSections(&self) -> &Vec<PESection> {
        &self.m_vecSections
    }

    pub fn get_sections(&self) -> &Vec<PESection> {
        &self.m_vecSections
    }

    pub fn GetSectionsMut(&mut self) -> &mut Vec<PESection> {
        &mut self.m_vecSections
    }

    pub fn get_sections_mut(&mut self) -> &mut Vec<PESection> {
        &mut self.m_vecSections
    }

    pub fn GetExportDir(&self) -> PIMAGE_EXPORT_DIRECTORY { self.m_pExportDir }
    pub fn get_export_dir(&self) -> PIMAGE_EXPORT_DIRECTORY { self.m_pExportDir }

    pub fn GetImportDir(&self) -> PIMAGE_IMPORT_DESCRIPTOR { self.m_pImportDir }
    pub fn get_import_dir(&self) -> PIMAGE_IMPORT_DESCRIPTOR { self.m_pImportDir }

    pub fn GetResourceDir(&self) -> PIMAGE_RESOURCE_DIRECTORY { self.m_pResourceDir }
    pub fn get_resource_dir(&self) -> PIMAGE_RESOURCE_DIRECTORY { self.m_pResourceDir }

    pub fn GetExceptionDir(&self) -> PIMAGE_RUNTIME_FUNCTION_ENTRY { self.m_pExceptionDir }
    pub fn get_exception_dir(&self) -> PIMAGE_RUNTIME_FUNCTION_ENTRY { self.m_pExceptionDir }

    pub fn GetSecurityDir(&self) -> LPWIN_CERTIFICATE { self.m_pSecurityDir }
    pub fn get_security_dir(&self) -> LPWIN_CERTIFICATE { self.m_pSecurityDir }

    pub fn GetBaseRelocDir(&self) -> PIMAGE_BASE_RELOCATION { self.m_pBaseRelocDir }
    pub fn get_base_reloc_dir(&self) -> PIMAGE_BASE_RELOCATION { self.m_pBaseRelocDir }

    pub fn GetDebugDir(&self) -> PIMAGE_DEBUG_DIRECTORY { self.m_pDebugDir }
    pub fn get_debug_dir(&self) -> PIMAGE_DEBUG_DIRECTORY { self.m_pDebugDir }

    pub fn GetArchitectureDir(&self) -> PIMAGE_ARCHITECTURE_HEADER { self.m_pArchitectureDir }
    pub fn get_architecture_dir(&self) -> PIMAGE_ARCHITECTURE_HEADER { self.m_pArchitectureDir }

    pub fn GetGlobalPtrDir(&self) -> PVOID { self.m_pGlobalPtrDir }
    pub fn get_global_ptr_dir(&self) -> PVOID { self.m_pGlobalPtrDir }

    pub fn GetTlsDir(&self) -> PIMAGE_TLS_DIRECTORY { self.m_pTlsDir }
    pub fn get_tls_dir(&self) -> PIMAGE_TLS_DIRECTORY { self.m_pTlsDir }

    pub fn GetConfigDir(&self) -> PIMAGE_LOAD_CONFIG_DIRECTORY { self.m_pConfigDir }
    pub fn get_config_dir(&self) -> PIMAGE_LOAD_CONFIG_DIRECTORY { self.m_pConfigDir }

    pub fn GetBoundImportDir(&self) -> PIMAGE_BOUND_IMPORT_DESCRIPTOR { self.m_pBoundImportDir }
    pub fn get_bound_import_dir(&self) -> PIMAGE_BOUND_IMPORT_DESCRIPTOR { self.m_pBoundImportDir }

    pub fn GetThunkDataDir(&self) -> PIMAGE_THUNK_DATA { self.m_pThunkDataDir }
    pub fn get_thunk_data_dir(&self) -> PIMAGE_THUNK_DATA { self.m_pThunkDataDir }

    pub fn GetDelayLoadDir(&self) -> PIMAGE_DELAYLOAD_DESCRIPTOR { self.m_pDelayLoadDir }
    pub fn get_delay_load_dir(&self) -> PIMAGE_DELAYLOAD_DESCRIPTOR { self.m_pDelayLoadDir }

    pub fn GetCor20Dir(&self) -> PIMAGE_COR20_HEADER { self.m_pCor20Dir }
    pub fn get_cor20_dir(&self) -> PIMAGE_COR20_HEADER { self.m_pCor20Dir }

    pub fn GetError(&self) -> EError {
        self.m_eLastError
    }

    pub fn get_error(&self) -> EError {
        self.m_eLastError
    }

    pub fn FindSection(&self, section_name: &str) -> Option<&PESection> {
        for section in self.GetSections() {
            if section.GetName() == section_name {
                return Some(section);
            }
        }
        None
    }

    pub fn find_section(&self, section_name: &str) -> Option<&PESection> {
        self.FindSection(section_name)
    }

    fn ParseDosHeader(&mut self) -> bool {
        if self.m_iSize < std::mem::size_of::<IMAGE_DOS_HEADER>() {
            self.m_eLastError = EError::E_TOO_SMALL_BINARY;
            return false;
        }

        self.m_imgDosHeader = self.m_pData as PIMAGE_DOS_HEADER;

        unsafe {
            if (*self.m_imgDosHeader).e_magic != IMAGE_DOS_SIGNATURE {
                self.m_eLastError = EError::E_INVALID_DOS_SIGNATURE;
                return false;
            }
        }

        self.m_eLastError = EError::E_SUCCESS;
        true
    }

    fn ParseNtHeaders(&mut self) -> bool {
        unsafe {
            let e_lfanew = (*self.m_imgDosHeader).e_lfanew;
            if e_lfanew < 0 || (e_lfanew as usize) + std::mem::size_of::<IMAGE_NT_HEADERS64>() > self.m_iSize {
                self.m_eLastError = EError::E_INVALID_HEADER_SIGNATURE;
                return false;
            }

            self.m_imgNtHeaders = self.m_pData.offset(e_lfanew as isize) as PIMAGE_NT_HEADERS;

            if (*self.m_imgNtHeaders).Signature != IMAGE_NT_SIGNATURE {
                self.m_eLastError = EError::E_INVALID_HEADER_SIGNATURE;
                return false;
            }
        }

        self.m_eLastError = EError::E_SUCCESS;
        true
    }

    fn ParseOptionalHeader(&mut self) -> bool {
        unsafe {
            self.m_imgOptionalHeader = &(*self.m_imgNtHeaders).OptionalHeader as PIMAGE_OPTIONAL_HEADER;
        }
        true
    }

    fn ParseSectionHeaders(&mut self) -> bool {
        unsafe {
            let e_lfanew = (*self.m_imgDosHeader).e_lfanew as usize;
            let opt_hdr_offset = std::mem::offset_of!(IMAGE_NT_HEADERS64, OptionalHeader);
            let size_of_opt_hdr = (*self.m_imgNtHeaders).FileHeader.SizeOfOptionalHeader as usize;

            let section_header_offset = e_lfanew + opt_hdr_offset + size_of_opt_hdr;
            if section_header_offset >= self.m_iSize {
                return false;
            }

            let p_section_header = self.m_pData.add(section_header_offset) as *const IMAGE_SECTION_HEADER;
            if p_section_header.is_null() {
                return false;
            }

            let number_of_sections = (*self.m_imgNtHeaders).FileHeader.NumberOfSections;

            for i in 0..number_of_sections {
                let sec_ptr = p_section_header.add(i as usize);
                if (sec_ptr as usize) + std::mem::size_of::<IMAGE_SECTION_HEADER>() > (self.m_pData as usize + self.m_iSize) {
                    break;
                }
                let section = PESection::new(*sec_ptr);
                self.m_vecSections.push(section);
            }
        }

        true
    }

    pub fn ParseExports(&self) -> bool {
        true
    }

    fn ParseDataDirectories(&mut self) -> bool {
        unsafe {
            let opt_hdr = &*self.m_imgOptionalHeader;

            macro_rules! gather_datadir {
                ($index:expr, $out:expr, $type:ty) => {
                    let dir = opt_hdr.DataDirectory[$index];
                    let rva = dir.VirtualAddress;
                    if rva != 0 {
                        let offset = self.RvaToOffset(rva);
                        if offset != 0 {
                            $out = self.m_pData.add(offset as usize) as $type;
                        }
                    }
                };
            }

            gather_datadir!(IMAGE_DIRECTORY_ENTRY_EXPORT, self.m_pExportDir, PIMAGE_EXPORT_DIRECTORY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_IMPORT, self.m_pImportDir, PIMAGE_IMPORT_DESCRIPTOR);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_RESOURCE, self.m_pResourceDir, PIMAGE_RESOURCE_DIRECTORY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_EXCEPTION, self.m_pExceptionDir, PIMAGE_RUNTIME_FUNCTION_ENTRY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_SECURITY, self.m_pSecurityDir, LPWIN_CERTIFICATE);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_BASERELOC, self.m_pBaseRelocDir, PIMAGE_BASE_RELOCATION);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_DEBUG, self.m_pDebugDir, PIMAGE_DEBUG_DIRECTORY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_ARCHITECTURE, self.m_pArchitectureDir, PIMAGE_ARCHITECTURE_HEADER);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_GLOBALPTR, self.m_pGlobalPtrDir, PVOID);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_TLS, self.m_pTlsDir, PIMAGE_TLS_DIRECTORY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG, self.m_pConfigDir, PIMAGE_LOAD_CONFIG_DIRECTORY);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT, self.m_pBoundImportDir, PIMAGE_BOUND_IMPORT_DESCRIPTOR);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_IAT, self.m_pThunkDataDir, PIMAGE_THUNK_DATA);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT, self.m_pDelayLoadDir, PIMAGE_DELAYLOAD_DESCRIPTOR);
            gather_datadir!(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR, self.m_pCor20Dir, PIMAGE_COR20_HEADER);
        }

        true
    }

    pub fn RvaToOffset(&self, rva: DWORD) -> DWORD {
        for section in &self.m_vecSections {
            let section_header = section.section_header();
            let start = section_header.VirtualAddress;
            let virtual_size = unsafe { section_header.Misc.VirtualSize };
            let end = start + virtual_size;
            if rva >= start && rva < end {
                return rva - start + section_header.PointerToRawData;
            }
        }

        0
    }

    pub fn rva_to_offset(&self, rva: DWORD) -> DWORD {
        self.RvaToOffset(rva)
    }

    fn Load(&mut self, data: *const u8, size: usize) -> EError {
        self.m_pData = data;
        self.m_iSize = size;

        if data.is_null() {
            return EError::E_INVALID_BINARY;
        }

        if !self.ParseDosHeader() {
            return self.m_eLastError;
        }

        if !self.ParseNtHeaders() {
            return self.m_eLastError;
        }

        if !self.ParseOptionalHeader() {
            return self.m_eLastError;
        }

        if !self.ParseSectionHeaders() {
            return self.m_eLastError;
        }

        if !self.ParseDataDirectories() {
            return self.m_eLastError;
        }

        EError::E_SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_str() {
        assert_eq!(ErrorToStr(EError::E_SUCCESS), "E_SUCCESS");
        assert_eq!(ErrorToStr(EError::E_TOO_SMALL_BINARY), "E_TOO_SMALL_BINARY");
        assert_eq!(ErrorToStr(EError::E_INVALID_DOS_SIGNATURE), "E_INVALID_DOS_SIGNATURE");
        assert_eq!(ErrorToStr(EError::E_INVALID_HEADER_SIGNATURE), "E_INVALID_HEADER_SIGNATURE");
        assert_eq!(ErrorToStr(EError::E_NONE), "INVALID_ERROR_CODE");
    }

    #[test]
    fn test_invalid_binary() {
        let pe = PE::new(&[]);
        assert!(PE_FAILED(pe.GetError()));
        assert_eq!(pe.GetError(), EError::E_TOO_SMALL_BINARY);
    }
}
