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
 *       Microsoft Corporation - Initial implementation
 *       truzzt GmbH - EDC extension implementation
 *
 */

plugins {
    `java-library`
    id("application")
    id("com.github.johnrengelman.shadow") version "7.1.2"
}

configurations.all {
    exclude(group = "de.fraunhofer.iais.eis.ids.infomodel", module = "java")
}

dependencies {
    runtimeOnly(project(":extensions:multipart"))

    runtimeOnly(edc.bundles.connector)
    runtimeOnly(edc.oauth2.core)
    runtimeOnly(edc.vault.filesystem)

    runtimeOnly(":infomodel-java-4.1.3")
    runtimeOnly(":infomodel-util-4.0.4")
}

application {
    mainClass.set("org.eclipse.edc.boot.system.runtime.BaseRuntime")
}

tasks.withType<com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar> {
    mergeServiceFiles()
    archiveFileName.set("clearing-house-edc.jar")
}
