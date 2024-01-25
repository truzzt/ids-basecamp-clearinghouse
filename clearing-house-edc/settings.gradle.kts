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
            val user = providers.gradleProperty("gitHubUserName")
            val token = providers.gradleProperty("gitHubUserPassword")
            url = uri("https://maven.pkg.github.com/ids-basecamp/gradle-plugins-fork")
            credentials {
                username = user.orNull
                password = token.orNull
            }
        }
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

dependencyResolutionManagement {
    repositories {
        val user = providers.gradleProperty("gitHubUserName")
        val token = providers.gradleProperty("gitHubUserPassword")
        maven {
            url = uri("https://maven.pkg.github.com/ids-basecamp/gradle-plugins-fork")
            credentials {
                username = user.orNull
                password = token.orNull
            }
        }
        maven {
            url = uri("https://maven.pkg.github.com/ids-basecamp/edc-fork")
            credentials {
                username = user.orNull
                password = token.orNull
            }
        }
        mavenCentral()
        mavenLocal()
    }
    versionCatalogs {
        val gradlePluginsGroup = providers.gradleProperty("gradlePluginsGroup")
        val gradlePluginsVersion = providers.gradleProperty("gradlePluginsVersion")
        create("libs") {
            from(gradlePluginsGroup.get() + ":edc-versions:" + gradlePluginsVersion.get())
        }

        val edcGroup = providers.gradleProperty("edcGroup")
        val edcVersion = providers.gradleProperty("edcVersion")
        create("edc") {
            version("edc", edcVersion.get())
            library("spi-catalog", edcGroup.get(), "catalog-spi").versionRef("edc")
            library("spi-core", edcGroup.get(), "core-spi").versionRef("edc")
            library("spi-web", edcGroup.get(), "web-spi").versionRef("edc")
            library("util", edcGroup.get(), "util").versionRef("edc")
            library("boot", edcGroup.get(), "boot").versionRef("edc")
            library("config-filesystem", edcGroup.get(), "configuration-filesystem").versionRef("edc")
            library("core-controlplane", edcGroup.get(), "control-plane-core").versionRef("edc")
            library("core-connector", edcGroup.get(), "connector-core").versionRef("edc")
            library("core-jetty", edcGroup.get(), "jetty-core").versionRef("edc")
            library("core-jersey", edcGroup.get(), "jersey-core").versionRef("edc")
            library("junit", edcGroup.get(), "junit").versionRef("edc")
            library("api-management-config", edcGroup.get(), "management-api-configuration").versionRef("edc")
            library("api-management", edcGroup.get(), "management-api").versionRef("edc")
            library("api-observability", edcGroup.get(), "api-observability").versionRef("edc")
            library("ext-http", edcGroup.get(), "http").versionRef("edc")
            library("spi-ids", edcGroup.get(), "ids-spi").versionRef("edc")
            library("ids", edcGroup.get(), "ids").versionRef("edc")
            library("ids-jsonld-serdes", edcGroup.get(), "ids-jsonld-serdes").versionRef("edc")
            library("oauth2-core", edcGroup.get(), "oauth2-core").versionRef("edc")
            library("vault-filesystem", edcGroup.get(), "vault-filesystem").versionRef("edc")

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
