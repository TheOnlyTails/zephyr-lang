package com.theonlytails.zephyr_lang.config

import com.akuleshov7.ktoml.Toml
import com.akuleshov7.ktoml.TomlInputConfig
import com.akuleshov7.ktoml.TomlOutputConfig
import com.akuleshov7.ktoml.exceptions.TomlDecodingException
import com.theonlytails.zephyr_lang.URLSerializer
import com.theonlytails.zephyr_lang.err
import com.theonlytails.zephyr_lang.ok
import io.github.z4kn4fein.semver.Version
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import java.net.URL

/**
 * Represents the Zephyr configuration.
 *
 * This class is used to store the configuration details of a Zephyr project.
 *
 * @property name The name of the project.
 * @property version The current version of the project.
 * @property zephyr The version of Zephyr used in the project.
 *                  Will throw an error if the major version specified does not match the one running the project.
 * @property private When set to true, using this project as a dependency is prohibited.
 * @property description The description of the project (optional).
 * @property homepage The homepage URL of the project (optional).
 * @property author The author of the project (optional).
 * @property repository The details of the project's repository (optional).
 * @property license The license of the project (optional).
 * @property dependencies The dependencies of the project (optional).
 */
@Serializable
data class ZephyrConfig(
    val name: String,
    val version: Version,
    val zephyr: Version,
    val private: Boolean = false,
    val description: String?,
    @Serializable(with = URLSerializer::class)
    val homepage: URL?,
    val author: String?,
    val repository: Repository?,
    val license: String?,
    val dependencies: Map<String, Repository>?,
) {
    /**
     * Represents a repository with the necessary information to identify it.
     *
     * @property host The host of the repository (GitHub, Gitlab, Bitbucket).
     * @property username The username associated with the repository.
     * @property name The name of the repository.
     */
    @Serializable
    data class Repository(val host: Host, val username: String, val name: String) {
        @Serializable
        enum class Host {
            @SerialName("github")
            Github,

            @SerialName("gitlab")
            Gitlab,

            @SerialName("bitbucket")
            Bitbucket,
        }
    }

    companion object {
        fun fromString(configFile: String) = try {
            Toml.decodeFromString<ZephyrConfig>(configFile).ok()
        } catch (e: TomlDecodingException) {
            e.err()
        }
    }
}


