package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.MessageType
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.idscp2.default_drivers.keystores.PreConfiguration
import de.fraunhofer.iais.eis.DynamicAttributeTokenBuilder
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.TokenFormat
import okhttp3.OkHttpClient
import javax.net.ssl.SSLContext
import javax.net.ssl.X509TrustManager

class MultipartEndpointTest {

    companion object {
        private val trustManagers = PreConfiguration.getX509ExtTrustManager(
            Utility.trustStorePath,
            "password".toCharArray()
        )

        private val keyManagers = PreConfiguration.getX509ExtKeyManager(
            "password".toCharArray(),
            Utility.keyStorePath,
            "password".toCharArray(),
            "1.0.1",
            "RSA"
        )

        private val sslContext = SSLContext.getInstance("TLS")

        init{
            sslContext.init(keyManagers, trustManagers, null);
        }

        val client = OkHttpClient.Builder()
        .sslSocketFactory(sslContext.socketFactory, trustManagers[0] as X509TrustManager)
        .build();

        fun getMessage(type: MessageType, client: Int = 1): Message{
            return when (client){
                2 -> Utility.getMessage(type, Utility.getDapsToken(Utility.dapsDriverOtherClient.token))
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
    }
}