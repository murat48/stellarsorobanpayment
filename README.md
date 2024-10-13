# Stellar Payment Smart Contract

Bu proje, Stellar blockchain üzerinde çalışan ve Soroban SDK kullanılarak geliştirilmiş bir ödeme akıllı sözleşmesidir. Sözleşme, temel XLM transferlerini, tekrarlayan ödemeleri ve işlem geçmişi takibini yönetir.

## Özellikler

- XLM transfer işlemleri
- Tekrarlayan ödeme oluşturma ve yönetme
- İşlem geçmişi görüntüleme
- Bakiye sorgulama (şu anda sabit bir değer döndürür)
- Kullanıcı mesajları yönetimi

## Kurulum

1. Rust ve Cargo'yu yükleyin: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Soroban CLI'yı yükleyin: `cargo install soroban-cli`
3. Bu projeyi klonlayın: `git clone [REPO_URL]`
4. Proje dizinine gidin: `cd stellar-payment-contract`

## Derleme

Projeyi derlemek için aşağıdaki komutu çalıştırın:

```
cargo build --target wasm32-unknown-unknown --release
```

## Dağıtım

Sözleşmeyi Stellar test ağına dağıtmak için:

1. Soroban CLI'yı kullanarak bir Stellar hesabı oluşturun.
2. Test ağı için biraz test XLM alın.
3. Aşağıdaki komutu kullanarak sözleşmeyi dağıtın:

```
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/stellar_payment_contract.wasm --network testnet --source [YOUR_SECRET_KEY]
```

## Kullanım

Sözleşme dağıtıldıktan sonra, aşağıdaki fonksiyonları çağırabilirsiniz:

- `initialize`: Sözleşmeyi başlatır.
- `transfer_xlm`: XLM transferi yapar.
- `get_balance`: Bir adresin bakiyesini sorgular (şu anda sabit bir değer döndürür).
- `get_messages`: Bir kullanıcının mesajlarını alır.
- `create_recurring_payment`: Tekrarlayan bir ödeme oluşturur.
- `get_transaction_history`: Bir hesabın işlem geçmişini alır.
- `process_recurring_payments`: Zamanı gelen tekrarlayan ödemeleri işler.


## Lisans

Bu proje [MIT Lisansı](LICENSE) altında lisanslanmıştır.

## İletişim

Sorularınız veya geri bildirimleriniz için lütfen info@yazilimciburada.com adresine e-posta gönderin.
