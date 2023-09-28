FROM openjdk:17-slim-buster

RUN apt update \
  && apt install -y curl \
  && rm -rf /var/cache/apt/archives /var/lib/apt/lists

WORKDIR /app

COPY ./build/libs/connector.jar /app

ENV WEB_HTTP_PORT="9191"
ENV WEB_HTTP_PATH="/api"

HEALTHCHECK --interval=5s --timeout=5s --retries=10 CMD curl --fail http://localhost:8181/api/check/health

ENTRYPOINT [ "sh", "-c", "exec java $ENV_JVM_ARGS -jar connector.jar"]
