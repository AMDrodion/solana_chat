build:
	cargo +solana build --target sbpf-solana-solana --release
keygen:
	solana-keygen new -o ./target/deploy/chat_program-keypair.json
deploy:
	solana program-v4 deploy ./target/sbpf-solana-solana/release/chat_program.so --program-keypair ./target/deploy/chat_program-keypair.json --authority ~/.config/solana/id.json --url https://api.devnet.solana.com
redeploy:
	 solana program-v4 deploy ./target/sbpf-solana-solana/release/chat_program.so --program-id ./target/deploy/chat_program-keypair.json
