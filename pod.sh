#!/bin/bash

red=`tput setaf 1`
green=`tput setaf 2`
bold=`tput bold`
und=`tput smul`
eund=`tput rmul`
res=`tput sgr0`


#TODO add params for port mapping
if [ "$1" == "up" ]; then
    FRONT_HOST_PORT=80
    FRONT_CON_PORT=5005
    BACK_HOST_PORT=3001
    BACK_CON_PORT=3001
    echo "${green}Building div.is containers...${res}"
    if [ "$2" == "spa" ]; then
        FRONT_CON_PORT=5000
    else
        FRONT_CON_PORT=5005
    fi
    if [ "$1" == "dev" ]; then
        echo "${green}${und}DEV: Running div.is front end container..."
        FRONT_HOST_PORT=$FRONT_CON_PORT
    elif [ "$2" == "prod" ]; then
        echo "${green}${und}PROD: Running div.is front end container..."
        FRONT_HOST_PORT=80
    else
        echo "ERR: Invalid param" 
        break
    fi
    sudo podman build back -t divb
    sudo podman build client -t divf
    sudo podman run -dt -p $FRONT_HOST_PORT:$FRONT_CON_PORT localhost/divf
    sudo podman run -dt -p $BACK_HOST_PORT:$BACK_CON_PORT localhost/divb
    echo "${green}${bold}Running frontend on http://localhost:$FRONTEND_HOST_PORT" 
    echo "${green}${bold}Running backend on http://localhost:$FRONTEND_HOST_PORT${res}" 
elif [ "$1" == "down" ]; then
    echo "${green}Tearing down pods...${res}"
    sudo podman image rm --all --force
    sudo podman image prune --force
    sudo podman pod rm --all --force
    sudo podman pod prune --force
    sudo podman container rm --all --force
    sudo podman container prune --force
    echo "${green}Pods have been torn down."
elif [ "$1" == "check" ]; then
    echo "Checking pods..."
elif [ "$1" == "test" ]; then
    echo -e "${red}ERR"
else
    echo "${red}No valid argument passed"
fi
