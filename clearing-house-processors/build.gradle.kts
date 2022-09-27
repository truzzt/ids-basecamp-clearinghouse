import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.io.FileInputStream
import java.util.*

plugins {
    java
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.spring.dependencyManagement)
    `maven-publish`
}

group = "de.fhg.aisec.ids.clearinghouse"

val fis = FileInputStream("../clearing-house-app/logging-service/Cargo.toml")
val props = Properties()
props.load(fis)
version = props.getProperty("version").removeSurrounding("\"")

sourceSets{
    create("intTest"){
    }
}

val intTestImplementation by configurations.getting {
    extendsFrom(configurations.testImplementation.get())
}

configurations["intTestRuntimeOnly"].extendsFrom(configurations.runtimeOnly.get())

val integrationTest = task<Test>("integrationTest") {
    // set to true for debugging
    testLogging.showStandardStreams = false
    useJUnitPlatform()

    description = "Runs integration tests."
    group = "verification"

    testClassesDirs = sourceSets["intTest"].output.classesDirs
    classpath = sourceSets["intTest"].runtimeClasspath
    shouldRunAfter("test")
}

tasks.register("printChVersion") {

    doFirst {
        println(version)
    }
}

buildscript {
    repositories {
        mavenCentral()

        fun findProperty(s: String) = project.findProperty(s) as String?

        maven {
            name = "GitHubPackages"

            url = uri("https://maven.pkg.github.com/Fraunhofer-AISEC/ids-clearing-house-service")
            credentials(HttpHeaderCredentials::class) {
                name = findProperty("github.username")
                value = findProperty("github.token")
            }
            authentication {
                create<HttpHeaderAuthentication>("header")
            }
        }   
    }
}

publishing {
    fun findProperty(s: String) = project.findProperty(s) as String?

    publications {
        create<MavenPublication>("binary") {
            artifact(tasks["jar"])
        }
    }
    repositories {
        maven {            
            name = "GitHubPackages"

            url = uri("https://maven.pkg.github.com/Fraunhofer-AISEC/ids-clearing-house-service")
            credentials(HttpHeaderCredentials::class) {
                name = findProperty("github.username")
                value = findProperty("github.token")
            }
            authentication {
                create<HttpHeaderAuthentication>("header")
            }
        }
    }
}

repositories {
    mavenCentral()
    // References IAIS repository that contains the infomodel artifacts
    maven("https://maven.iais.fraunhofer.de/artifactory/eis-ids-public/")
}

dependencies {
    // Imported from IDS feature in TC at runtime
    implementation(libs.infomodel.model)
    implementation(libs.infomodel.serializer)

    implementation(libs.camel.idscp2)
    implementation(libs.camel.core)
    implementation(libs.camel.api)
    implementation(libs.camel.jetty)

    implementation(libs.apacheHttp.core)
    implementation(libs.apacheHttp.client)
    implementation(libs.apacheHttp.mime)
    implementation(libs.commons.fileupload)

    testApi(libs.slf4j.simple)
    testImplementation(libs.junit5)
    testImplementation(libs.okhttp3)
    testImplementation(kotlin("test"))
    testImplementation(libs.kotlin.serialization.json)
}

tasks.withType<KotlinCompile> {
    kotlinOptions {
        jvmTarget = "11"
    }
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
    sourceCompatibility = "11"
    targetCompatibility = "11"
}

tasks.jar {
    manifest {
        attributes(mapOf(Pair("Bundle-Vendor", "Fraunhofer AISEC"),
                         Pair("-noee", true)))
    }
}