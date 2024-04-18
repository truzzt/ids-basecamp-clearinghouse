FROM gradle:7-jdk17 AS build

ARG VERSION
ARG GITHUB_USER
ARG GITHUB_TOKEN
ENV VERSION $VERSION
ENV GITHUB_USER $GITHUB_USER
ENV GITHUB_TOKEN $GITHUB_TOKEN

COPY --chown=gradle:gradle . /home/gradle/project/
WORKDIR /home/gradle/project/
RUN ./gradlew clean build \
-PdefaultVersion="${VERSION}" \
-Dorg.gradle.project.gitHubUserName="${GITHUB_USER}" \
-Dorg.gradle.project.gitHubUserPassword="${GITHUB_TOKEN}"

FROM openjdk:17-slim-buster

WORKDIR /app

COPY --from=build /home/gradle/project/launchers/connector-prod/build/libs/clearing-house-edc.jar /app

ENTRYPOINT [ "sh", "-c", "exec java $ENV_JVM_ARGS -jar clearing-house-edc.jar"]
