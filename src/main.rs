use aegis::aegis256::{Key, Nonce};
use rand::Rng;

type Tag = aegis::aegis256::Tag<32>;

fn human_readable_size(mut size: f64) -> String {
    let unit = vec![
            ((1 << 30) as f64, "GiB"),
            ((1 << 20) as f64, "MiB"),
            ((1 << 10) as f64, "KiB"),
            ((      0) as f64, "b"),
        ]
        .into_iter()
        .filter(|(s,_)| s <= &size.abs())
        .next()
        .unwrap();

    size /= unit.0;

    format!("{size} {}", unit.1)
}

fn main() {
    println!("CPU Features:");
    println!("") ;
    println!("get_cpu_features(): {}", aegis::cpu::get_cpu_features()) ;
    println!("has_neon():         {}", aegis::cpu::has_neon());
    println!("has_armcrypto():    {}", aegis::cpu::has_armcrypto());
    println!("has_avx():          {}", aegis::cpu::has_avx());
    println!("has_avx2():         {}", aegis::cpu::has_avx2());
    println!("has_avx512f():      {}", aegis::cpu::has_avx512f());
    println!("has_aesni():        {}", aegis::cpu::has_aesni());
    println!("has_vaes():         {}", aegis::cpu::has_vaes());
    println!("") ;


    const TOTAL_DATA_SIZE: usize = 100 << 20;
    const BLOCK_SIZE: usize = 16 << 10;

    let mut total_data_encrypted: usize = 0;
    let mut data = vec![0u8;BLOCK_SIZE];

    let mut key = Key::default();
    let mut nonce = Nonce::default();

    rand::thread_rng().fill(key.as_mut());
    rand::thread_rng().fill(nonce.as_mut());

    let aegis = aegis::aegis256::Aegis256::new(&key, &nonce);

    let start = std::time::Instant::now();
    while total_data_encrypted < TOTAL_DATA_SIZE {
        let _tag: Tag = aegis.encrypt_in_place(&mut data, &[]);
        total_data_encrypted += data.len();
    }
    let end = std::time::Instant::now();

    let elapsed = end-start;
    println!("Encrypted: {}", human_readable_size(total_data_encrypted as f64));
    println!("Elapsed: {}s", elapsed.as_secs_f64());
    println!("Throughput: {}/s", human_readable_size((total_data_encrypted as f64) / elapsed.as_secs_f64()));
}
