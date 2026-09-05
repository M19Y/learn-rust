/* Enum
*
*
*/

enum Level {
    Regular,
    Premium,
    Platinum,
}

fn main() {
    let _level1: Level = Level::Regular;
    let _level2: Level = Level::Premium;
    let _level3: Level = Level::Platinum;
}

// enum data
enum Payment {
    // card number
    CreditCard(String),

    // bank name, account number
    BankTransfer(String, String),

    // e-wallet name, e-wallet number
    EWallet(String, String),
}

// enum method
impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying amount: {}", amount);
    }
}

#[test]
fn enum_data() {
    let payment_via_bank: Payment =
        Payment::BankTransfer(String::from("BRI"), String::from("2121"));
    payment_via_bank.pay(10_000);

    let payment_via_credit_card: Payment = Payment::CreditCard(String::from("1234567890"));
    payment_via_credit_card.pay(50_000);

    let payment_via_e_wallet: Payment =
        Payment::EWallet(String::from("Dana"), String::from("2121"));
    payment_via_e_wallet.pay(20_000_000);
}
