/*
 *  Copyright (c) 2023 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       truzzt GmbH - Initial implementation
 *
 */

plugins {
    `java-library`
    `java-test-fixtures`
    jacoco
}

val auth0JWTVersion: String by project

dependencies {
    api(edc.spi.core)

    implementation(edc.ids)
    implementation(edc.ids.jsonld.serdes)
    implementation(edc.api.management.config)
    implementation(libs.jersey.multipart)
    implementation("com.auth0:java-jwt:${auth0JWTVersion}")

    testImplementation(libs.junit.jupiter.api)
    testImplementation(libs.mockito.inline)
    testImplementation(libs.mockito.inline)

    testFixturesImplementation(edc.ids)
    testFixturesImplementation("com.auth0:java-jwt:${auth0JWTVersion}")

    testRuntimeOnly(libs.junit.jupiter.engine)
}

tasks.test {
    useJUnitPlatform()
}
tasks.jacocoTestReport {
    reports {
        xml.required.set(true)
    }
    dependsOn(tasks.test)
    classDirectories.setFrom(
            files(classDirectories.files.map {
                fileTree(it) {
                    exclude(
                            "**/dto/**",
                            "**/types/clearinghouse/*",
                            "**/types/ids/*",
                            "**/types/Pagging*")
                }
            })
    )
}
