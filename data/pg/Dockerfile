FROM postgres:13-alpine

WORKDIR /usr/src/div/data/pg

COPY ../../back/sql/* /usr/src/div/data/sql
COPY . /usr/src/div/data/pg

EXPOSE 5432

#VOLUME  ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]

