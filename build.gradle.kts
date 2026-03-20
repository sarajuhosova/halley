import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "2.2.20"
    antlr
    idea
}

group = "com.sarajuhosova.halley"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    antlr(libs.antlr4)

    testImplementation(kotlin("test"))
    testImplementation(libs.assertj)
}

tasks.test {
    useJUnitPlatform()
}
kotlin {
    jvmToolchain(21)
}

idea {
    module {
        sourceDirs.add(file("$projectDir/src/main/antlr"))
    }
}

tasks.generateGrammarSource {
    outputDirectory = file("$projectDir/build/generated/sources/main/java/antlr")

    arguments = arguments +
            "-package" + "com.sarajuhosova.halley" +
            "-visitor"
}

tasks.withType<KotlinCompile>().configureEach {
    dependsOn(tasks.withType<AntlrTask>())
}
tasks.withType<Jar>().configureEach {
    dependsOn(tasks.withType<AntlrTask>())
}

sourceSets {
    main {
        java {
            srcDir(tasks.generateGrammarSource)
        }
    }
    test {
        java {
            srcDir(tasks.generateGrammarSource)
        }
    }
}
