#!/bin/bash
set -e

BOTNAME=rich-uncle-pennybags-bot 
LOCATION=/opt/$BOTNAME/ 
tput setaf 4
echo  -e "Building release binaries..." 
tput sgr0
cargo build --release 

tput setaf 4
echo  -e "Copying binaries to server..." 
tput sgr0
rsync -av ./target/release/$BOTNAME jnf:~/$BOTNAME/target/release/$BOTNAME 

tput setaf 4
echo  -e "Copying Coinfile to server..." 
tput sgr0
rsync -av  ./Coins.toml jnf:~/$BOTNAME/Coins.toml 

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
echo  -e "\033[33;32m Done deploying!";
tput sgr0
