#include <iostream>

#include <NeoPE.h>
#include "hello.h"

#define PRINT_PROP(y, x) std::cout << "  = " << y << ": " << x << "\n";
#define PRINT_SUBJECT(x) std::cout << "\n" << "=== " << x << " ===" << "\n";

int main(int argc, char** argv)
{
    NeoPE::PE pe(__hello_exe, __hello_exe_len);

    if (PE_FAILED(pe.GetError()))
        return 1;

    // DosHeader
    auto DosHeader = pe.GetDosHeader();

    PRINT_SUBJECT("DosHeader");
    PRINT_PROP("e_cblp", DosHeader->e_cblp);
    PRINT_PROP("e_cp", DosHeader->e_cp);
    PRINT_PROP("e_cparhdr", DosHeader->e_cparhdr);
    PRINT_PROP("e_crlc", DosHeader->e_crlc);
    PRINT_PROP("e_cs", DosHeader->e_cs);
    PRINT_PROP("e_csum", DosHeader->e_csum);
    PRINT_PROP("e_ip", DosHeader->e_ip);
    PRINT_PROP("e_lfanew", DosHeader->e_lfanew);
    PRINT_PROP("e_lfarlc", DosHeader->e_lfarlc);
    PRINT_PROP("e_magic", DosHeader->e_magic);
    PRINT_PROP("e_maxalloc", DosHeader->e_maxalloc);
    PRINT_PROP("e_minalloc", DosHeader->e_minalloc);
    PRINT_PROP("e_oemid", DosHeader->e_oemid);
    PRINT_PROP("e_oeminfo", DosHeader->e_oeminfo);
    PRINT_PROP("e_ovno", DosHeader->e_ovno);
    PRINT_PROP("e_res", DosHeader->e_res);
    PRINT_PROP("e_res2", DosHeader->e_res2);
    PRINT_PROP("e_sp", DosHeader->e_sp);
    PRINT_PROP("e_ss", DosHeader->e_ss);

    // NtHeaders
    auto NtHeaders = pe.GetNtHeaders();

    PRINT_SUBJECT("NtHeaders");
    PRINT_PROP("Signature", NtHeaders->Signature);
    PRINT_PROP("FileHeader", (void*)&NtHeaders->FileHeader);
    PRINT_PROP("OptionalHeader", (void*)&NtHeaders->OptionalHeader);
}