import org.yaml.snakeyaml.Yaml

buildscript {
    repositories {
        mavenCentral()
    }
    dependencies {
        classpath("org.yaml:snakeyaml:1.26")
    }
}

plugins {
    java
    id("biz.aQute.bnd") version "4.2.0" apply false
}

group = "de.fhg.aisec.ids.clearinghouse"
version = "1.1-SNAPSHOT"

repositories {
    mavenCentral()
    // References IAIS repository that contains the infomodel artifacts
    maven("https://maven.iais.fraunhofer.de/artifactory/eis-ids-public/")
}

apply(plugin = "biz.aQute.bnd.builder")

dependencies {
    @Suppress("UNCHECKED_CAST") val libraryVersions =
            Yaml().load(File("${rootDir}/libraryVersions.yaml").inputStream()) as Map<String, String>

    // Imported from IDS feature in TC at runtime
    implementation("de.fraunhofer.iais.eis.ids.infomodel", "java", libraryVersions["infomodel"])
    implementation("de.fraunhofer.iais.eis.ids", "infomodel-serializer", libraryVersions["infomodel"])

    implementation("de.fhg.aisec.ids", "ids-api", libraryVersions["api"])
    implementation("de.fhg.aisec.ids", "camel-idscp2", libraryVersions["idscp2"])

    implementation("org.apache.camel", "camel-core", libraryVersions["camel"])
    implementation("org.apache.camel", "camel-jetty", libraryVersions["camel"])
    implementation("org.apache.camel", "camel-http4", libraryVersions["camelHttp4"])

    implementation("org.apache.httpcomponents", "httpcore", libraryVersions["httpcore"])
    implementation("org.apache.httpcomponents", "httpclient", libraryVersions["httpclient"])
    implementation("org.apache.httpcomponents", "httpmime", libraryVersions["httpclient"])
    implementation("commons-fileupload", "commons-fileupload", libraryVersions["commonsFileUpload"])

    testImplementation("junit", "junit", libraryVersions["junit4"])
}

configure<JavaPluginConvention> {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
}

tasks.withType<Jar> {
    manifest {
        attributes(Pair("Bundle-Vendor", "Fraunhofer AISEC"))
        attributes(Pair("-noee", true))
    }
    destinationDirectory.set(file("../build/trusted-connector"))
}
