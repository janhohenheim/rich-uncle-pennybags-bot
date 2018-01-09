#!/bin/bash
BOTNAME=rich-uncle-pennybags-bot &&
LOCATION=/opt/$BOTNAME/ &&

echo  -e "\033[33;36m Copying Coinfile to server... \033[0m" &&
rsync -av  ./Coins.toml jnf:/opt/$BOTNAME/ &&

echo  -e "\033[33;36m Restarting services... \033[0m" &&
ssh jnf service $BOTNAME restart &&

echo  -e "\033[33;32m Done deploying! \033[0m";