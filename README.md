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
- [ ] generate kubernetes YML file for podman containerized deploy
- [X] make ansible-playbook build and deploy to digitalocean 
- [X] make minimal but functional dockerfiles for backend, frontend, and postgres db
- [ ] reconfigure sqlx/restructure queries/schema for postgres after setting up pg deployment
- [o] Record CRUD
    - [X] Record CRUD in backend 
    - [ ] Record CRUD functionality in frontend
- [ ] Item CRUD
    - [ ] Item CRUD in backend 
    - [ ] Item CRUD functionality in frontend
- [ ] currently need to look into exposig networking ports and pod networking for podman, so just run each container separately
- [X] get rust backend build to work properly
- [ ] figure out routing with pods / routing on instance for subdomain configuration (using traefik?)
- [ ] create yew parallel site (skeletal) for testing
- [ ] set up api credentials (for mobile app, other users, etc.)
- [ ] set up redis db dockerfile  
- [ ] set up fetch calls on client side to be called from express or otherwise to be called more elegantly and dynamically (api not on port 3001)
- [ ] set up postgres db migration automation
- [ ] set up in-instance / between container / in pod networking for client/api/db comms, instead of calling api url from .env
- [ ] add $ready to frontend to render prettily when data is fetched:
    - [ ] *IMPORTANT* TODO fix authentication scoping in frontend so it doesn't freeze on passing user data to child pages
- [ ] remove .env files and dotenv usage in backend, provision environmental variables in container specs
- [ ] *next* use certbot to get ssl certs
- [ ] set deploy script to run psql and rust backend on api.div.is, front on div.is (or api + front on div.is?)
- [ ] fix container run scripts in ansible playbook to actually tear down containers and images (also appears to create 2 of each? according to cockpit? look into that)
- [ ] don't build images, just run containers from
- [ ] don't reject cookie refresh requests -- return falsy value
- [ ] *important* actually implement frontend api fetching error handling properly
- [ ] consider entirely restructuring your frontend, now that fetching logic is done, there's nothing special about it besides how messy it is
- [ ] add ansible playbooks to automate provisioning of digitalocean droplets / ec2 instances
- [ ] optimize api fetches on frontend so read/write is minimized and redundant api calls are reduced
- [ ] store authentication memory server side instead of in svelte store (?)
- [ ] $07/25/20$ find out if there is *any way* to reduce the redundancy of passing and cloning db whenever a handler / route / model method is called
- [ ] $07/25/20$ benchmark to see whether it's faster to have all routes as variables in a function or modularized in separate modules/functions (or rather, how much of a performance hit it is)
