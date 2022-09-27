# Clearing House Processors

## Building from Source
The Clearing House Processors are written in Java and require Java 11 and can be build using gradle (version 7.5+):

```
cd clearing-house-processors
./gradlew build
```

## Camel Routes
The Clearing House Processors include a file that contains the [routes](src/routes/clearing-house-routes.xml) used by [Apache Camel](https://camel.apache.org) (used in the Trusted Connector) to provide the endpoints of the Clearing House Service. The routes also contain some important steps that transform and forward data to the services of the Clearing House.

The routes define TLS endpoints and require access to the `keystore` and `truststore` used by the Trusted Connector. Currently, the passwords for both need to be configured in the routes file.

The routes also expect the `Logging Service` to be accessible via the docker-url `logging-service`. If this is not the case in your deployment, you will need to change this in the routes file.

## Testing
All tests are integration tests and will try to establish a TLS connection to an instance of the Clearing House. 
The tests will only run successfully if they can authenticate the peer (i.e. the Clearing House). 
To set up a local test environment, the docker container running the Trusted Connector for the Clearing House 
needs to be named `provider-core` and use the `provider-keystore.p12` as keystore.

The host running the test must include the line
```
127.0.0.1	provider-core
```

in its `/etc/hosts` file. Remote setups for testing will need to adapt the settings accordingly.

To run the tests use 
```
./gradlew integrationTest 
```
