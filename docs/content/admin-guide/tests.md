# Tests


## Clearinghouse-edc

For the test clearinghouse-edc it uses Junit 5 and Jacoco for the coverage.

### Running Tests
To run the unit-tests execute the following command:

    ./gradlew test


### Test Coverage
To generate the tests coverage execute the following command:

    ./gradlew jacocoTestReport

The coverage reports will be available in the following folders:

- [core/build/reports/jacoco/test/html/index.html](./core/build/reports/jacoco/test/html/index.html)
- [extensions/multipart/build/reports/jacoco/test/html/index.html](./extensions/multipart/build/reports/jacoco/test/html/index.html)