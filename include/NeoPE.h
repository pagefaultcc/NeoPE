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
#include <vector>

#pragma region WINDOWS_DEFINITIONS

#if defined(_WIN64)
#include <Windows.h>
#else
constexpr short IMAGE_DOS_SIGNATURE = 23117;

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

#endif
#pragma endregion

NEOPE_START_NAMESPACE_

#define PE_FAILED(error) error < 1

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

class PE
{
public:
    explicit PE(unsigned char Data[], size_t Size)
    {
        m_eLastError = Load(Data, Size);
    }
    ~PE() = default;

    [[nodiscard]] inline PIMAGE_DOS_HEADER GetDosHeader()           const { return const_cast<const PIMAGE_DOS_HEADER>(&m_imgDosHeader); }
    [[nodiscard]] inline PIMAGE_NT_HEADERS GetNtHeaders()           const { return const_cast<const PIMAGE_NT_HEADERS>(&m_imgNtHeaders); }
    [[nodiscard]] inline PIMAGE_OPTIONAL_HEADER GetOptionalHeader() const { return const_cast<const PIMAGE_OPTIONAL_HEADER>(&m_imgOptionalHeader); }

    [[nodiscard]] inline EError GetError() const { return m_eLastError; }

private:
    bool ParseDosHeader()
    {
        if (!m_pData)
            return false;

        if (m_iSize < sizeof(IMAGE_DOS_HEADER))
        {
            m_eLastError = E_TOO_SMALL_BINARY;
            return false;
        }

        m_imgDosHeader = *reinterpret_cast<IMAGE_DOS_HEADER*>(m_pData);

        if (m_imgDosHeader.e_magic != IMAGE_DOS_SIGNATURE)
        {
            m_eLastError = E_INVALID_DOS_SIGNATURE;
            return false;
        }

        m_eLastError = E_SUCCESS;

        return true;
    }

    bool ParseNtHeaders()
    {
        if (!m_pData)
            return false;

        m_imgNtHeaders = *reinterpret_cast<IMAGE_NT_HEADERS*>(m_pData + m_imgDosHeader.e_lfanew);

        if (m_imgNtHeaders.Signature != IMAGE_NT_SIGNATURE)
        {
            m_eLastError = E_INVALID_HEADER_SIGNATURE;
            return false;
        }

        m_eLastError = E_SUCCESS;

        return true;
    }

    bool ParseOptionalHeader()
    {
        m_imgOptionalHeader = m_imgNtHeaders.OptionalHeader;

        return true;
    }

    bool ParseSectionHeaders()
    {
        return true;
    }

    EError Load(unsigned char Data[], size_t Size)
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

        return EError::E_SUCCESS;
    }

    // PE

    IMAGE_DOS_HEADER m_imgDosHeader = {};
    IMAGE_NT_HEADERS m_imgNtHeaders = {};
    IMAGE_OPTIONAL_HEADER64 m_imgOptionalHeader = {};

    // Internal

    // I want to clarify myself in here, this just stored for startup, afterwards everything is parsed, stays in the fields in this class.
    // This is done because if the Data pointer given by used could be deleted afterwards and we dont want use-after-free.
    //                                                                                                              --- kenanwastaken
    unsigned char* m_pData;
    size_t m_iSize;

    EError m_eLastError = E_NONE;
};

NEOPE_END_NAMESPACE_

#endif