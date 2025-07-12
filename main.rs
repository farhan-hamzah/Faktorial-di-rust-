use std::io;

fn main() {
    let mut input = String::new();
    println!("Masukkan bilangan untuk dihitung faktorialnya:");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");

    let n: u64 = input.trim().parse().expect("Masukkan angka positif");

    let mut hasil = 1;

    for i in 1..=n {
        hasil *= i;
    }

    println!("Faktorial dari {} adalah {}", n, hasil);
}
