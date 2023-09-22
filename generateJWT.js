const jwt = require('jsonwebtoken')

const payload = {
    "client_id": "69:F5:9D:B0:DD:A6:9D:30:5F:58:AA:2D:20:4D:B2:39:F0:54:FC:3B:keyid:4F:66:7D:BD:08:EE:C6:4A:D1:96:D8:7C:6C:A2:32:8A:EC:A6:AD:49",
    "iss": "69:F5:9D:B0:DD:A6:9D:30:5F:58:AA:2D:20:4D:B2:39:F0:54:FC:3B:keyid:4F:66:7D:BD:08:EE:C6:4A:D1:96:D8:7C:6C:A2:32:8A:EC:A6:AD:49",
    "iat": Math.floor(Date.now() / 1000),
    "nbf": Math.floor(Date.now() / 1000),
    "exp": Math.floor(Date.now() / 1000) + 3600,
    "aud": "1"
}

jwt.sign(payload, "123", { algorithm: 'HS256' }, function(err, token) {
  console.log(token);
});