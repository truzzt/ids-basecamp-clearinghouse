package de.fhg.aisec.ids.clearinghouse

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.databind.ObjectMapper
import de.fraunhofer.iais.eis.Message

class ClearingHouseMessage {
    var header: Message? = null
    var payload: String? = null
    var payloadType: String? = null

    fun toJson(): String {
        val objectMapper = ObjectMapper()
        objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL)
        return objectMapper.writeValueAsString(this)
    }
}