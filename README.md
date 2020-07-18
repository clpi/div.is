# memuri (memri?)

build net: `sudo podman network create --name divnet`
build pod: `podman pod create --name divis --network divnet -p 3001:3001 -p 5432:5432` 

### to run front:
`cd client && npm run dev`

build image: `podman image build ./client -t divf`
create container: `podman container create localhost/divf:latest`
run container: `podman run -dt --pod divis divf` 

SPA on localhost:5000, SSR on localhost:5005. uses Svelte + Routify

### to run back:
`cd back && cargo run`
or
`cd back && cargo-watch -x run`

build image: `podman image build ./back -t divb`
create container: `podman container create localhost/divb:latest`
run container: `podman run -dt -p 80:5005 --pod divis divb` 

API on localhost:3000. Uses Warp + sqlx

## db:
run postgres: `podman run -d --pod divis -e POSTGRES_PASSWORD=password postgres:latest`
url: `postgresql://postgres:password@localhost:5432/postgres`

### deploying:
Currently using Podman and Ansible to deploy to DigitalOcean

### to do
- [ ] set up sqlx parts for postgres, automate postgres deployment/migrations/etc
- [ ] implement cookie sessions with OAuth2 instead of JWT
- [ ] generate kubernetes YML file for podman containerized deploy
- [ ] make ansible-playbook build and deploy to digitalocean 
- [ ] make minimal but functional dockerfiles for backend, frontend, and postgres db
