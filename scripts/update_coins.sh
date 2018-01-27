#!/bin/bash
set -e

BOTNAME=rich-uncle-pennybags-bot
LOCATION=/opt/$BOTNAME/

tput setaf 4
echo  -e "Copying Coinfile to server..."
tput sgr0
rsync -av  ./Coins.toml jnf:/opt/$BOTNAME/

tput setaf 4
echo  -e "Restarting services..."
tput sgr0
ssh jnf service $BOTNAME restart

tput setaf 2
echo  -e "Done deploying!"
tput sgr0
