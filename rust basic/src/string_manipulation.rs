fn main() {
    let name = String::from("OtonG SuRoTonG MarKoTONG");

    println!("Uppercase from '{}' to '{}'", name, name.to_uppercase());
    println!("Lowercase from '{}' to '{}'", name, name.to_lowercase());
    println!("The length of '{}' is '{}' characters", name, name.len());
    println!(
        "Replace from '{}' to '{}'",
        name,
        name.replace("OtonG", "Sotong")
    );
    println!("Is {} contains tong -> {}", name, name.contains("tong"));
    println!(
        "Is {} contains MarKoTONG -> {}",
        name,
        name.contains("MarKoTONG")
    );
    println!("Is {} start with OtonG -> {}", name, name.contains("OtonG"));
    println!(
        "Is {} end with MarKoTONG -> {}",
        name,
        name.contains("MarKoTONG")
    );
    println!("Trim from {} to {}", name, name.trim());
    println!("Get 0 until 5 char from {} to {:?}", name, name.get(0..5));
}
