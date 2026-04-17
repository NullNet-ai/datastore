FROM postgres:15-bullseye
RUN apt-get update \
  && apt-get install -y gnupg lsb-release curl ca-certificates \
  && curl -fsSL https://packagecloud.io/timescale/timescaledb/gpgkey | gpg --dearmor -o /usr/share/keyrings/timescaledb-keyring.gpg \
  && echo "deb [signed-by=/usr/share/keyrings/timescaledb-keyring.gpg] https://packagecloud.io/timescale/timescaledb/debian/ $(lsb_release -cs) main" > /etc/apt/sources.list.d/timescaledb.list \
  && apt-get update \
  && apt-get install -y timescaledb-2-postgresql-15 postgresql-15-cron \
  && rm -rf /var/lib/apt/lists/*
CMD ["postgres", "-c", "shared_preload_libraries=timescaledb,pg_cron"]
# SELECT * FROM pg_available_extensions WHERE name = 'pg_cron';
