FROM clickhouse/clickhouse-server:latest

ENV CLICKHOUSE_USER=default
ENV CLICKHOUSE_PASSWORD=111

COPY migrations/001_create_transfers.sql /docker-entrypoint-initdb.d/
