FROM bitnami/redis:latest-alpine

COPY redis.conf /usr/src/div/data/redis/redis.conf

EXPOSE 6379

CMD [ "redis-server", "/usr/src/div/data/redis/redis.conf" ]
