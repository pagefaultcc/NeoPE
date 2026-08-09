//  _  _         ___ ___
// | \| |___ ___| _ \ __|  Header-only and lightweight PE utility library.
// | .` / -_) _ \  _/ _|   version 0.0.0
// |_|\_\___\___/_| |___|  https://github.com/pagefaultcc/NeoPE
//
// SPDX-FileCopyrightText. 2026 - 2027 pagefault.cc. <https://github.com/pagefaultcc>
// SPDX-License-Identifier: MIT

#pragma once

#ifndef NEOPE_H_
#define NEOPE_H_

#define NEOPE_START_NAMESPACE_ namespace NeoPE {
#define NEOPE_END_NAMESPACE_ }

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#pragma region WINDOWS_DEFINITIONS
#if defined(_WIN64)
#include <Windows.h>
#include <wintrust.h>
#else

typedef unsigned char BYTE;
typedef char CHAR;
typedef unsigned char UCHAR;
typedef short SHORT;
typedef unsigned short USHORT;
typedef unsigned short WORD;
typedef int INT;
typedef unsigned int UINT;
typedef long LONG;
typedef unsigned long ULONG;
typedef unsigned long DWORD;
typedef __int64 LONGLONG;
typedef unsigned __int64 ULONGLONG;
typedef ULONGLONG DWORDLONG;
typedef unsigned int DWORD32;
typedef unsigned __int64 DWORD64;

typedef signed char INT8;
typedef signed short INT16;
typedef signed int INT32;
typedef signed __int64 INT64;

typedef unsigned char UINT8;
typedef unsigned short UINT16;
typedef unsigned int UINT32;
typedef unsigned __int64 UINT64;

typedef int LONG32;
typedef __int64 LONG64;
typedef unsigned int ULONG32;
typedef unsigned __int64 ULONG64;

#define IMAGE_SIZEOF_SHORT_NAME 8

//0x40 bytes (sizeof)
typedef struct _IMAGE_DOS_HEADER
{
    USHORT e_magic;                                                         //0x0
    USHORT e_cblp;                                                          //0x2
    USHORT e_cp;                                                            //0x4
    USHORT e_crlc;                                                          //0x6
    USHORT e_cparhdr;                                                       //0x8
    USHORT e_minalloc;                                                      //0xa
    USHORT e_maxalloc;                                                      //0xc
    USHORT e_ss;                                                            //0xe
    USHORT e_sp;                                                            //0x10
    USHORT e_csum;                                                          //0x12
    USHORT e_ip;                                                            //0x14
    USHORT e_cs;                                                            //0x16
    USHORT e_lfarlc;                                                        //0x18
    USHORT e_ovno;                                                          //0x1a
    USHORT e_res[4];                                                        //0x1c
    USHORT e_oemid;                                                         //0x24
    USHORT e_oeminfo;                                                       //0x26
    USHORT e_res2[10];                                                      //0x28
    LONG e_lfanew;                                                          //0x3c
} IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;

//0x28 bytes (sizeof)
typedef struct _IMAGE_SECTION_HEADER
{
    UCHAR Name[8];                                                          //0x0
    union
    {
        ULONG PhysicalAddress;                                              //0x8
        ULONG VirtualSize;                                                  //0x8
    } Misc;                                                                 //0x8
    ULONG VirtualAddress;                                                   //0xc
    ULONG SizeOfRawData;                                                    //0x10
    ULONG PointerToRawData;                                                 //0x14
    ULONG PointerToRelocations;                                             //0x18
    ULONG PointerToLinenumbers;                                             //0x1c
    USHORT NumberOfRelocations;                                             //0x20
    USHORT NumberOfLinenumbers;                                             //0x22
    ULONG Characteristics;                                                  //0x24
} IMAGE_SECTION_HEADER, *PIMAGE_SECTION_HEADER;

//0x14 bytes (sizeof)
typedef struct _IMAGE_FILE_HEADER
{
    USHORT Machine;                                                         //0x0
    USHORT NumberOfSections;                                                //0x2
    ULONG TimeDateStamp;                                                    //0x4
    ULONG PointerToSymbolTable;                                             //0x8
    ULONG NumberOfSymbols;                                                  //0xc
    USHORT SizeOfOptionalHeader;                                            //0x10
    USHORT Characteristics;                                                 //0x12
} IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;

