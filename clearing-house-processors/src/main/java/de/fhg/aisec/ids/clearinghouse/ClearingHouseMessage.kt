package de.fhg.aisec.ids.clearinghouse

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.databind.ObjectMapper
import de.fraunhofer.iais.eis.Message
import org.slf4j.LoggerFactory
import java.nio.charset.Charset
import javax.xml.bind.DatatypeConverter

class ClearingHouseMessage (var header: Message? = null, var payloadType: String? =  null, var payload: String? = null){
    private var charset: String = Charset.defaultCharset().toString()

    fun toJson(): String {
        val objectMapper = ObjectMapper()
        objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL)
        return objectMapper.writeValueAsString(this)
    }

    constructor(idsHeader: Message, contentTypeHeader: String?, payload: ByteArray) : this() {
        this.header = idsHeader
        parseContentType(contentTypeHeader)
        when (this.payloadType){
            "text/plain", "application/json", "application/ld+json" -> {
                this.payload = String(payload, Charset.forName(charset))
            }
            else -> {
                this.payloadType = "application/octet-stream"
                this.payload = DatatypeConverter.printBase64Binary(payload)
            }
        }
    }

    private fun parseContentType(contentTypeHeader: String?) {
        // Parsing Content-Type and Charset
        if (contentTypeHeader != null) {
            val parts = contentTypeHeader.split(";")
            when (parts.size){
                1 -> {
                    this.payloadType = parts[0]
                }
                2 -> {
                    this.payloadType = parts[0]
                    val charsetInput = parts[1].split("=")
                    if (charsetInput.size == 2){
                        this.charset = charsetInput[1]
                        LOG.debug("Using Charset from Content-Type header: {}", charset)
                    }
                }
                else -> {
                    this.payloadType = "text/plain"
                }
            }
        }
        else{
            this.payloadType = "application/octet-stream"
        }
    }

    companion object {
        private val LOG = LoggerFactory.getLogger(ClearingHouseMessage::class.java)
    }
}
