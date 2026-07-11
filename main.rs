use glob::glob;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Tek bir dosyayı sil.
fn remove_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

/// Bir dizini ve içindeki her şeyi sil.
fn remove_dir(path: &str) -> io::Result<()> {
    fs::remove_dir_all(path)
}

/// `path`'in dosya mı yoksa dizin mi olduğuna bakıp uygun şekilde siler.
fn delete_path(path: &str) {
    let p = Path::new(path);
    let result = if p.is_file() {
        remove_file(path)
    } else if p.is_dir() {
        remove_dir(path)
    } else {
        println!("'{}' bir dosya ya da dizin değil, atlanıyor.", path);
        return;
    };

    match result {
        Ok(_) => println!("Silindi: {}", path),
        Err(e) => println!("Silinemedi {}: {}", path, e),
    }
}

/// Kullanıcıya evet/hayır sorusu sorar, "y"/"yes" ise true döner.
fn confirm(prompt: &str) -> bool {
    print!("{} [e/H]: ", prompt);
    io::stdout().flush().ok();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    matches!(answer.trim().to_lowercase().as_str(), "e" | "evet" | "y" | "yes")
}

/// Seçenek 1: Kullanıcının yazdığı tek bir dosya/dizini siler.
fn clean_specific() {
    println!("Silinecek dosya veya dizinin tam yolunu girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    if !Path::new(path).exists() {
        println!("Dosya veya dizin bulunamadı.");
        return;
    }

    if confirm(&format!("'{}' kalıcı olarak silinsin mi?", path)) {
        delete_path(path);
    } else {
        println!("İptal edildi.");
    }
}

/// Seçenek 2: Bir glob desenine uyan tüm dosya/dizinleri siler.
/// Örnek: C:\**\*.tmp  ya da  /home/kullanici/**/*.log
fn clean_by_pattern() {
    println!("Eşleştirmek için bir glob deseni girin (örn: C:\\**\\*.tmp):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let pattern = input.trim();

    let entries: Vec<_> = match glob(pattern) {
        Ok(paths) => paths.filter_map(Result::ok).collect(),
        Err(e) => {
            println!("Geçersiz desen: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        println!("Bu desenle eşleşen dosya veya dizin bulunamadı.");
        return;
    }

    println!("{} eşleşme bulundu:", entries.len());
    for entry in &entries {
        println!("  {}", entry.display());
    }

    if confirm("Yukarıdaki TÜM öğeler silinsin mi?") {
        for entry in entries {
            delete_path(entry.to_str().unwrap());
        }
    } else {
        println!("İptal edildi.");
    }
}

fn main() {
    println!("Vacuum Cleaner uygulamasına hoş geldiniz");
    println!("Lütfen bir seçenek seçin:");
    println!("1. Belirli bir dosya veya dizini temizle");
    println!("2. Bir desene uyan dosya/dizinleri temizle");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => clean_specific(),
        "2" => clean_by_pattern(),
        _ => println!("Geçersiz seçim"),
    }
}
