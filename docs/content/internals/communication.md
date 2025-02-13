# Communication

The APIs are documented in the following dscriptions:
- Connector to Clearinghouse: [IDS-G](https://github.com/International-Data-Spaces-Association/IDS-G/tree/main/Communication/protocols/multipart)
- Clearinghouse to Clearinghouse-App: [OpenAPI](https://github.com/truzzt/ids-basecamp-clearinghouse-postman/blob/main/index.yaml)

The following section contains examples of the communication between the components.

## Connector to Clearinghouse-EDC

The Clearinghouse-EDC received IDS-Multipart messages of the type `ids:LogMessage` in the *header* and an arbitrary *payload*. The following shows an example of a multipart message:

```
POST /messages/log/1 HTTP/1.1
Host: ch-ids.aisec.fraunhofer.de
Content-Type: multipart/form-data; boundary=X-TEST-REQUEST-BOUNDARY
Accept: */*

--X-TEST-REQUEST-BOUNDARY
Content-Disposition: form-data; name="header"
Content-Type: application/json
{
  "@context" : {
    "ids" : "https://w3id.org/idsa/core/",
    "idsc" : "https://w3id.org/idsa/code/"
  },
  "@type" : "ids:LogMessage",
  "@id" : "https://w3id.org/idsa/autogen/logMessage/c6c15a90-7799-4aa1-ac21-9323b87a7xv9",
  "ids:securityToken" : {
    "@type" : "ids:DynamicAttributeToken",
    "@id" : "https://w3id.org/idsa/autogen/dynamicAttributeToken/6378asd9-480d-80df-c5cb02e4e260",
    "ids:tokenFormat" : {
      "@id" : "idsc:JWT"
    },
    "ids:tokenValue" : "eyJ0eXAiOiJKV1QiLCJraWQiOiJkZWZhdWx0IiwiYWxnIjoi....."
  },
  "ids:senderAgent" : "http://example.org",
  "ids:modelVersion" : "4.1.0",
  "ids:issued" : {
    "@value" : "2020-12-14T08:57:57.057+01:00",
    "@type" : "http://www.w3.org/2001/XMLSchema#dateTimeStamp"
  },
  "ids:issuerConnector" : {
    "@id" : "https://companyA.com/connector/59a68243-dd96-4c8d-88a9-0f0e03e13b1b"
  }
}
--X-TEST-REQUEST-BOUNDARY
Content-Disposition: form-data; name="payload"
Content-Type: application/json
{
  "@context" : "https://w3id.org/idsa/contexts/context.jsonld",
  "@type" : "ids:ConnectorUpdateMessage",
  "id" : "http://industrialdataspace.org/connectorAvailableMessage/34d761cf-5ca4-4a77-a7f4-b14d8f75636a",
  "issued" : "2019-12-02T08:25:08.245Z",
  "modelVersion" : "4.1.0",
  "issuerConnector" : "https://companyA.com/connector/59a68243-dd96-4c8d-88a9-0f0e03e13b1b",
  "securityToken" : {
    "@type" : "ids:DynamicAttributeToken",
    "tokenFormat" : "https://w3id.org/idsa/code/tokenformat/JWT",
    "tokenValue" : "eyJhbGciOiJSUzI1NiIsInR5cCI..."
}
--X-TEST-REQUEST-BOUNDARY--
```

## Clearinghouse-EDC to Clearinghouse-App

The Clearinghouse-EDC extracts the *header* and *payload* and forwards it to the Clearinghouse-App via REST. The message looks like this:

```json
{
    "header": {
        "@context" : {
            "ids" : "https://w3id.org/idsa/core/",
            "idsc" : "https://w3id.org/idsa/code/"
        },
        "@type" : "ids:LogMessage",
        "@id" : "https://w3id.org/idsa/autogen/logMessage/c6c15a90-7799-4aa1-ac21-9323b87a7xv9",
        "ids:securityToken" : {
            "@type" : "ids:DynamicAttributeToken",
            "@id" : "https://w3id.org/idsa/autogen/dynamicAttributeToken/6378asd9-480d-80df-c5cb02e4e260",
            "ids:tokenFormat" : {
            "@id" : "idsc:JWT"
            },
            "ids:tokenValue" : "eyJ0eXAiOiJKV1QiLCJraWQiOiJkZWZhdWx0IiwiYWxnIjoi....."
        },
        "ids:senderAgent" : "http://example.org",
        "ids:modelVersion" : "4.1.0",
        "ids:issued" : {
            "@value" : "2020-12-14T08:57:57.057+01:00",
            "@type" : "http://www.w3.org/2001/XMLSchema#dateTimeStamp"
        },
        "ids:issuerConnector" : {
            "@id" : "https://companyA.com/connector/59a68243-dd96-4c8d-88a9-0f0e03e13b1b"
        }
    },
    "payload": {
        "@context" : "https://w3id.org/idsa/contexts/context.jsonld",
        "@type" : "ids:ConnectorUpdateMessage",
        "id" : "http://industrialdataspace.org/connectorAvailableMessage/34d761cf-5ca4-4a77-a7f4-b14d8f75636a",
        "issued" : "2019-12-02T08:25:08.245Z",
        "modelVersion" : "4.1.0",
        "issuerConnector" : "https://companyA.com/connector/59a68243-dd96-4c8d-88a9-0f0e03e13b1b",
        "securityToken" : {
            "@type" : "ids:DynamicAttributeToken",
            "tokenFormat" : "https://w3id.org/idsa/code/tokenformat/JWT",
            "tokenValue" : "eyJhbGciOiJSUzI1NiIsInR5cCI..."
        }
    }
}
```

## Clearinghouse-App to Clearinghouse-EDC

```json
{
    "data": "eyJhbGciOiJQUzUxMiIsImtpZCI6IlFyYS8vMjlGcnhiajVoaDVBemVmK0czNlNlaU9tOXE3czgrdzh1R0xEMjgifQ.eyJ0cmFuc2FjdGlvbl9pZCI6IjAwMDAwMDAwIiwidGltZXN0YW1wIjoxNjk2NDExMTM2LCJwcm9jZXNzX2lkIjoiMSIsImRvY3VtZW50X2lkIjoiNmNkNDQwNjQtZWFjNi00NmQzLWFhZTUtODcxYjgwYjU4OWMxIiwicGF5bG9hZCI6Int9IiwiY2hhaW5faGFzaCI6IjAiLCJjbGllbnRfaWQiOiJGNjoyNTo1ODpDNTo2MTo2ODo3QToyMTpGMTo0MDo5Rjo0RTpGQjo5NTpEQjo5OTo4ODpDOTpBNzoxQTpDNTpGODpCRjo0Qzo1NToxODo1NjozNTozNTo0MzpDNTpEQzo5NDpCNTpFQjo0NTozMDpGNTpBRjpDRSIsImNsZWFyaW5nX2hvdXNlX3ZlcnNpb24iOiIwLjEwLjAifQ.eo1KoF9gAZLF7CuhuQ-Sd9WSjw6dvDsrmM8w-A-FdTl4cOaPqp75k9O0tKxY8_ZNBsWmOzBzAfGng6YdvpDHIw9xFZTA7N_UMjTrrPuc8ehrVO2rwltTKb8N2bK4bQ4_Uq22Kd8mSFI6IyOZ7KeTkZ_iN30PXlYFAdt2GQHoT7xNERyQbHNEkJmOgGnaraMv0xEbl2zJktQqkTH9Kk4ZF2T_GbxKInhVxUhOsJ707ZeQ2Nxk4H6yO2RXwG5yKXFkwBDOMLg1f0Dnrgz_H1f-fQ7gPOrAL_4G4L7M9o7EVkMJlMpJR1xNBCeYbT_IvfL1CB5gi1NF-VNzt-8Zg5Yj-vNNR9j38yZTe6vH2dMkGl20B99KrEKTjkyVkCUIKnlb3oEKldse0E4ouw9v6WnIWq33-KnGV0ajwZrs13bQLZyLWvdNCBmYA5NujzbqOGkDROXloAB6MXBm5KiGTU8FxrqS6s_J7OW1CLTlAlTFF_U2Tr1xSvcusnpOGrU22IrCuqVuGCNNGCrPYjKJmMc05wIG0cmdxTdRnoe8R-vOVg2Zd07jdrBLX5l5tZtF60LC8DZKw4k2JaCu37W_dXdWHLSXEnpR9MGgnqC8MbOAMIIzSXpWKFdXcS-86SkgTvDA16geN_Bj7Ac6xcuUnEhM3_9tVnpjNMgPcStyO0KiP3c"
}
```

## Clearinghouse-EDC to Connector

```
--Boundary_1_377557244_1696411137008
Content-Type: application/json
Content-Disposition: form-data; name="header"

{
    "@context": {
        "ids": "https://w3id.org/idsa/core/",
        "idsc": "https://w3id.org/idsa/code/"
    },
    "@id": "urn:message:92a2da5a-b5de-4709-bda9-c16a0ae293f6",
    "@type": "ids:MessageProcessedNotificationMessage",
    "ids:securityToken": {
        "@id": "https://w3id.org/idsa/autogen/dynamicAttributeToken/6378asd9-480d-80df-c5cb02e4e260",
        "@type": "ids:DynamicAttributeToken",
        "ids:tokenFormat": {
            "@id": "idsc:JWT"
        },
        "ids:tokenValue": "eyJ0eXAiOiJhdCtqd3QiLCJraWQiOiJkNzRlYzU1MGY0MzkxYTAwZGIwODA5Mzg5MjdjOGU4YWQ0NjE3NmM4NGQ3MzhkZGMwODM1ODMzYzM5YWJkMzRhIiwiYWxnIjoiUlMyNTYifQ.eyJzY29wZSI6Imlkc2M6SURTX0NPTk5FQ1RPUl9BVFRSSUJVVEVTX0FMTCIsImF1ZCI6WyJpZHNjOklEU19DT05ORUNUT1JTX0FMTCJdLCJpc3MiOiJkYXBzLmRlbW8udHJ1enp0cG9ydC5jb20iLCJzdWIiOiJGNjoyNTo1ODpDNTo2MTo2ODo3QToyMTpGMTo0MDo5Rjo0RTpGQjo5NTpEQjo5OTo4ODpDOTpBNzoxQTpDNTpGODpCRjo0Qzo1NToxODo1NjozNTozNTo0MzpDNTpEQzo5NDpCNTpFQjo0NTozMDpGNTpBRjpDRSIsIm5iZiI6MTY5NjQxMTAxNiwiaWF0IjoxNjk2NDExMDE2LCJqdGkiOiI0MjY2OTY0NC01MzgzLTQ2NDYtYmMxMC0zMzJlMzRkMjdmNGMiLCJleHAiOjE2OTY0MTQ2MTYsImNsaWVudF9pZCI6IkY2OjI1OjU4OkM1OjYxOjY4OjdBOjIxOkYxOjQwOjlGOjRFOkZCOjk1OkRCOjk5Ojg4OkM5OkE3OjFBOkM1OkY4OkJGOjRDOjU1OjE4OjU2OjM1OjM1OjQzOkM1OkRDOjk0OkI1OkVCOjQ1OjMwOkY1OkFGOkNFIn0.sa2zCMCwap7KjqV6RkzQ4jeR-nMPXo546oqxSzyZSPamhfkPc35LfldZTkuX_gxy6P1Ra2ltrannQTH7467FC8H00giF3mamZ_LuyUHMRUZzab0UvNJaGqt1mJZaMiOnupixP1cUhsXszfmCRKXWvatbwvlc0nhw5gdO2lH_njWBrXUy5Bt2MIIFp892ijf_rP5KC7yfa0cW9lwTFuWZYMMRBeOfY_g1Mx_YVkQXy9mFI0x3zC6rms8jq8OWRompNfkQ7mZsiFPAafls2f0iP8M2HKWA8JeOG5rkAIw0ESWSVT7iB-oV50LlX7L7zAYVLGdDyM3s_khDNxrbvlW_bQ"
    },
    "ids:issuerConnector": {
        "@id": "urn:connector:example-connector"
    },
    "ids:modelVersion": "4.1.3",
    "ids:issued": {
        "@value": "2023-10-04T09:18:56.998Z",
        "@type": "http://www.w3.org/2001/XMLSchema#dateTimeStamp"
    },
    "ids:senderAgent": {
        "@id": "urn:connector:example-connector"
    }
}
--Boundary_1_377557244_1696411137008
Content-Type: application/json
Content-Disposition: form-data; name="payload"

{
    "data": "eyJhbGciOiJQUzUxMiIsImtpZCI6IlFyYS8vMjlGcnhiajVoaDVBemVmK0czNlNlaU9tOXE3czgrdzh1R0xEMjgifQ.eyJ0cmFuc2FjdGlvbl9pZCI6IjAwMDAwMDAwIiwidGltZXN0YW1wIjoxNjk2NDExMTM2LCJwcm9jZXNzX2lkIjoiMSIsImRvY3VtZW50X2lkIjoiNmNkNDQwNjQtZWFjNi00NmQzLWFhZTUtODcxYjgwYjU4OWMxIiwicGF5bG9hZCI6Int9IiwiY2hhaW5faGFzaCI6IjAiLCJjbGllbnRfaWQiOiJGNjoyNTo1ODpDNTo2MTo2ODo3QToyMTpGMTo0MDo5Rjo0RTpGQjo5NTpEQjo5OTo4ODpDOTpBNzoxQTpDNTpGODpCRjo0Qzo1NToxODo1NjozNTozNTo0MzpDNTpEQzo5NDpCNTpFQjo0NTozMDpGNTpBRjpDRSIsImNsZWFyaW5nX2hvdXNlX3ZlcnNpb24iOiIwLjEwLjAifQ.eo1KoF9gAZLF7CuhuQ-Sd9WSjw6dvDsrmM8w-A-FdTl4cOaPqp75k9O0tKxY8_ZNBsWmOzBzAfGng6YdvpDHIw9xFZTA7N_UMjTrrPuc8ehrVO2rwltTKb8N2bK4bQ4_Uq22Kd8mSFI6IyOZ7KeTkZ_iN30PXlYFAdt2GQHoT7xNERyQbHNEkJmOgGnaraMv0xEbl2zJktQqkTH9Kk4ZF2T_GbxKInhVxUhOsJ707ZeQ2Nxk4H6yO2RXwG5yKXFkwBDOMLg1f0Dnrgz_H1f-fQ7gPOrAL_4G4L7M9o7EVkMJlMpJR1xNBCeYbT_IvfL1CB5gi1NF-VNzt-8Zg5Yj-vNNR9j38yZTe6vH2dMkGl20B99KrEKTjkyVkCUIKnlb3oEKldse0E4ouw9v6WnIWq33-KnGV0ajwZrs13bQLZyLWvdNCBmYA5NujzbqOGkDROXloAB6MXBm5KiGTU8FxrqS6s_J7OW1CLTlAlTFF_U2Tr1xSvcusnpOGrU22IrCuqVuGCNNGCrPYjKJmMc05wIG0cmdxTdRnoe8R-vOVg2Zd07jdrBLX5l5tZtF60LC8DZKw4k2JaCu37W_dXdWHLSXEnpR9MGgnqC8MbOAMIIzSXpWKFdXcS-86SkgTvDA16geN_Bj7Ac6xcuUnEhM3_9tVnpjNMgPcStyO0KiP3c"
}
--Boundary_1_377557244_1696411137008--
```