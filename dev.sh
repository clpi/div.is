#!/bin/sh

run() {
    if [ "$1" != "" ]; then
    cd back && cargo-watch -x run &
    cd ../client && npm run dev
}

build() {


}

deploy() {

}

