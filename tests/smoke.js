import http from 'k6/http';
import { check } from 'k6';
import logMessage from './util/logMessage.js';
import header from './util/header.js'

export const options = {
  vus: 2
};

const url = `http://${__ENV.HOSTNAME}`;

export default () => {
  const jwksRes = http.get(`${url}/.well-known/jwks.json`);
  check(jwksRes, {
    'ch-app GET jwks is status 200': (r) => r.status === 200,
  });

  const doctypeRes = http.get(`${url}/doctype`);
  check(doctypeRes, {
    'ch-app GET doctype is status 200': (r) => r.status === 200,
  });

  const logMessageRes = http.post(`${url}/messages/log/6`, JSON.stringify(logMessage(), null, 2), { headers: header() });
  check(logMessageRes, {
    'ch-app POST logmessage is status 201': (r) => r.status === 201,
  });
};
