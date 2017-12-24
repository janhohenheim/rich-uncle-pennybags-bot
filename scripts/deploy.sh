#!/bin/bash

echo  -e "\033[33;36m Building release binaries... \033[0m" &&
cargo build --release &&

echo  -e "\033[33;36m Stopping services... \033[0m" &&
ssh jnf service rich-uncle-pennybags-bot stop &&

LOCATION=jnf:/opt/rich-uncle-pennybags-bot &&
echo  -e "\033[33;36m Copying files to $LOCATION... \033[0m" &&
scp target/release/rich-uncle-pennybags-bot $LOCATION &&
# Todo: Find out why we can't terminate the next line in '&&', as it stops the script otherwise
scp .env $LOCATION &&

echo  -e "\033[33;36m Starting services... \033[0m" &&
ssh jnf service rich-uncle-pennybags-bot start &&

echo  -e "\033[33;32m Done deploying! \033[0m";
