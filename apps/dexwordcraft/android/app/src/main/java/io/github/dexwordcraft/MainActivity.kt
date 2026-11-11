package io.github.dexwordcraft

import io.github.craftandroid.CraftActivity

/**
 * DexWordCraft's activity. Everything Android-side (documents through the Storage Access
 * Framework, clipboard, printing, mouse and keyboard, the on-screen keyboard, lifecycle,
 * diagnostics) is craft-android's [CraftActivity]; the library it loads (`libdexwordcraft.so`,
 * the manifest's `android.app.lib_name`) runs WordCraft's interface.
 */
class MainActivity : CraftActivity() {
    /** Documents dragged onto the window open as WordCraft's File › Open opens them. */
    override fun droppableMimeTypes(): List<String> = DOCUMENT_TYPES

    companion object {
        /**
         * What WordCraft opens (its File › Open filter): Word (docx, docm, dotx, dotm, doc, dot),
         * OpenDocument (odt, ott, fodt), RTF, plain text, Markdown, HTML and MHTML, LaTeX.
         */
        val DOCUMENT_TYPES = listOf(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "application/vnd.ms-word.document.macroEnabled.12",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
            "application/vnd.ms-word.template.macroEnabled.12",
            "application/msword",
            "application/vnd.oasis.opendocument.text",
            "application/vnd.oasis.opendocument.text-template",
            "application/vnd.oasis.opendocument.text-flat-xml",
            "application/rtf",
            "text/rtf",
            "text/plain",
            "text/markdown",
            "text/html",
            "message/rfc822",
            "application/x-tex",
        )
    }
}
