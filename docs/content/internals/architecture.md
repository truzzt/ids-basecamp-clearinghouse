# Architecture

The Clearingouse consist of two services: The Clearinghouse-EDC and the Clearinghouse-App. The Clearinghouse-EDC is used to terminate IDS connections and map the requests to the API of the Clearinghouse-App. The Clearinghouse-App is the brain of the the Clearinghouse and uses complex algorithms to encrypt and store log messages in the MongoDB and provide mechanisms to query for log messages.

```d2
direction: right
ch: Clearinghouse {
    cha: Clearinghouse-App
    che: Clearinghouse-EDC
    m: MongoDB {
        shape: cylinder
    }

    che -> cha: REST
    cha -> m
}
c: Connector
c -> ch: IDS Multipart
```

> **Short history lesson**
> 
> The clearinghouse-app consisted before out of three separate microservices - logging, keyring, document - which were merged into one service. The reason for this is that the services were too tightly coupled and there was no benefit in having them separated. The new service is called just "clearinghouse-app".