//0x8 bytes (sizeof)
struct _IMAGE_DATA_DIRECTORY
{
    ULONG VirtualAddress;                                                   //0x0
    ULONG Size;                                                             //0x4
};

//0xf0 bytes (sizeof)
typedef struct _IMAGE_OPTIONAL_HEADER64
{
    USHORT Magic;                                                           //0x0
    UCHAR MajorLinkerVersion;                                               //0x2
    UCHAR MinorLinkerVersion;                                               //0x3
    ULONG SizeOfCode;                                                       //0x4
    ULONG SizeOfInitializedData;                                            //0x8
    ULONG SizeOfUninitializedData;                                          //0xc
    ULONG AddressOfEntryPoint;                                              //0x10
    ULONG BaseOfCode;                                                       //0x14
    ULONGLONG ImageBase;                                                    //0x18
    ULONG SectionAlignment;                                                 //0x20
    ULONG FileAlignment;                                                    //0x24
    USHORT MajorOperatingSystemVersion;                                     //0x28
    USHORT MinorOperatingSystemVersion;                                     //0x2a
    USHORT MajorImageVersion;                                               //0x2c
    USHORT MinorImageVersion;                                               //0x2e
    USHORT MajorSubsystemVersion;                                           //0x30
    USHORT MinorSubsystemVersion;                                           //0x32
    ULONG Win32VersionValue;                                                //0x34
    ULONG SizeOfImage;                                                      //0x38
    ULONG SizeOfHeaders;                                                    //0x3c
    ULONG CheckSum;                                                         //0x40
    USHORT Subsystem;                                                       //0x44
    USHORT DllCharacteristics;                                              //0x46
    ULONGLONG SizeOfStackReserve;                                           //0x48
    ULONGLONG SizeOfStackCommit;                                            //0x50
    ULONGLONG SizeOfHeapReserve;                                            //0x58
    ULONGLONG SizeOfHeapCommit;                                             //0x60
    ULONG LoaderFlags;                                                      //0x68
    ULONG NumberOfRvaAndSizes;                                              //0x6c
    struct _IMAGE_DATA_DIRECTORY DataDirectory[16];                         //0x70
} IMAGE_OPTIONAL_HEADER64, *PIMAGE_OPTIONAL_HEADER64;

typedef struct _IMAGE_NT_HEADERS64
{
    ULONG Signature;                                                        //0x0
    struct _IMAGE_FILE_HEADER FileHeader;                                   //0x4
    struct _IMAGE_OPTIONAL_HEADER64 OptionalHeader;                         //0x18
} IMAGE_NT_HEADERS64, *PIMAGE_NT_HEADERS64;

#ifdef _WIN64
typedef IMAGE_NT_HEADERS64                  IMAGE_NT_HEADERS;
typedef PIMAGE_NT_HEADERS64                 PIMAGE_NT_HEADERS;
#else
typedef IMAGE_NT_HEADERS32                  IMAGE_NT_HEADERS;
typedef PIMAGE_NT_HEADERS32                 PIMAGE_NT_HEADERS;
#endif

#ifdef _WIN64
typedef IMAGE_OPTIONAL_HEADER64             IMAGE_OPTIONAL_HEADER;
typedef PIMAGE_OPTIONAL_HEADER64            PIMAGE_OPTIONAL_HEADER;
#define IMAGE_NT_OPTIONAL_HDR_MAGIC         IMAGE_NT_OPTIONAL_HDR64_MAGIC
#else
typedef IMAGE_OPTIONAL_HEADER32             IMAGE_OPTIONAL_HEADER;
typedef PIMAGE_OPTIONAL_HEADER32            PIMAGE_OPTIONAL_HEADER;
#define IMAGE_NT_OPTIONAL_HDR_MAGIC         IMAGE_NT_OPTIONAL_HDR32_MAGIC
#endif

#define IMAGE_DOS_SIGNATURE                 0x5A4D      // MZ
#define IMAGE_NT_SIGNATURE                  0x00004550  // PE00

