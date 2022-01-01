#[cfg(windows)]
mod windows;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("./install_cert cert.crt");
        return;
    }
    let ca_cert_bytes = std::fs::read(&args[1]).expect("ca cert file path not valid!");

    #[allow(unused)]
    let certs =
        rustls_pemfile::certs(&mut ca_cert_bytes.as_slice()).expect("failed to parse certificate");

    #[cfg(windows)]
    windows::install_cert(&certs[0]);

    #[cfg(not(windows))]
    panic!("not implemented on this system")
}
