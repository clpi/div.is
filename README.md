# memuri (memri?)

build net: `sudo podman network create --name divnet`
build pod: `podman pod create --name divis --network divnet -p 3001:3001 -p 5432:5432` 

### to run front:
`cd client && npm run dev`

build image: `sudo podman image build client -t divf`
run container: `sudo podman run -dt -p 3001:3001 localhost/divf` 

SPA on localhost:5000, SSR on localhost:5005. uses Svelte + Routify

### to run back:
`cd back && cargo run`
or
`cd back && cargo-watch -x run`

build image: `sudo podman image build back -t divb`
run container: `sudo podman run -dt -p 80:5005 localhost/divb` 

API on localhost:3000. Uses Warp + sqlx

## db:
run postgres: `sudo podman run -dt -e POSTGRES_PASSWORD=password postgres:latest`
url: `postgresql://postgres:password@localhost:5432/postgres`

### deploying:
Currently using Podman and Ansible to deploy to DigitalOcean

### to do
- [ ] set up sqlx parts for postgres, automate postgres deployment/migrations/etc
- [ ] implement cookie sessions with OAuth2 instead of JWT
- [ ] generate kubernetes YML file for podman containerized deploy
- [X] make ansible-playbook build and deploy to digitalocean 
- [X] make minimal but functional dockerfiles for backend, frontend, and postgres db
- [ ] reconfigure sqlx/restructure queries/schema for postgres after setting up pg deployment
- [ ] add record CRUD functionality + display on frontend
- [ ] add entry CRUD funcitonality + display on frontend
- [ ] currently need to look into exposig networking ports and pod networking for podman, so just run each container separately
- [ ] get rust backend build to work properly
- [ ] figure out routing with pods / routing on instance for subdomain configuration (using traefik?)
- [ ] create yew parallel site (skeletal) for testing
- [ ] set up api credentials (for mobile app, other users, etc.)
- [ ] set up redis db dockerfile  
- [ ] set up fetch calls on client side to be called from express or otherwise to be called more elegantly and dynamically (api not on port 3001)
- [ ] set up postgres db migration automation
- [ ] set up in-instance / between container / in pod networking for client/api/db comms, instead of calling api url from .env
- [ ] add $ready to frontend to render prettily when data is fetched
- [ ] remove .env files and dotenv usage in backend, provision environmental variables in container specs
