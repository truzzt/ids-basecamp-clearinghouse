# Clearinghouse: How it works

The Clearingouse consist of two services. The Clearinghouse-edc and the Clearinghouse-App. The Clearinghouse-edc is used to terminate IDS connections and map the requests to the API of the 
Clearinghouse-App. The Clearinghouse-App is the brain of the the Clearinghouse and uses complex algorithms to encrypt and store log messages in the clearinghouse and provide mechanisms to query for log messages.

## Clearinghouse-edc

## Clearinghouse-App

First of all the clearinghouse-app consisted before out of three separate microservices - logging, keyring, document - which were merged into one service. The reason for this is that the services were too tightly coupled and there was no benefit in having them separated. The new service is called just "clearinghouse-app".

### Functionality 

The Clearinghouse-App provides the following functionality: logging and querying of log messages. Adding and changing of DocumentTypes.

#### Log Message

The logging service (as an entity inside the remaining clearinghouse-app) is responsible for orchestrating the flow between document service and keyring service:

When logging a message, the message consists of two parts, originating from the IDS communication structure. There is a `header` and a `payload`. First part is to merge those two parts into a single struct (a Document).

The logging service creates a process id (if not exists) and checks the authorization.

After all prerequisites are checked and completed, it starts to get the transaction counter and assign i

### API

The API is described here in the [OpenAPI specification]().

#### 