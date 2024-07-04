# Clearinghouse App Installation

The Clearinghouse App (`ch-app`) comes pre-packaged in a docker container.

## Releases

For existing releases visit [ids-basecamp-clearinghouse/ch-app Releases](https://github.com/truzzt/ids-basecamp-clearinghouse/pkgs/container/ids-basecamp-clearing/ch-app).

## Usage

Starting the `ch-app` Docker-only, use the following command and adapt it to your needs. 

```sh
docker run -d \
    -p 8000:8000 \
    -v ${PRIVATE_KEY_PATH}:/app/keys/private_key.der:ro \
    -e CH_APP_PROCESS_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_KEYRING_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_DOCUMENT_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_CLEAR_DB='false' \
    -e CH_APP_LOG_LEVEL='INFO' \
    -e CH_APP_STATIC_PROCESS_OWNER='MDS_ID' \
    -e SERVICE_ID_LOG='1' \
    -e SHARED_SECRET='123' \
    --name ch-app \
    ghcr.io/truzzt/ids-basecamp-clearing/ch-app:${TAG}
```

The following example starts the `ch-app` together with a `mongodb` also running on docker (good for local development):

```sh
# Create a docker network
docker network create testch
# Start mongodb
docker run -d -p 27017:27017 --net=testch --name mongohost mongo
# Start ch-app
docker run -d \
    -p 8000:8000 --net=testch \
    -v ${PRIVATE_KEY_PATH}:/app/keys/private_key.der:ro \
    -e CH_APP_PROCESS_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_KEYRING_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_DOCUMENT_DATABASE_URL='mongodb://mongohost:27017' \
    -e CH_APP_CLEAR_DB='false' \
    -e CH_APP_LOG_LEVEL='INFO' \
    -e CH_APP_STATIC_PROCESS_OWNER='MDS_ID' \
    -e SERVICE_ID_LOG='1' \
    -e SHARED_SECRET='123' \
    --name ch-app \
    ghcr.io/truzzt/ids-basecamp-clearing/ch-app:${TAG}

# ---
# Cleanup
docker rm -f mongohost ch-app
docker network rm testch
```

## Build

To build the ch-app yourself change into the `/clearing-house-app` directory and run `docker build -t ch-app:latest .`. 
