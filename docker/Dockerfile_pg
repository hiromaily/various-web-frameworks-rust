# https://hub.docker.com/_/postgres
FROM postgres:16.3-alpine3.20 AS database
COPY ./pg/init.sql /docker-entrypoint-initdb.d/
ENV LANG ja_JP.utf8
