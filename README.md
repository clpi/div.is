# memuri (memri?)


### to run front:
cd front && npm run dev

SPA on localhost:5000, SSR on localhost:5005. uses Svelte + Routify

### to run back:
cd back && cargo run

API on localhost:3000. Uses Warp

### to run both:
chmod +x dev.sh && ./dev.sh

### to do
- [ ] set up sqlx parts for postgres, automate postgres deployment/migrations/etc
- [ ] implement password hashing, oauth2, passing jwt in session cookies etc
- [ ] make ansible build and deploy to digitalocean playbooks
- [ ] make dockerfiles and docker-compose stack yml files
