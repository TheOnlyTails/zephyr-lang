plugins {
    kotlin("jvm") version libs.versions.kotlin
    kotlin("plugin.serialization") version libs.versions.kotlin
    antlr
}

group = "com.theonlytails"
version = "0.1.0"


repositories {
    mavenCentral()
}

dependencies {
    implementation(libs.kotlinx.serialization)
    implementation(libs.okio)
    implementation(libs.mordant)
    implementation(libs.fuzzy)
    implementation(libs.ktoml.core)
    implementation(libs.ktoml.file)
    implementation(libs.semver)

    antlr(libs.antlr)
    compileClasspath(libs.antlr.runtime)
    testImplementation(kotlin("test"))
}

kotlin {
    jvmToolchain(19)
}

tasks.compileKotlin {
    dependsOn(":generateGrammarSource")
}

tasks.test {
    useJUnitPlatform()
}
