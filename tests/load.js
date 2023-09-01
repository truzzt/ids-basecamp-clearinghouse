import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 10,
  duration: "1m"
};

const url = 'http://localhost:8000';
const TOKEN = 'xxx'

export default () => {
  const logMessageHeader = {
    "Content-Type": "application/json",
    "CH-SERVICE": TOKEN
  }

  const date = new Date();
  const logMessagePayload = {
    "header": {

      "@context": {
          // ... (HashMap<String, String>)
      },
      "@type": "ids:LogMessage",
      "@id": "String",
      "modelVersion": "String",
      "correlationMessage": "String",
      "issued": date.toISOString(),
      "issuerConnector": "InfoModelId",
      "senderAgent": "String",
      "recipientConnector": [
          "test"
      ],
      "recipientAgent": [
         "test"
      ],
      "transferContract": "String",
      "contentVersion": "String",
      "securityToken": null,
      "authorizationToken": "String",
      "payload": "String",
      "payload_type": "String"
    },
    payload: "hello world"
  }
  const logMessageRes = http.post(`${url}/messages/log/6`, JSON.stringify(logMessagePayload, null, 2), { headers: logMessageHeader });
  check(logMessageRes, {
    'ch-app POST logmessage is status 201': (r) => r.status === 201,
  });

};
