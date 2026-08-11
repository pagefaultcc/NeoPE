mod hello;

use neope::{PE_FAILED, PE};

macro_rules! print_prop {
    ($y:expr, $x:expr) => {
        println!("  = {}: {:?}", $y, $x);
    };
}

macro_rules! print_subject {
    ($x:expr) => {
        println!("\n=== {} ===", $x);
    };
}

fn main() {
    let pe = PE::new(hello::HELLO_EXE);

    if PE_FAILED(pe.GetError()) {
        std::process::exit(1);
    }

    // DosHeader
    let dos_header = pe.GetDosHeader().expect("Failed to get DosHeader");

    print_subject!("DosHeader");
    print_prop!("e_cblp", dos_header.e_cblp);
    print_prop!("e_cp", dos_header.e_cp);
    print_prop!("e_cparhdr", dos_header.e_cparhdr);
    print_prop!("e_crlc", dos_header.e_crlc);
    print_prop!("e_cs", dos_header.e_cs);
    print_prop!("e_csum", dos_header.e_csum);
    print_prop!("e_ip", dos_header.e_ip);
    print_prop!("e_lfanew", dos_header.e_lfanew);
    print_prop!("e_lfarlc", dos_header.e_lfarlc);
    print_prop!("e_magic", dos_header.e_magic);
    print_prop!("e_maxalloc", dos_header.e_maxalloc);
    print_prop!("e_minalloc", dos_header.e_minalloc);
    print_prop!("e_oemid", dos_header.e_oemid);
    print_prop!("e_oeminfo", dos_header.e_oeminfo);
    print_prop!("e_ovno", dos_header.e_ovno);
    print_prop!("e_res", dos_header.e_res.as_ptr());
    print_prop!("e_res2", dos_header.e_res2.as_ptr());
    print_prop!("e_sp", dos_header.e_sp);
    print_prop!("e_ss", dos_header.e_ss);

    // NtHeaders
    let nt_headers = pe.GetNtHeaders().expect("Failed to get NtHeaders");

    print_subject!("NtHeaders");
    print_prop!("Signature", nt_headers.Signature);
    print_prop!("FileHeader", &nt_headers.FileHeader as *const _);
    print_prop!("OptionalHeader", &nt_headers.OptionalHeader as *const _);
}
