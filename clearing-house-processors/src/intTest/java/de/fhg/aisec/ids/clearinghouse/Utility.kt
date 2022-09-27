package de.fhg.aisec.ids.clearinghouse

import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriver
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityProfile
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements
import de.fraunhofer.iais.eis.*
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.*
import okhttp3.Headers
import okhttp3.MultipartReader
import java.net.URI
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.nio.file.Path
import java.nio.file.Paths
import java.time.LocalDateTime
import java.util.*
import javax.xml.datatype.DatatypeFactory

@Serializable
data class ChJwt(val transaction_id: String,
                         val timestamp: Int,
                         val process_id: String,
                         val document_id: String,
                         val payload: String,
                         val chain_hash: String,
                         val client_id: String,
                         val clearing_house_version: String)

@Serializable
private data class ChReceipt(val data: String)

@Serializable
data class QueryResult(val date_from: String,
                       val date_to: String,
                       val page: Int,
                       val size: Int,
                       val order: String,
                       val documents: List<String>)

@Serializable
data class OwnerList(val owners: List<String>)


enum class MessageType{
    LOG, PID, QUERY
}

class Utility {
    companion object{
        val CONNECTOR_1 = "A5:0C:A5:F0:84:D9:90:BB:BC:D9:57:3A:04:C8:7F:93:ED:97:A2:52:keyid:CB:8C:C7:B6:85:79:A8:23:A6:CB:15:AB:17:50:2F:E6:65:43:5D:E8"
        val CONNECTOR_2 = "7A:2B:DD:2A:14:22:A3:50:3D:EA:FB:60:72:6A:FB:2E:58:41:CB:C0:keyid:CB:8C:C7:B6:85:79:A8:23:A6:CB:15:AB:17:50:2F:E6:65:43:5D:E8"

        val STATUS_400 = "Bad Request"
        val STATUS_401 = "Unauthorized"
        val STATUS_403 = "Forbidden"
        val STATUS_404 = "Not Found"
        val STATUS_500 = "Internal Server Error"

        private val TEST_RUN_ID = (0..2147483647).random()

        private val SERIALIZER = Serializer()



        val keyStorePath: Path = Paths.get(
            Objects.requireNonNull(
                MultipartEndpointTest::class.java.classLoader
                    .getResource("ssl/consumer-keystore.p12")
            ).path
        )

        val keyStorePathOtherClient: Path = Paths.get(
            Objects.requireNonNull(
                MultipartEndpointTest::class.java.classLoader
                    .getResource("ssl/provider-keystore.p12")
            ).path
        )

        val trustStorePath: Path = Paths.get(
            Objects.requireNonNull(
                MultipartEndpointTest::class.java.classLoader
                    .getResource("ssl/truststore.p12")
            ).path
        )

        val securityRequirements = SecurityRequirements.Builder()
            .setRequiredSecurityLevel(SecurityProfile.INVALID)
            .build()

        val dapsDriver = AisecDapsDriver(
            AisecDapsDriverConfig.Builder()
                .setKeyStorePath(keyStorePath)
                .setTrustStorePath(trustStorePath)
                .setDapsUrl("https://daps.aisec.fraunhofer.de")
                .setSecurityRequirements(securityRequirements)
                .build()
        )

        val dapsDriverOtherClient = AisecDapsDriver(
            AisecDapsDriverConfig.Builder()
                .setKeyStorePath(keyStorePathOtherClient)
                .setTrustStorePath(trustStorePath)
                .setDapsUrl("https://daps.aisec.fraunhofer.de")
                .setSecurityRequirements(securityRequirements)
                .build()
        )

        fun formatId(id: String): String{
            return "${id}_${TEST_RUN_ID}"
        }

        fun getDapsToken(token: ByteArray = dapsDriver.token): DynamicAttributeToken{
            return DynamicAttributeTokenBuilder()
                ._tokenFormat_(TokenFormat.JWT)
                ._tokenValue_(String(token, StandardCharsets.UTF_8))
                .build()
        }

        fun <T: Message> checkIdsMessage(m: String, c: Class<T>){
            SERIALIZER.deserialize(m, c)
        }

        private fun getPart(headers: Headers): String{
            val partName = headers["Content-Disposition"]!!.split(";")[1].split("=")[1]
            return partName.substring(1, partName.length-1)
        }

        fun getParts(reader: MultipartReader): Pair<String, String>{
            var header = ""
            var payload = ""
            reader.use {
                while (true) {
                    val part = reader.nextPart() ?: break
                    when (getPart(part.headers)){
                        "header" -> {
                            header = part.body.readString(Charset.forName("utf-8"))
                        }
                        "payload" -> {
                            payload = part.body.readString(Charset.forName("utf-8"))
                        }
                    }
                }
            }
            return Pair(header, payload)
        }

        fun parseJwt(receipt: String): ChJwt{
            val data = Json.decodeFromString<ChReceipt>(receipt)
            val chunks: List<String> = data.data.split(".")
            val decoder: Base64.Decoder = Base64.getUrlDecoder()
            val payload = String(decoder.decode(chunks[1]))
            return Json.decodeFromString(payload)
        }

        fun parseQueryResult(body: String): QueryResult{
            val json = Json.parseToJsonElement(body).jsonObject
            return QueryResult(
                json["date_from"]!!.jsonPrimitive.content,
                json["date_to"]!!.jsonPrimitive.content,
                json["page"]!!.jsonPrimitive.int,
                json["size"]!!.jsonPrimitive.int,
                json["order"]!!.jsonPrimitive.content,
                json["documents"]!!.jsonArray.map { it.toString() }
            )
        }

        fun getMessage(type: MessageType, token: DynamicAttributeToken): Message{
            when (type) {
                MessageType.LOG -> return LogMessageBuilder()
                    ._securityToken_(token)
                    ._issuerConnector_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                    ._senderAgent_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._modelVersion_("4.0")
                    .build()
                MessageType.QUERY -> return QueryMessageBuilder()
                    ._securityToken_(token)
                    ._issuerConnector_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                    ._senderAgent_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._modelVersion_("4.0")
                    .build()
                MessageType.PID -> return RequestMessageBuilder()
                    ._securityToken_(token)
                    ._issuerConnector_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                    ._senderAgent_(URI.create("http://ch-ids.aisec.fraunhofer.de/idscp-client"))
                    ._modelVersion_("4.0")
                    .build()
            }
        }
    }
}