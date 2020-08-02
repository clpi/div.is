#!/bin/bash

red=`tput setaf 1`
green=`tput setaf 2`
bold=`tput bold`
und=`tput smul`
eund=`tput rmul`
res=`tput sgr0`

# local scripts for development

app_run() {
    echo "${green}Running div.is.ion desktop app...${res}"
    cd app & yarn run dev & yarn tauri dev 
    echo "${green}Running div.is.ion on localhost:4000${res}"
}

web_run() {
    echo "${green}Building div.is svelte web app...${res}"
    cd client & yarn run dev
    echo "${green}Running div.is webapp on localhost:4000${res}"
}

server_run() {
    echo "${green}Running div.is warp server...${res}"
    cd server & cargo-watch -x run
    echo "${green}Running div.is server on localhost:3001${res}"
}

if [ "$1" == "app" ]; then

elif [ "$1" == "api" ]; then

elif [ "$1" == "web" ]; then
    echo "Checking pods...${res}"
elif [ "$1" == "test" ]; then
    echo -e "${red}ERR${res}"
elif [ "$1" == "all" ]; then
    echo -e "${red}ERR${res}"
else
    echo "${red}No valid argument passed${res}"
fi
