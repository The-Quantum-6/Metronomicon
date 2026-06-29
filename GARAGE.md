# Garage (file/object storage)

We use [Garage](https://garagehq.deuxfleurs.fr/), an S3-compatible object store,
for user-uploaded files (images, documents). The backend talks to it with the
official AWS S3 SDK (`aws-sdk-s3`), so the same code works against Amazon S3 later
if we ever switch.

Garage runs as the `garage` service in `compose.yaml`, configured by `garage.toml`.
Data lives in the named volumes `garage_meta` / `garage_data` and survives restarts.

## One-time setup

After the first time you start Garage, you must assign it a storage layout once.
The bucket and access keys you create then persist in the volumes.

1. **Start Garage:**

   ```
   docker compose up -d garage
   ```

2. **Find the node ID** (look for `NO ROLE ASSIGNED`):

   ```
   docker compose exec garage /garage status
   ```

3. **Assign and apply a layout** (use the node ID from step 2):

   ```
   docker compose exec garage /garage layout assign -z dc1 -c 1G <node-id>
   docker compose exec garage /garage layout apply --version 1
   ```

4. **Create the bucket and an access key:**

   ```
   docker compose exec garage /garage bucket create test-bucket
   docker compose exec garage /garage key create backend-key
   docker compose exec garage /garage bucket allow --read --write test-bucket --key backend-key
   ```

   `key create` prints a **Key ID** (`GK...`) and a **Secret key** — copy both.

5. **Put the keys in your `.env`** (both `./.env` for docker and `backend/.env`
   for running the backend on the host):

   ```
   AWS_ACCESS_KEY_ID=GK...
   AWS_SECRET_ACCESS_KEY=...
   ```

   The `.env` files are gitignored, so keys never get committed.

## Verify it works

With Garage running and keys in `backend/.env`:

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
- To reset storage completely: `docker compose down -v` wipes the volumes — then
  redo the one-time setup. A plain `docker compose down` keeps your data.
- The `rpc_secret` in `garage.toml` is a local-dev value only; do not reuse it
  in production.
