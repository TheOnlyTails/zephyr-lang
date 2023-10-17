import org.jetbrains.dokka.base.DokkaBase
import org.jetbrains.dokka.base.DokkaBaseConfiguration
import java.net.URL

buildscript {
    dependencies {
        classpath("org.jetbrains.dokka", "dokka-base", libs.versions.kotlin.get())
    }
}

plugins {
    kotlin("jvm") version libs.versions.kotlin
    kotlin("plugin.serialization") version libs.versions.kotlin
    id("org.jetbrains.dokka") version libs.versions.kotlin
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

tasks.dokkaHtml {
    outputDirectory = projectDir.resolve("docs/public/reference")

    pluginConfiguration<DokkaBase, DokkaBaseConfiguration> {
        templatesDir = file("dokka/templates")
    }

    dokkaSourceSets.configureEach {
        // links to the source of declarations in the GitHub project
        sourceLink {
            localDirectory = projectDir.resolve("src")
            @Suppress("DEPRECATION")
            remoteUrl = URL("https://github.com/theonlytails/zephyr-lang/tree/main/src")
        }

        // add
        externalDocumentationLink {
            @Suppress("DEPRECATION")
            url = URL("https://kotlinlang.org/api/kotlinx.serialization/")
            packageListUrl = rootProject.projectDir.resolve("serialization.package.list").toURI().toURL()
        }
    }
}
