package de.fhg.aisec.ids.clearinghouse.idscp2

import de.fhg.aisec.ids.clearinghouse.MessageType
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.idscp2.default_drivers.remote_attestation.dummy.RaProverDummy2
import de.fhg.aisec.ids.idscp2.default_drivers.remote_attestation.dummy.RaVerifierDummy2
import de.fhg.aisec.ids.idscp2.default_drivers.secure_channel.tlsv1_3.NativeTlsConfiguration
import de.fhg.aisec.ids.idscp2.idscp_core.api.configuration.AttestationConfig
import de.fhg.aisec.ids.idscp2.idscp_core.api.configuration.Idscp2Configuration
import de.fraunhofer.iais.eis.DynamicAttributeTokenBuilder
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.TokenFormat

class Idscp2EndpointTest {

    companion object {

        private val localAttestationConfig = AttestationConfig.Builder()
            .setSupportedRaSuite(arrayOf(RaProverDummy2.RA_PROVER_DUMMY2_ID))
            .setExpectedRaSuite(arrayOf(RaVerifierDummy2.RA_VERIFIER_DUMMY2_ID))
            .setRaTimeoutDelay(300 * 1000L) // 300 seconds
            .build()

        // create idscp2 config
        private val settings = Idscp2Configuration.Builder()
            .setAckTimeoutDelay(500) //  500 ms
            .setHandshakeTimeoutDelay(5 * 1000L) // 5 seconds
            .setAttestationConfig(localAttestationConfig)
            .setDapsDriver(Utility.dapsDriver)
            .build()

        // create secureChannel config
        private val nativeTlsConfiguration = NativeTlsConfiguration.Builder()
            .setKeyStorePath(Utility.keyStorePath)
            .setTrustStorePath(Utility.trustStorePath)
            .setCertificateAlias("1.0.1")
            .setHost("provider-core")
            .build()

        val client = Idscp2Client(settings, nativeTlsConfiguration)

        fun getMessage(type: MessageType, client: Int = 1): Message{
            return when (client){
                2 -> Utility.getMessage(type,
                    Utility.getDapsToken(Utility.dapsDriverOtherClient.token)
                )
                else -> Utility.getMessage(type, Utility.getDapsToken())
            }
        }

        fun getInvalidMessage(type: MessageType): Message{
            val invToken = DynamicAttributeTokenBuilder()
                ._tokenFormat_(TokenFormat.JWT)
                ._tokenValue_("This is not a valid token!")
                .build()
            return Utility.getMessage(type, invToken)
        }

        fun logMessage(pid: String, payload: String, authenticated: Boolean = true, client: Int = 1): Triple<Message?, ByteArray?, Map<String, String>?> {
            val m = if (authenticated){
                getMessage(MessageType.LOG, client)
            } else{
                getInvalidMessage(MessageType.LOG)
            }
            val header = mapOf("ch-ids-pid" to pid)
            val p = payload.toByteArray()
            return Idscp2EndpointTest.client.send(m, header, p)
        }

        fun pidMessage(pid: String, payload: String, authenticated: Boolean = true, client: Int = 1): Triple<Message?, ByteArray?, Map<String, String>?> {
            val m = if (authenticated){
                getMessage(MessageType.PID, client)
            } else{
                getInvalidMessage(MessageType.PID)
            }
            val header = mapOf("ch-ids-pid" to pid, "Content-Type" to "application/json" )
            val p = payload.toByteArray()
            return Idscp2EndpointTest.client.send(m, header, p)
        }

        fun queryMessage(pid: String, id: String?, payload: String, authenticated: Boolean = true, client: Int = 1, page: Int = 1, size: Int = 100, sort: String = "desc"): Triple<Message?, ByteArray?, Map<String, String>?> {
            val m = if (authenticated){
                getMessage(MessageType.QUERY, client)
            } else{
                getInvalidMessage(MessageType.QUERY)
            }
            val header = if (id != null){
                mapOf("ch-ids-pid" to pid, "ch-ids-id" to id, "Content-Type" to "application/json" )
            }
            else{
                mapOf("ch-ids-pid" to pid, "ch-ids-page" to page.toString(), "ch-ids-size" to size.toString(), "ch-ids-sort" to sort, "Content-Type" to "application/json" )
            }
            val p = payload.toByteArray()
            return Idscp2EndpointTest.client.send(m, header, p)
        }
    }
}
