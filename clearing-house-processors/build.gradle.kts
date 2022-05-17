import org.yaml.snakeyaml.Yaml
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.io.FileInputStream
import java.util.*

plugins {
    kotlin("jvm") version "1.6.20" apply true
    kotlin("plugin.serialization") version "1.6.20"
    `java-library`
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

    dependencies {
        classpath("org.yaml:snakeyaml:1.26")
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
    @Suppress("UNCHECKED_CAST") val libraryVersions =
            Yaml().load(File("${rootDir}/libraryVersions.yaml").inputStream()) as Map<String, String>

    // Imported from IDS feature in TC at runtime
    implementation("de.fraunhofer.iais.eis.ids.infomodel", "java", libraryVersions["infomodel"])
    implementation("de.fraunhofer.iais.eis.ids", "infomodel-serializer", libraryVersions["infomodel"])

    implementation("de.fhg.aisec.ids", "camel-idscp2", libraryVersions["idscp2"])

    implementation("org.apache.camel", "camel-core", libraryVersions["camel"])
    implementation("org.apache.camel", "camel-api", libraryVersions["camel"])
    implementation("org.apache.camel", "camel-jetty", libraryVersions["camel"])

    implementation("org.apache.httpcomponents", "httpcore", libraryVersions["httpcore"])
    implementation("org.apache.httpcomponents", "httpclient", libraryVersions["httpclient"])
    implementation("org.apache.httpcomponents", "httpmime", libraryVersions["httpclient"])
    implementation("commons-fileupload", "commons-fileupload", libraryVersions["commonsFileUpload"])

    testApi("org.slf4j", "slf4j-simple", libraryVersions["slf4j"])
    testImplementation("org.junit.jupiter", "junit-jupiter", libraryVersions["junit5"])
    testImplementation("com.squareup.okhttp3", "okhttp", libraryVersions["okhttp"])
    testImplementation(kotlin("test"))
    testImplementation("org.jetbrains.kotlinx", "kotlinx-serialization-json", "1.3.2")
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