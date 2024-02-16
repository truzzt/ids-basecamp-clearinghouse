const date = new Date();

export default () => {
  return {
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
    payload: '{ "owners": ["MDSBPN1.123", "MDSBPN2.123"]}'
  }
}
