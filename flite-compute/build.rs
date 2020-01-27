fn main() {
    println!("cargo:rustc-link-lib=flite");
    println!("cargo:rustc-link-lib=flite_cmu_us_kal");
    println!("cargo:rustc-link-lib=flite_usenglish");
    println!("cargo:rustc-link-lib=flite_cmulex");
    println!("cargo:rustc-link-search=flite-compute/lib");
}
