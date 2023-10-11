FROM gradle:7-jdk17 AS build

COPY --chown=gradle:gradle . /home/gradle/project/
WORKDIR /home/gradle/project/
RUN ./gradlew clean build
RUN ls -la


FROM openjdk:17-slim-buster

WORKDIR /app


COPY --from=build /home/gradle/project/build/libs/clearing-house-edc.jar /app

ENV WEB_HTTP_PORT="8181"
ENV WEB_HTTP_PATH="/api"

ENTRYPOINT [ "sh", "-c", "exec java $ENV_JVM_ARGS -jar clearing-house-edc.jar"]
