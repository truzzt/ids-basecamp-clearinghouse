import http from 'k6/http';
import { check } from 'k6';
import logMessage from './util/logMessage.js';
import createProcess from './util/createProcess.js';
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

  const createProcessRes = http.post(`${url}/process/1`, JSON.stringify(createProcess(), null, 2), { headers: header() });
  check(createProcessRes, {
    'ch-app POST /process is status 201': (r) => r.status === 201,
  });


  const logMessageRes = http.post(`${url}/messages/log/1`, JSON.stringify(logMessage(), null, 2), { headers: header() });
  check(logMessageRes, {
    'ch-app POST /messages/log is status 201': (r) => r.status === 201,
  });

  const logMessageResNew = http.post(`${url}/messages/log/2`, JSON.stringify(logMessage(), null, 2), { headers: header() });
  check(logMessageResNew, {
    'ch-app POST /messages/log is status 201': (r) => r.status === 201,
  });
};
