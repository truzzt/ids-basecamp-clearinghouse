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

pluginManagement {
    repositories {
        maven {
            url = uri("https://oss.sonatype.org/content/repositories/snapshots/")
        }
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://oss.sonatype.org/content/repositories/snapshots/")
        }
        maven {
            url = uri("https://maven.iais.fraunhofer.de/artifactory/eis-ids-public/")
        }
        mavenCentral()
        mavenLocal()
    }
    versionCatalogs {
        create("libs") {
            from("org.eclipse.edc:edc-versions:0.0.1-milestone-8")
        }
        create("edc") {
            version("edc", "0.0.1-milestone-8")
            library("spi-catalog", "org.eclipse.edc", "catalog-spi").versionRef("edc")
            library("spi-core", "org.eclipse.edc", "core-spi").versionRef("edc")
            library("spi-web", "org.eclipse.edc", "web-spi").versionRef("edc")
            library("util", "org.eclipse.edc", "util").versionRef("edc")
            library("boot", "org.eclipse.edc", "boot").versionRef("edc")
            library("config-filesystem", "org.eclipse.edc", "configuration-filesystem").versionRef("edc")
            library("core-controlplane", "org.eclipse.edc", "control-plane-core").versionRef("edc")
            library("core-connector", "org.eclipse.edc", "connector-core").versionRef("edc")
            library("core-jetty", "org.eclipse.edc", "jetty-core").versionRef("edc")
            library("core-jersey", "org.eclipse.edc", "jersey-core").versionRef("edc")
            library("junit", "org.eclipse.edc", "junit").versionRef("edc")
            library("api-management-config", "org.eclipse.edc", "management-api-configuration").versionRef("edc")
            library("api-management", "org.eclipse.edc", "management-api").versionRef("edc")
            library("api-observability", "org.eclipse.edc", "api-observability").versionRef("edc")
            library("ext-http", "org.eclipse.edc", "http").versionRef("edc")
            library("spi-ids", "org.eclipse.edc", "ids-spi").versionRef("edc")
            library("ids", "org.eclipse.edc", "ids").versionRef("edc")
            library("ids-jsonld-serdes", "org.eclipse.edc", "ids-jsonld-serdes").versionRef("edc")
            library("oauth2-core", "org.eclipse.edc", "oauth2-core").versionRef("edc")
            library("vault-filesystem", "org.eclipse.edc", "vault-filesystem").versionRef("edc")

            bundle(
                "connector",
                listOf("boot", "core-connector", "core-jersey", "core-controlplane", "api-observability")
            )
        }
    }
}

include(":core")
include(":extensions:multipart")
include(":launchers:connector-local")
include(":launchers:connector-prod")
