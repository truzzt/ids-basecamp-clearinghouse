import http from 'k6/http';
import { check } from 'k6';
import logMessage from './util/logMessage.js';
import header from './util/header.js';

export const options = {
  vus: 10,
  duration: "1m"
};

const url = `http://${__ENV.HOSTNAME}`;
const TOKEN = `${__ENV.TOKEN}`

export default () => {
  const logMessageHeader = {
    "Content-Type": "application/json",
    "CH-SERVICE": TOKEN
  }

  const logMessageRes = http.post(`${url}/messages/log/6`, JSON.stringify(logMessage(), null, 2), { headers: header() });
  check(logMessageRes, {
    'ch-app POST logmessage is status 201': (r) => r.status === 201,
  });

};
