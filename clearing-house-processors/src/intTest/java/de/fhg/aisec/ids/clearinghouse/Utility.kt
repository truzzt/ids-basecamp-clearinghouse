package de.fhg.aisec.ids.clearinghouse

import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest
import de.fhg.aisec.ids.idscp2.daps.aisecdaps.AisecDapsDriver
import de.fhg.aisec.ids.idscp2.daps.aisecdaps.AisecDapsDriverConfig
import de.fhg.aisec.ids.idscp2.keystores.KeyStoreUtil.loadKeyStore
import de.fraunhofer.iais.eis.DynamicAttributeToken
import de.fraunhofer.iais.eis.DynamicAttributeTokenBuilder
import de.fraunhofer.iais.eis.LogMessageBuilder
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.QueryMessageBuilder
import de.fraunhofer.iais.eis.RequestMessageBuilder
import de.fraunhofer.iais.eis.TokenFormat
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.int
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import okhttp3.Headers
import okhttp3.MultipartReader
import java.net.URI
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.nio.file.Path
import java.nio.file.Paths
import java.time.LocalDateTime
import java.util.Base64
import java.util.Objects
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

        val CONNECTOR_1 = "D2:70:FE:7F:32:BB:37:BF:DF:F4:08:36:6B:F1:9E:7A:EB:A4:2D:2A:keyid:CB:8C:C7:B6:85:79:A8:23:A6:CB:15:AB:17:50:2F:E6:65:43:5D:E8"
        val CONNECTOR_2 = "13:09:2E:1C:50:9B:8B:77:DE:01:1F:3B:B5:E0:D2:CC:1B:C5:88:9E:keyid:CB:8C:C7:B6:85:79:A8:23:A6:CB:15:AB:17:50:2F:E6:65:43:5D:E8"

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
                    .getResource("ssl/client-keystore.p12")
            ).path
        )

        val keyStorePathOtherClient: Path = Paths.get(
            Objects.requireNonNull(
                MultipartEndpointTest::class.java.classLoader
                    .getResource("ssl/server-keystore.p12")
            ).path
        )

        val trustStorePath: Path = Paths.get(
            Objects.requireNonNull(
                MultipartEndpointTest::class.java.classLoader
                    .getResource("ssl/truststore.p12")
            ).path
        )

        val password = "password".toCharArray()

        // Load certificates from local KeyStore
        val ks = loadKeyStore(keyStorePath, password)
        val ksOtherClient = loadKeyStore(keyStorePathOtherClient, password)

        val dapsDriver = AisecDapsDriver(
            AisecDapsDriverConfig.Builder()
                .setKeyStorePath(keyStorePath)
                .setKeyStorePassword(password)
                .setKeyPassword(password)
                .setKeyAlias("1")
                .setTrustStorePath(trustStorePath)
                .setTrustStorePassword(password)
                .setDapsUrl("https://daps-dev.aisec.fraunhofer.de/v4")
                .loadTransportCertsFromKeystore(ks)
                .build()
        )

        val dapsDriverOtherClient = AisecDapsDriver(
            AisecDapsDriverConfig.Builder()
                .setKeyStorePath(keyStorePathOtherClient)
                .setKeyStorePassword(password)
                .setKeyPassword(password)
                .setKeyAlias("1")
                .setTrustStorePath(trustStorePath)
                .setTrustStorePassword(password)
                .setDapsUrl("https://daps-dev.aisec.fraunhofer.de/v4")
                .loadTransportCertsFromKeystore(ksOtherClient)
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
