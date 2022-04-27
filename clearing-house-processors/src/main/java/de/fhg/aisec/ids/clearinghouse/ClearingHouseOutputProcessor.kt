/*-
 * ========================LICENSE_START=================================
 * camel-multipart-processor
 * %%
 * Copyright (C) 2019 Fraunhofer AISEC
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
package de.fhg.aisec.ids.clearinghouse

import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriver
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityProfile
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements.Builder
import de.fraunhofer.iais.eis.*
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.slf4j.LoggerFactory
import java.net.URI
import java.nio.charset.StandardCharsets
import java.time.LocalDateTime
import javax.xml.datatype.DatatypeFactory

class ClearingHouseOutputProcessor : Processor {

    override fun process(exchange: Exchange) {
        processClearingHouseOutput(exchange)
    }

    companion object {
        private val LOG = LoggerFactory.getLogger(ClearingHouseOutputProcessor::class.java)
        private val SERIALIZER = Serializer()

        fun processClearingHouseOutput(exchange: Exchange) {
            val egetIn = exchange.getIn()
            val headers = egetIn.headers
            if (LOG.isTraceEnabled) {
                LOG.trace("[IN] ${ClearingHouseOutputProcessor::class.java.simpleName}")
                for (header in headers.keys) {
                    LOG.trace("Found header '{}':'{}'", header, headers[header])
                }
            }

            // get DAPS token
            val securityRequirements: SecurityRequirements = Builder()
                .setRequiredSecurityLevel(SecurityProfile.TRUSTED)
                .build()
            val dapsConfig = Configuration.createDapsConfig(securityRequirements)
            val dapsDriver = AisecDapsDriver(dapsConfig)
            val dapsToken = DynamicAttributeTokenBuilder()
                ._tokenFormat_(TokenFormat.JWT)
                ._tokenValue_(String(dapsDriver.token, StandardCharsets.UTF_8))
                .build()

            // If this property is null, the routes are not defined correctly!
            val originalRequest = exchange.getProperty(IDS_MESSAGE_HEADER, Message::class.java)

            val statusCode = (headers[CAMEL_HTTP_STATUS_CODE_HEADER] as Int?)!!.toInt()
            // creating IDS header for the response
            val responseMessage = when (statusCode) {
                200 -> ResultMessageBuilder()
                        ._issued_(
                            DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString())
                        )
                        ._modelVersion_(Configuration.infomodelVersion)
                        ._issuerConnector_(URI(Configuration.issuerConnector))
                        ._senderAgent_(URI(Configuration.senderAgent))
                        ._correlationMessage_(originalRequest.id)
                        ._recipientAgent_(listOf(originalRequest.senderAgent))
                        ._recipientConnector_(listOf(originalRequest.issuerConnector))
                        ._securityToken_(dapsToken).build()
                201 -> MessageProcessedNotificationMessageBuilder()
                        ._issued_(
                            DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString())
                        )
                        ._modelVersion_(Configuration.infomodelVersion)
                        ._issuerConnector_(URI(Configuration.issuerConnector))
                        ._senderAgent_(URI(Configuration.senderAgent))
                        ._correlationMessage_(originalRequest.id)
                        ._recipientAgent_(listOf(originalRequest.senderAgent))
                        ._recipientConnector_(listOf(originalRequest.issuerConnector))
                        ._securityToken_(dapsToken).build()
                else -> RejectionMessageBuilder()
                        ._issued_(
                            DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString())
                        )
                        ._modelVersion_(Configuration.infomodelVersion)
                        ._issuerConnector_(URI(Configuration.issuerConnector))
                        ._senderAgent_(URI(Configuration.senderAgent))
                        ._correlationMessage_(originalRequest.id)
                        ._recipientAgent_(listOf(originalRequest.senderAgent))
                        ._recipientConnector_(listOf(originalRequest.issuerConnector))
                        ._securityToken_(dapsToken).build()
            }

            // set the IDS header
            when (headers[IDS_PROTOCOL] as String){
                PROTO_IDSCP2 -> {
                    egetIn.setHeader(IDSCP2_IDS_HEADER, responseMessage)
                    when(statusCode){
                        400 -> egetIn.body = "Bad Request"
                        401 -> egetIn.body = "Unauthorized"
                        403 -> egetIn.body = "Forbidden"
                        404 -> egetIn.body = "Not Found"
                        500 -> egetIn.body = "Internal Server Error"
                    }
                }
                PROTO_MULTIPART -> egetIn.setHeader(CAMEL_MULTIPART_HEADER, SERIALIZER.serialize(responseMessage))
            }

            // clean up headers
            egetIn.removeHeader(IDS_PROTOCOL)
        }
    }
}