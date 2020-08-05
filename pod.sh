#!/bin/bash
#NOTE Local podman commands to run containerized images of different parts of web app
# and other services (db, redis, etc.)

red=`tput setaf 1`
green=`tput setaf 2`
bold=`tput bold`
und=`tput smul`
eund=`tput rmul`
res=`tput sgr0`

#TODO add params for port mapping
#TODO add postgres container deployment after sqlx is configured for postgres
#TODO figure out why this doesnt work with ansible playbook
#TODO link redis and postgres?

run_redis() {
    sudo podman build ~/lab/dv/div/data/redis -t divredis & sudo podman run -idt --rm -p 6379:6379 --name divredis --hostname divredis localhost/divredis
}

run_postgres() {
    sudo podman build ~/lab/dv/div/data/pg -t divdb & sudo podman run -idt --rm -p 5432:5432 -v //data/pg/pgdata:/var/lib/postgresql --env-file /usr/src/div/data/pg/pg.env --name divdb --hostname divdb localhost/divdb

}

run_sonic() {
    sudo podman run -p 1491:1491 -v /path/to/your/sonic/config.cfg:/etc/sonic.cfg -v /path/to/your/sonic/store/:/var/lib/sonic/store/ valeriansaliou/sonic:v1.3.0
}

#run_web() {

#}

#run_server() {

#}

kill_all() {
    sudo podman image rm --all --force
    sudo podman image prune --force
    sudo podman container rm --all --force
    sudo podman container prune --force
    sudo podman pod rm --all --force
    sudo podman pod prune --force
}

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
    kill_all
    echo "${green}Pods have been torn down.${res}"
elif [ "$1" == "check" ]; then
    echo "Checking pods...${res}"
elif [ "$1" == "test" ]; then
    echo -e "${red}ERR${res}"
elif [ "$1" == "pg" ]; then
    if [ "$2" == "up" ]; then
        run_postgres
    elif [ "$2" == "down" ]; then
        kill_all
    elif [ "$2" == "cli" ]; then
        sudo podman exec -it divdb sh -c 'psql'
    elif [ "$2" == "logs" ]; then
        sudo podman logs divdb
    else
        echo "${red}No valid argument passed${res}"
    fi
elif [ "$1" == "redis" ]; then
    if [ "$2" == "up" ]; then
        run_redis
    elif [ "$2" == "down" ]; then
        kill_all
    elif [ "$2" == "cli" ]; then
        sudo podman exec -it divredis sh -c "redis-cli"
    elif [ "$2" == "logs" ]; then
        sudo podman logs divredis
    else
        echo "${red}No valid argument passed${res}"
    fi

else
    echo "${red}No valid argument passed${res}"
fi
