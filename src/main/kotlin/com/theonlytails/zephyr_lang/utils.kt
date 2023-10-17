package com.theonlytails.zephyr_lang

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import okio.FileSystem
import okio.Path
import okio.Path.Companion.toPath
import org.antlr.v4.runtime.misc.Interval
import java.net.URL
import java.nio.file.InvalidPathException

fun Interval.toRange() = a..b

fun <T> T.ok() = Result.success(this)
fun <T : Throwable> T.err() = Result.failure<Nothing>(this)

fun isValidPath(s: String) = try {
    s.toPath().toNioPath()
    true
} catch (e: InvalidPathException) {
    false
}

val fs = FileSystem.SYSTEM

val Path.isDirectory
    get() = fs.metadataOrNull(this)?.isDirectory == true

val Path.isRegularFile
    get() = fs.metadataOrNull(this)?.isRegularFile == true


fun findParentWithFile(startPath: Path, fileName: String): Path? {
    // Go up the directory tree until the root
    var parent: Path? = startPath
    while (parent != null) {
        // Return the parent directory that contains the file
        if ((parent / fileName).isRegularFile) return parent
        parent = parent.parent
    }

    // Return null if file is not found in any parent directory
    return null
}

object URLSerializer : KSerializer<URL> {
    override val descriptor = PrimitiveSerialDescriptor("URL", PrimitiveKind.STRING)

    override fun deserialize(decoder: Decoder) = @Suppress("DEPRECATION") URL(decoder.decodeString())

    override fun serialize(encoder: Encoder, value: URL) = encoder.encodeString(value.toString())
}
