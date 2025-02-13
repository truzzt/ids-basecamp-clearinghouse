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

configurations.all {
    exclude(group = "de.fraunhofer.iais.eis.ids.infomodel", module = "java")
}

dependencies {
    api(edc.spi.core)

    implementation(project(":core"))

    implementation(edc.ids)
    implementation(edc.ids.jsonld.serdes)
    implementation(edc.api.management.config)
    implementation(libs.jakarta.rsApi)
    implementation(libs.jersey.multipart)

    implementation(":infomodel-java-4.1.3")
    implementation(":infomodel-util-4.0.4")

    testImplementation(libs.junit.jupiter.api)
    testImplementation(libs.mockito.inline)
    testImplementation(libs.mockito.inline)

    testImplementation(testFixtures(project(":core")))

    testRuntimeOnly(libs.junit.jupiter.engine)
}

tasks.test {
    useJUnitPlatform()
}
tasks.jacocoTestReport {
    dependsOn(tasks.test)
    reports {
        xml.required = true
    }
}
