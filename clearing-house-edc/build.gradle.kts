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
}

val javaVersion: String by project
val defaultVersion: String by project
val annotationProcessorVersion: String by project
val metaModelVersion: String by project

var actualVersion: String = (project.findProperty("version") ?: defaultVersion) as String
if (actualVersion == "unspecified") {
    actualVersion = defaultVersion
}

buildscript {
    dependencies {
        val gradlePluginsGroup: String by project
        val gradlePluginsVersion: String by project
        classpath("${gradlePluginsGroup}.edc-build:${gradlePluginsGroup}.edc-build.gradle.plugin:${gradlePluginsVersion}")
    }
}

allprojects {
    val gradlePluginsGroup: String by project
    apply(plugin = "${gradlePluginsGroup}.edc-build")

    // configure which version of the annotation processor to use. defaults to the same version as the plugin
    configure<org.eclipse.edc.plugins.autodoc.AutodocExtension> {
        processorVersion.set(annotationProcessorVersion)
        outputDirectory.set(project.buildDir)
    }

    configure<org.eclipse.edc.plugins.edcbuild.extensions.BuildExtension> {
        versions {
            // override default dependency versions here
            projectVersion.set(actualVersion)
            metaModel.set(metaModelVersion)
        }
        javaLanguageVersion.set(JavaLanguageVersion.of(javaVersion))
    }

    configure<CheckstyleExtension> {
        configFile = rootProject.file("resources/edc-checkstyle-config.xml")
        configDirectory.set(rootProject.file("resources"))
    }

    repositories {
        val gitHubUserName: String? by project
        val gitHubUserPassword: String? by project
        maven {
            url = uri("https://maven.pkg.github.com/ids-basecamp/edc-fork")
            credentials {
                username = gitHubUserName
                password = gitHubUserPassword
            }
        }
        maven {
            url = uri("https://maven.pkg.github.com/ids-basecamp/gradle-plugins-fork")
            credentials {
                username = gitHubUserName
                password = gitHubUserPassword
            }
        }
    }

}