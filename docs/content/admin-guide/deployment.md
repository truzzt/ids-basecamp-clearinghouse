# Deployment

## Production

### Publishing the .well-known Route
The [ch-app](/ch-app_installation.md) makes the clearinghouse's public key accessible via the following route: /.well-known/jwks.json. To ensure proper routing, the reverse proxy must be configured to direct this path to the [ch-app](/ch-app_installation.md).

**Example Traefik Configuration**

For Docker:
```
deploy:
  labels:
    - traefik.http.routers.clearing-app-http.rule=Host(`${CLEARING_DOMAIN}`) && PathPrefix(`/.well-known/jwks.json`)
```
