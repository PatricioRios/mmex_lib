plugins {
    kotlin("jvm") version "1.9.22"
    `maven-publish`
}

group = "com.github.patriciorios"
version = "0.1.0-beta.2"

repositories {
    mavenCentral()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0")
    testImplementation(kotlin("test"))
}

sourceSets {
    main {
        kotlin.srcDirs("src/main/kotlin", "generated")
    }
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = "17"
}

java {
    withSourcesJar()
    withJavadocJar()
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            pom {
                name.set("MMEX Library")
                description.set("Kotlin bindings for Money Manager EX database library (via UniFFI)")
                url.set("https://github.com/PatricioRios/mmex_lib")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                developers {
                    developer {
                        id.set("PatricioRios")
                        name.set("Patricio Rios")
                    }
                }
                scm {
                    connection.set("scm:git:git://github.com/PatricioRios/mmex_lib.git")
                    developerConnection.set("scm:git:ssh://github.com/PatricioRios/mmex_lib.git")
                    url.set("https://github.com/PatricioRios/mmex_lib")
                }
            }
        }
    }
    repositories {
        maven {
            name = "localStaging"
            url = uri("${layout.buildDirectory.get()}/staging-deploy")
        }
    }
}
