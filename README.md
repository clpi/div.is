# div.is

### to run front:
`cd client && npm run dev`

SPA on localhost:5000, SSR on localhost:5005. uses Svelte + Routify

### to run back:
`cd back && cargo run`
or
`cd back && cargo-watch -x run`

API on localhost:3001. Uses Warp + sqlx

## db:
wip (using sqlite right now)

### deploying:
Currently using Podman and Ansible to deploy to DigitalOcean

### to do
- [o] set up sqlx parts for postgres, automate postgres deployment/migrations/etc
    - [X] implement postgres deployment in podman deployment ansible playbook
    - [ ] implement postgres in sqlx in backend
- [ ] implement cookie sessions with OAuth2 instead of JWT
- [o] Record CRUD
    - [X] Record CRUD in backend 
    - [ ] Record CRUD functionality in frontend
- [ ] Item CRUD
    - [ ] Item CRUD in backend 
    - [ ] Item CRUD functionality in frontend
- [ ] currently need to look into exposig networking ports and pod networking for podman, so just run each container separately
- [ ] figure out routing with pods / routing on instance for subdomain configuration (using traefik?)
- [ ] set up api credentials (for mobile app, other users, etc.)
- [ ] set up redis db dockerfile  
- [ ] set up fetch calls on client side to be called from express or otherwise to be called more elegantly and dynamically (api not on port 3001)
- [ ] set up postgres db migration automation
- [ ] set up in-instance / between container / in pod networking for client/api/db comms, instead of calling api url from .env
- [ ] add $ready to frontend to render prettily when data is fetched:
- [ ] remove .env files and dotenv usage in backend, provision environmental variables in container specs
- [ ] *next* use certbot to get ssl certs
- [ ] don't reject cookie refresh requests -- return falsy value
