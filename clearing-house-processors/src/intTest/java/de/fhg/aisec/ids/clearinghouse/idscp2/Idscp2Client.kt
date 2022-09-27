/*-
 * ========================LICENSE_START=================================
 * idscp2-examples
 * %%
 * Copyright (C) 2021 Fraunhofer AISEC
 * %%
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * =========================LICENSE_END==================================
 */
package de.fhg.aisec.ids.clearinghouse.idscp2

import de.fhg.aisec.ids.idscp2.app_layer.AppLayerConnection
import de.fhg.aisec.ids.idscp2.default_drivers.remote_attestation.dummy.RaProverDummy2
import de.fhg.aisec.ids.idscp2.default_drivers.remote_attestation.dummy.RaVerifierDummy2
import de.fhg.aisec.ids.idscp2.default_drivers.secure_channel.tlsv1_3.NativeTLSDriver
import de.fhg.aisec.ids.idscp2.default_drivers.secure_channel.tlsv1_3.NativeTlsConfiguration
import de.fhg.aisec.ids.idscp2.idscp_core.FastLatch
import de.fhg.aisec.ids.idscp2.idscp_core.api.configuration.Idscp2Configuration
import de.fhg.aisec.ids.idscp2.idscp_core.api.idscp_connection.Idscp2ConnectionAdapter
import de.fhg.aisec.ids.idscp2.idscp_core.ra_registry.RaProverDriverRegistry
import de.fhg.aisec.ids.idscp2.idscp_core.ra_registry.RaVerifierDriverRegistry
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import org.slf4j.LoggerFactory
import java.nio.charset.StandardCharsets

class Idscp2Client constructor(configuration: Idscp2Configuration, nativeTlsConfiguration: NativeTlsConfiguration) {

    val configuration = configuration
    val nativeTlsConfiguration = nativeTlsConfiguration

    init{
        // register ra drivers
        RaProverDriverRegistry.registerDriver(
            RaProverDummy2.RA_PROVER_DUMMY2_ID, ::RaProverDummy2, null
        )

        RaVerifierDriverRegistry.registerDriver(
            RaVerifierDummy2.RA_VERIFIER_DUMMY2_ID, ::RaVerifierDummy2, null
        )
    }

    fun send(message: Message, headers: Map<String, String>?, payload: ByteArray?): Triple<Message?, ByteArray?, Map<String,String>?>{
        var resultMessage: Message? = null
        var resultPayload: ByteArray? = null
        var resultHeaders: Map<String, String>? = null

        // Use this latch for waiting
        val latch = FastLatch()

        val secureChannelDriver = NativeTLSDriver<AppLayerConnection>()
        val connectionFuture = secureChannelDriver.connect(::AppLayerConnection, configuration, nativeTlsConfiguration)
        connectionFuture.thenAccept { connection: AppLayerConnection ->
            LOG.info("Client: New connection with id " + connection.id)
            connection.addConnectionListener(object : Idscp2ConnectionAdapter() {
                override fun onError(t: Throwable) {
                    LOG.error("Client connection error occurred", t)
                }

                override fun onClose() {
                    LOG.info("Client: Connection with id " + connection.id + " has been closed")
                    latch.unlock()
                }
            })
            connection.addIdsMessageListener { c: AppLayerConnection, m: Message?, data: ByteArray?, headers: Map<String, String> ->
                resultMessage = m
                resultHeaders = headers
                resultPayload = data
                headers.forEach { (name, value) ->
                    LOG.debug("Found header '{}':'{}'", name, value)
                }
                LOG.debug("All headers logged!")
                LOG.info("Received IDS message: " + Serializer().serialize(m))
                LOG.info("with payload: " + String(data!!, StandardCharsets.UTF_8))
                c.close()
            }

            connection.unlockMessaging()
            LOG.info("Send Message ...")
            connection.sendIdsMessage(message, payload, headers)
            LOG.info("Local DAT: " + String(connection.localDynamicAttributeToken, StandardCharsets.UTF_8))
        }.exceptionally { t: Throwable? ->
            LOG.error("Client endpoint error occurred", t)
            latch.unlock()
            null
        }

        // Wait until error or connection close
        latch.await()
        return Triple(resultMessage, resultPayload, resultHeaders)
    }

    companion object {
        private val LOG = LoggerFactory.getLogger(Idscp2Client::class.java)
    }
}
