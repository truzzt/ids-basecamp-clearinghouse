import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 1
};

const url = 'http://localhost:8000';

export default () => {
  const jwksRes = http.get(`${url}/.well-known/jwks.json`);
  check(jwksRes, {
    'ch-app GET jwks is status 200': (r) => r.status === 200,
  });

  const doctypeRes = http.get(`${url}/doctype`);
  check(doctypeRes, {
    'ch-app GET doctype is status 200': (r) => r.status === 200,
  });

  const logMessageHeader = {
    "Content-Type": "application/json",
    "CH-SERVICE": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjbGllbnRfaWQiOiI2OTpGNTo5RDpCMDpERDpBNjo5RDozMDo1Rjo1ODpBQToyRDoyMDo0RDpCMjozOTpGMDo1NDpGQzozQjprZXlpZDo0Rjo2Njo3RDpCRDowODpFRTpDNjo0QTpEMTo5NjpEODo3Qzo2QzpBMjozMjo4QTpFQzpBNjpBRDo0OSIsImlzcyI6IjY5OkY1OjlEOkIwOkREOkE2OjlEOjMwOjVGOjU4OkFBOjJEOjIwOjREOkIyOjM5OkYwOjU0OkZDOjNCOmtleWlkOjRGOjY2OjdEOkJEOjA4OkVFOkM2OjRBOkQxOjk2OkQ4OjdDOjZDOkEyOjMyOjhBOkVDOkE2OkFEOjQ5IiwiaWF0IjoxNjkzNTU2NDM2LCJuYmYiOjE2OTM1NTY0MzYsImV4cCI6MTY5MzU2MDAzNiwiYXVkIjoiMSJ9.WGZVbfJqK2bFwE8vEN29VeZzfPC2F_w2_bBkadNm4WM"
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


  sleep(1);
};
