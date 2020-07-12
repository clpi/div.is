FROM postgres:13-alpine
COPY ../back/sql/*.sql .
WORKDIR /usr/app
ENV PG_USER "admin"
ENV PG_PASS "pass"
COPY . /usr/app/pg

