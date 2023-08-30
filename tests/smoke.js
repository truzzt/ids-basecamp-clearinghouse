import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 1, // Key for Smoke test. Keep it at 2, 3, max 5 VUs,
};

const url = 'http://localhost:8000';

export default () => {
  const jwksRes = http.get(`${url}/.well-known/jwks.json`);
  check(jwksRes, {
    'ch-app GET jwks is status 200': (r) => r.status === 200,
  });

  const logMessageResHeader = {
    content
  }

  const logMessageRes = http.post(`${url}/messages/log/1`, {}. {});
  check(logMessageRes, {
    'ch-app POST jwks is status 200': (r) => r.status === 200,
  });


  sleep(1);
};
