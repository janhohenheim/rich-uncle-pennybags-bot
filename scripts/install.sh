#!/bin/bash
LOCATION=/opt/rich-uncle-pennybags-bot &&
echo  -e "\033[33;36m Updating source... \033[0m" &&
ssh jnf "(cd rich-uncle-pennybags-bot && git pull)" &&
scp .env jnf:~/rich-uncle-pennybags-bot/ &&
scp coins.toml jnf:$LOCATION &&

echo  -e "\033[33;36m Building release binaries... \033[0m" &&
ssh jnf "(cd rich-uncle-pennybags-bot && /root/.cargo/bin/cargo build --release)" &&

echo  -e "\033[33;36m Stopping services... \033[0m" &&
ssh jnf service rich-uncle-pennybags-bot stop &&

echo  -e "\033[33;36m Copying files to $LOCATION... \033[0m" &&
ssh jnf cp rich-uncle-pennybags-bot/target/release/rich-uncle-pennybags-bot $LOCATION &&

echo  -e "\033[33;36m Starting services... \033[0m" &&
ssh jnf service rich-uncle-pennybags-bot start &&

echo  -e "\033[33;32m Done deploying! \033[0m";
