Зашифрованные сообщения поверх Solana. Каждое сообщение - отдельный PDA-аккаунт, который может прочитать только получатель.

## Шифрование

Ключ получателя (ed25519) переводится в x25519, дальше ECDH с эфемерным ключом -> HKDF -> AES-256-GCM. Сообщение шифруется случайным ключом, а сам ключ - общим секретом. В аккаунте лежат эфемерный pubkey, зашифрованный ключ и зашифрованный текст.

## FFI

```c
typedef void (*Callback)(bool success, const char *msg);

void get_keypair(uint8_t *buffer);                                   // 64 байта
void airdrop(const char *pubkey, Callback cb);
void send_message_ffi(const uint8_t (*sk)[64], const char *to, const char *msg, Callback cb);
void get_messages_async_from_sk_bytes(const uint8_t (*sk)[64], Callback cb);  // msg = JSON
void rust_string_free(const char *p);
```
