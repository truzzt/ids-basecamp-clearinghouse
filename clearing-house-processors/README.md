# Clearing House Processors

## Testing
All tests are integration tests and will try to establish a TLS connection to an instance of the Clearing House. 
The tests will only run successfully if they can authenticate the peer (i.e. the Clearing House). 
To set up a local test environment, the docker container running the trusted connector for the Clearing House 
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
