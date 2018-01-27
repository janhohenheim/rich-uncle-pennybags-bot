#!/bin/bash
set -e

BOTNAME=rich-uncle-pennybags-bot
LOCATION=/opt/$BOTNAME/
tput setaf 4
echo  -e "Updating source..."
tput sgr0
ssh jnf "(cd $BOTNAME git pull)"

tput setaf 4
echo  -e "Building release binaries..."
tput sgr0
ssh jnf "(cd $BOTNAME /root/.cargo/bin/cargo build --release)"

tput setaf 4
echo  -e "Stopping services..."
tput sgr0
ssh jnf service $BOTNAME stop

tput setaf 4
echo  -e "Copying files to $LOCATION..."
tput sgr0
ssh jnf cp $BOTNAME/target/release/$BOTNAME $LOCATION
ssh jnf cp $BOTNAME/Coins.toml $LOCATION

tput setaf 4
echo  -e "Starting services..."
tput sgr0
ssh jnf service $BOTNAME start

tput setaf 2
echo  -e "\033[33;32m Done installing!";
tput sgr0
