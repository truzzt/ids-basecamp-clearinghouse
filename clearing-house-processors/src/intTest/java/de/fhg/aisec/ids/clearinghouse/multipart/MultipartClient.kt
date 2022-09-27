package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.MessageType
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import okhttp3.Headers
import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.MultipartBody
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody

class MultipartClient {

    companion object{
        private val SERIALIZER = Serializer()
        private var JSON = "application/json; charset=utf-8".toMediaTypeOrNull()!!

        private val BASE_URL = "https://provider-core:9999/"
        private val LOG_URL = "messages/log/"
        private val QUERY_URL = "messages/query/"
        private val PROCESS_URL = "process/"

        private fun makePart(name: String, payload: String, ctJson: Boolean): MultipartBody.Part{
            var headers = Headers.Builder().add("Content-Disposition", "form-data; name=\"$name\"")
            val body = if (ctJson){
               payload.toRequestBody(JSON)
            }
            else{
                payload.toRequestBody()
            }

            return MultipartBody.Part.create(headers.build(), body)
        }

        private fun makeRequest(url: String, m: Message, payload: String, ctJson: Boolean): Request{
            val requestBody = MultipartBody.Builder()
                .setType(MultipartBody.ALTERNATIVE)
                .addPart(makePart("header", SERIALIZER.serialize(m), ctJson))
                .addPart(makePart("payload", payload, ctJson))
                .build()

            return Request.Builder()
                .header("Authorization", "Bearer " + m.securityToken)
                .url(url)
                .post(requestBody)
                .build()
        }

        fun logMessage(pid: String, payload: String, authenticated: Boolean = true, client: Int = 1): Request{
            val m = if (authenticated){
                MultipartEndpointTest.getMessage(MessageType.LOG, client)
            } else{
                MultipartEndpointTest.getInvalidMessage(MessageType.LOG)
            }
            val url = "$BASE_URL$LOG_URL$pid"
            return makeRequest(url, m, payload, false)
        }

        fun queryMessage(pid: String, id: String?, payload: String, authenticated: Boolean = true, client: Int = 1, page: Int = 1, size: Int = 100, sort: String = "desc"): Request{
            val m = if (authenticated){
                MultipartEndpointTest.getMessage(MessageType.QUERY, client)
            } else{
                MultipartEndpointTest.getInvalidMessage(MessageType.QUERY)
            }
            val url = if (id == null) "$BASE_URL$QUERY_URL$pid?page=$page&size=$size&sort=$sort" else "$BASE_URL$QUERY_URL$pid/$id"
            return makeRequest(url, m, payload, false)
        }

        fun pidMessage(pid: String, payload: String, ctJson: Boolean = true, authenticated: Boolean = true, client: Int = 1): Request{
            val m = if (authenticated){
                MultipartEndpointTest.getMessage(MessageType.PID, client)
            } else{
                MultipartEndpointTest.getInvalidMessage(MessageType.PID)
            }
            val url = "$BASE_URL$PROCESS_URL$pid"
            return makeRequest(url, m, payload, ctJson)
        }

    }
}