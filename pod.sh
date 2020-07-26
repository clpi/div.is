#!/bin/bash

red=`tput setaf 1`
green=`tput setaf 2`
bold=`tput bold`
und=`tput smul`
eund=`tput rmul`
res=`tput sgr0`


#TODO add params for port mapping
#TODO add postgres container deployment after sqlx is configured for postgres
#TODO figure out why this doesnt work with ansible playbook
if [ "$1" == "up" ]; then
    FRONT_HOST_PORT=80
    FRONT_CON_PORT=5005
    BACK_HOST_PORT=3001
    BACK_CON_PORT=3001
    ENV="dev"
    echo "${green}Building div.is containers...${res}"
    if [ "$3" == "spa" ]; then
        FRONT_CON_PORT=5000
    else
        FRONT_CON_PORT=5005
    fi
    if [ "$2" == "dev" ]; then
        echo "${green}${und}DEV: Running div.is front end container...${res}"
        FRONT_HOST_PORT=5005
        FRONT_CON_PORT=5005
        BACK_HOST_PORT=3001
        BACK_CON_PORT=3001
        ENV="dev"
        sudo podman build server -f Dockerfile.dev -t divbdev
        sudo podman build client -f Dockerfile.dev -t divfdev
        sudo podman run --rm -dt -p $FRONT_HOST_PORT:$FRONT_CON_PORT --name divfdev localhost/divfdev
        sudo podman run --rm -dt -p $BACK_HOST_PORT:$BACK_CON_PORT --name divb localhost/divbdev
    elif [ "$2" == "prod" ]; then
        echo "${green}${und}PROD: Running div.is front end container...${res}"
        FRONT_HOST_PORT=80
        FRONT_CON_PORT=5005
        BACK_HOST_PORT=80
        BACK_CON_PORT=3001
        ENV="prod"
        sudo podman build back -t divbprod
        sudo podman build client -t divfprod
        sudo podman run --rm -dt -p $FRONT_HOST_PORT:$FRONT_CON_PORT --name divfprod localhost/divfprod
        sudo podman run --rm -dt -p $BACK_HOST_PORT:$BACK_CON_PORT --name divb localhost/divbprod
    else
        break
    fi
    echo "${green}${bold}Running frontend on http://localhost:$FRONTEND_HOST_PORT" 
    echo "${green}${bold}Running backend on http://localhost:$FRONTEND_HOST_PORT${res}" 
elif [ "$1" == "down" ]; then
    echo "${green}Tearing down pods...${res}"
    sudo podman image rm --all --force
    sudo podman image prune --force
    sudo podman container rm --all --force
    sudo podman container prune --force
    sudo podman pod rm --all --force
    sudo podman pod prune --force
    echo "${green}Pods have been torn down.${res}"
elif [ "$1" == "check" ]; then
    echo "Checking pods...${res}"
elif [ "$1" == "test" ]; then
    echo -e "${red}ERR${res}"
else
    echo "${red}No valid argument passed${res}"
fi
