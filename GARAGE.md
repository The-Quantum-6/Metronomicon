# Garage (file/object storage)

We use [Garage](https://garagehq.deuxfleurs.fr/), an S3-compatible object store,
for user-uploaded files (images, documents). The backend talks to it with the
official AWS S3 SDK (`aws-sdk-s3`), so the same code works against Amazon S3 later
if we ever switch.

Garage runs as the `garage` service in `compose.yaml`, configured by `garage.toml`.
Data lives in the named volumes `garage_meta` / `garage_data` and survives restarts.

## Setup

There is none. Garage v2.3+ runs with `--single-node --default-bucket`
(see compose.yaml), which auto-creates the cluster layout, the bucket and the
access key on first start. The bucket name and credentials come from `.env`
(`S3_BUCKET`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`), so:

```
cp .env.template .env          # if you don't have one already
docker compose up -d garage
```

The committed credentials in `.env.template` are local-dev-only values for a
localhost-only service (same reasoning as `rpc_secret` in `garage.toml`).
Never reuse them in production.

**Upgrading from the old v1.0.1 setup:** the old volumes are not compatible.
Wipe them and start fresh — it's only dev data:

```
docker compose down -v
docker compose up -d
```

## Verify it works

With Garage running and `backend/.env` in place (copy from `backend/.env.template`):

```
cd backend
cargo run --example garage_smoke
```

You should see it upload, list, and download a test file. That proves the backend
can reach storage.

## Notes

- Endpoint differs by where the code runs: `http://localhost:3900` on the host,
  `http://garage:3900` inside docker compose (set via `S3_ENDPOINT`).
- Garage only supports **path-style** addressing, so the SDK config sets
  `force_path_style(true)`. Forgetting this is the most common error.
- To reset storage completely: `docker compose down -v` wipes the volumes.
  A plain `docker compose down` keeps your data. Since setup is automatic,
  a wipe costs nothing but the stored files.
- The access key ID must start with `GK`. Generate a fresh pair with
  `echo "GK$(openssl rand -hex 16)"` and `openssl rand -hex 32`.
- The `rpc_secret` in `garage.toml` and the keys in `.env.template` are
  local-dev values only; do not reuse them in production.