#endif
#pragma endregion

NEOPE_START_NAMESPACE_

#define PE_FAILED(error) error < 1
#define NEOPE_INTERNAL_GATHER_DATADIR(x, m)                                                 \
    IMAGE_DATA_DIRECTORY p##x##Dir = m_imgOptionalHeader->DataDirectory[x];                  \
    DWORD x##RVA    = p##x##Dir.VirtualAddress;                                              \
    if (!x##RVA) return false;                                                              \
    DWORD x##Offset = RvaToOffset(x##RVA);                                                  \
    m = reinterpret_cast<decltype(m)>(m_pData + x##Offset)

enum EError : uint16_t
{
    E_NONE = 0,

    E_SUCCESS,
    E_TOO_SMALL_BINARY,
    E_INVALID_BINARY,
    E_INVALID_DOS_SIGNATURE,
    E_INVALID_HEADER_SIGNATURE,

    E_MAX
};

static std::string ErrorToStr(const EError& Error)
{
    switch (Error)
    {
        case E_SUCCESS:
            return "E_SUCCESS";
            break;
        case E_TOO_SMALL_BINARY:
            return "E_TOO_SMALL_BINARY";
            break;
        case E_INVALID_DOS_SIGNATURE:
            return "E_INVALID_DOS_SIGNATURE";
            break;
        case E_INVALID_HEADER_SIGNATURE:
            return "E_INVALID_HEADER_SIGNATURE";
            break;
        default:
            return "INVALID_ERROR_CODE";
            break;
    }
}

class PESection
{
public:
    explicit PESection(const IMAGE_SECTION_HEADER& SectionHeader) : m_imgSectionHeader(SectionHeader) {};

    [[nodiscard]] inline std::string_view GetName() const
    {
        const char* rawName = reinterpret_cast<const char*>(m_imgSectionHeader.Name);
        const size_t length = strnlen(rawName, IMAGE_SIZEOF_SHORT_NAME);

        if (length == 0)
            return "INVALID";

        return std::string_view{ rawName, length };
    }

    [[nodiscard]] inline bool HasCharacteristic(const DWORD Characteristic) const { return (m_imgSectionHeader.Characteristics & Characteristic) == Characteristic; }
    inline void AddCharacteristic(const DWORD Characteristic) { m_imgSectionHeader.Characteristics |= Characteristic; }
    inline void RemoveCharacteristic(const DWORD Characteristic) { m_imgSectionHeader.Characteristics &= ~Characteristic; }

    [[nodiscard]] inline PIMAGE_SECTION_HEADER GetSectionHeader() const { return const_cast<const PIMAGE_SECTION_HEADER>(&m_imgSectionHeader); }

private:
    IMAGE_SECTION_HEADER m_imgSectionHeader;
};

class PEExportedFunction
{
public:
    explicit PEExportedFunction(
        const std::string_view& szName,
        const uint32_t          iOrdinal,
        const uint32_t          iRva,
        const bool              bCouldBeBadExportedFunction
        )
    {

    }

    explicit PEExportedFunction(
        const std::string_view& szName,
        const uint32_t          iOrdinal,
        const uint32_t          iRva,
        PEExportedFunction      cForwarded,
        const bool              bCouldBeBadExportedFunction
    )
    {

    }

private:
    std::string_view m_szName;
    uint32_t m_iOrdinal;
    uint32_t m_iRva;
    bool m_bCouldBeBadExportedFunction;

};

class PE
{
public:
    explicit PE(unsigned char Data[], const size_t Size)
    {
        m_eLastError = Load(Data, Size);
    }
    ~PE() = default;

    [[nodiscard]] inline PIMAGE_DOS_HEADER GetDosHeader()           const { return m_imgDosHeader; }
    [[nodiscard]] inline PIMAGE_NT_HEADERS GetNtHeaders()           const { return m_imgNtHeaders; }
    [[nodiscard]] inline PIMAGE_OPTIONAL_HEADER GetOptionalHeader() const { return m_imgOptionalHeader; }
    [[nodiscard]] inline std::vector<PESection>* GetSections()      const { return const_cast<std::vector<PESection>*>(&m_vecSections); }

    [[nodiscard]] inline PIMAGE_EXPORT_DIRECTORY        GetExportDir()        const { return m_pExportDir; }
    [[nodiscard]] inline PIMAGE_IMPORT_DESCRIPTOR       GetImportDir()        const { return m_pImportDir; }
    [[nodiscard]] inline PIMAGE_RESOURCE_DIRECTORY      GetResourceDir()      const { return m_pResourceDir; }
    [[nodiscard]] inline PIMAGE_RUNTIME_FUNCTION_ENTRY  GetExceptionDir()     const { return m_pExceptionDir; }
    [[nodiscard]] inline LPWIN_CERTIFICATE              GetSecurityDir()      const { return m_pSecurityDir; }
    [[nodiscard]] inline PIMAGE_BASE_RELOCATION         GetBaseRelocDir()     const { return m_pBaseRelocDir; }
    [[nodiscard]] inline PIMAGE_DEBUG_DIRECTORY         GetDebugDir()         const { return m_pDebugDir; }
    [[nodiscard]] inline PIMAGE_ARCHITECTURE_HEADER     GetArchitectureDir()  const { return m_pArchitectureDir; }
    [[nodiscard]] inline PVOID                          GetGlobalPtrDir()     const { return m_pGlobalPtrDir; }
    [[nodiscard]] inline PIMAGE_TLS_DIRECTORY           GetTlsDir()           const { return m_pTlsDir; }
    [[nodiscard]] inline PIMAGE_LOAD_CONFIG_DIRECTORY   GetConfigDir()        const { return m_pConfigDir; }
    [[nodiscard]] inline PIMAGE_BOUND_IMPORT_DESCRIPTOR GetBoundImportDir()   const { return m_pBoundImportDir; }
    [[nodiscard]] inline PIMAGE_THUNK_DATA              GetThunkDataDir()     const { return m_pThunkDataDir; }
    [[nodiscard]] inline PIMAGE_DELAYLOAD_DESCRIPTOR    GetDelayLoadDir()     const { return m_pDelayLoadDir; }
    [[nodiscard]] inline PIMAGE_COR20_HEADER            GetCor20Dir()         const { return m_pCor20Dir; }

    [[nodiscard]] inline EError GetError() const { return m_eLastError; }

    [[nodiscard]] inline PESection* FindSection(const std::string_view& SectionName) const
    {
        for (auto& Section : *GetSections())
        {
            if (Section.GetName() == SectionName)
                return &Section;
        }

        return nullptr;
    }

private:
    bool ParseDosHeader()
    {
        if (m_iSize < sizeof(IMAGE_DOS_HEADER))
        {
            m_eLastError = E_TOO_SMALL_BINARY;
            return false;
        }

        m_imgDosHeader = reinterpret_cast<PIMAGE_DOS_HEADER>(m_pData);

        if (m_imgDosHeader->e_magic != IMAGE_DOS_SIGNATURE)
        {
            m_eLastError = E_INVALID_DOS_SIGNATURE;
            return false;
        }

        m_eLastError = E_SUCCESS;

        return true;
    }

    bool ParseNtHeaders()
    {
        m_imgNtHeaders = reinterpret_cast<PIMAGE_NT_HEADERS>(m_pData + m_imgDosHeader->e_lfanew);

        if (m_imgNtHeaders->Signature != IMAGE_NT_SIGNATURE)
        {
            m_eLastError = E_INVALID_HEADER_SIGNATURE;
            return false;
        }

        m_eLastError = E_SUCCESS;

        return true;
    }

    bool ParseOptionalHeader()
    {
        m_imgOptionalHeader = &m_imgNtHeaders->OptionalHeader;

        return true;
    }

    bool ParseSectionHeaders()
    {
        const auto* pSectionHeader =
            reinterpret_cast<const IMAGE_SECTION_HEADER*>(
                m_pData
                + m_imgDosHeader->e_lfanew
                + offsetof(IMAGE_NT_HEADERS, OptionalHeader)
                + m_imgNtHeaders->FileHeader.SizeOfOptionalHeader
            );

        if (!pSectionHeader)
            return false;

        const WORD numberOfSections = m_imgNtHeaders->FileHeader.NumberOfSections;

        for (WORD i = 0; i < numberOfSections; ++i)
        {
            PESection Section(pSectionHeader[i]);

            m_vecSections.push_back(Section);
        }

        return true;
    }

    [[nodiscard]] bool ParseExports() const
    {
        return true;
    }

    [[nodiscard]] bool ParseDataDirectories()
    {
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_EXPORT,         m_pExportDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_IMPORT,         m_pImportDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_RESOURCE,       m_pResourceDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_EXCEPTION,      m_pExceptionDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_SECURITY,       m_pSecurityDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_BASERELOC,      m_pBaseRelocDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_DEBUG,          m_pDebugDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_ARCHITECTURE,   m_pArchitectureDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_GLOBALPTR,      m_pGlobalPtrDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_TLS,            m_pTlsDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG,    m_pConfigDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT,   m_pBoundImportDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_IAT,            m_pThunkDataDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT,   m_pDelayLoadDir);
        NEOPE_INTERNAL_GATHER_DATADIR(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR, m_pCor20Dir);

        return true;
    }

    [[nodiscard]] DWORD RvaToOffset(const DWORD Rva) const
    {
        for (auto& Section : m_vecSections)
        {
            const auto SectionHeader = Section.GetSectionHeader();

            const DWORD start = SectionHeader->VirtualAddress;
            if (const DWORD end = start + SectionHeader->Misc.VirtualSize; Rva >= start && Rva < end)
                return Rva - start + SectionHeader->PointerToRawData;
        }

        return 0;
    }

    EError Load(unsigned char Data[], const size_t Size)
    {
        m_pData = Data;
        m_iSize = Size;

        if (!Data)
            return EError::E_INVALID_BINARY;

        if (!ParseDosHeader())
            return m_eLastError;

        if (!ParseNtHeaders())
            return m_eLastError;

        if (!ParseOptionalHeader())
            return m_eLastError;

        if (!ParseSectionHeaders())
            return m_eLastError;

        if (!ParseDataDirectories())
            return m_eLastError;

        return EError::E_SUCCESS;
    }

    // PE
    PIMAGE_DOS_HEADER m_imgDosHeader = {};
    PIMAGE_NT_HEADERS m_imgNtHeaders = {};
    PIMAGE_OPTIONAL_HEADER m_imgOptionalHeader = {};
    std::vector<PESection> m_vecSections;

    PIMAGE_EXPORT_DIRECTORY          m_pExportDir        = nullptr;
    PIMAGE_IMPORT_DESCRIPTOR         m_pImportDir        = nullptr;
    PIMAGE_RESOURCE_DIRECTORY        m_pResourceDir      = nullptr;
    PIMAGE_RUNTIME_FUNCTION_ENTRY    m_pExceptionDir     = nullptr;
    LPWIN_CERTIFICATE                m_pSecurityDir      = nullptr;
    PIMAGE_BASE_RELOCATION           m_pBaseRelocDir     = nullptr;
    PIMAGE_DEBUG_DIRECTORY           m_pDebugDir         = nullptr;
    PIMAGE_ARCHITECTURE_HEADER       m_pArchitectureDir  = nullptr;
    PVOID                            m_pGlobalPtrDir     = nullptr;
    PIMAGE_TLS_DIRECTORY             m_pTlsDir           = nullptr;
    PIMAGE_LOAD_CONFIG_DIRECTORY     m_pConfigDir        = nullptr;
    PIMAGE_BOUND_IMPORT_DESCRIPTOR   m_pBoundImportDir   = nullptr;
    PIMAGE_THUNK_DATA                m_pThunkDataDir     = nullptr;
    PIMAGE_DELAYLOAD_DESCRIPTOR      m_pDelayLoadDir     = nullptr;
    PIMAGE_COR20_HEADER              m_pCor20Dir         = nullptr;

    // Internal
    unsigned char* m_pData = nullptr;
    size_t m_iSize = -1;

    EError m_eLastError = E_NONE;
};

NEOPE_END_NAMESPACE_

#endif