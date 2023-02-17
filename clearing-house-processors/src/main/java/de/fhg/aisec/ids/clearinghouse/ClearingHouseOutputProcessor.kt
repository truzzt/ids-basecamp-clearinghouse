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

import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.CAMEL_HTTP_STATUS_CODE_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_PROTOCOL
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.PROTO_IDSCP2
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.PROTO_MULTIPART
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessageBuilder
import de.fraunhofer.iais.eis.RejectionMessageBuilder
import de.fraunhofer.iais.eis.ResultMessageBuilder
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component

@Component("chOutputProcessor")
class ClearingHouseOutputProcessor : Processor {

    override fun process(exchange: Exchange) {
        val egetIn = exchange.getIn()
        val headers = egetIn.headers
        if (LOG.isTraceEnabled) {
            LOG.trace("[IN] ${ClearingHouseOutputProcessor::class.java.simpleName}")
            for (header in headers.keys) {
                LOG.trace("Found header '{}':'{}'", header, headers[header])
            }
        }

        // If this property is null, the routes are not defined correctly!
        val originalRequest = exchange.message.getHeader(IDS_HEADER) as Message

        val statusCode = (headers[CAMEL_HTTP_STATUS_CODE_HEADER] as Int?)!!.toInt()
        // creating IDS header for the response
        val responseMessage = when (statusCode) {
            200 -> ResultMessageBuilder()
                ._correlationMessage_(originalRequest.id)
                ._recipientAgent_(listOf(originalRequest.senderAgent))
                ._recipientConnector_(listOf(originalRequest.issuerConnector))
            201 -> MessageProcessedNotificationMessageBuilder()
                ._correlationMessage_(originalRequest.id)
                ._recipientAgent_(listOf(originalRequest.senderAgent))
                ._recipientConnector_(listOf(originalRequest.issuerConnector))
            else -> RejectionMessageBuilder()
                ._correlationMessage_(originalRequest.id)
                ._recipientAgent_(listOf(originalRequest.senderAgent))
                ._recipientConnector_(listOf(originalRequest.issuerConnector))
        }

        // set the IDS header
        egetIn.setHeader(IDS_HEADER, responseMessage)

        // idscp2 set status code
        when (headers[IDS_PROTOCOL] as String){
            PROTO_IDSCP2 -> {
                when(statusCode){
                    400 -> egetIn.body = "Bad Request"
                    401 -> egetIn.body = "Unauthorized"
                    403 -> egetIn.body = "Forbidden"
                    404 -> egetIn.body = "Not Found"
                    500 -> egetIn.body = "Internal Server Error"
                }
            }
            PROTO_MULTIPART -> {
                when(statusCode){
                    200, 201 ->
                        if (LOG.isTraceEnabled) {
                            LOG.trace("[OUT] ${ClearingHouseOutputProcessor::class.java.simpleName}")
                            LOG.trace("Message successfully processed.")
                        }
                    else -> {
                        egetIn.body = ""
                    }
                }
            }
        }

        // Clean up the headers
        egetIn.removeHeader(ClearingHouseConstants.AUTH_HEADER)
        egetIn.removeHeader(ClearingHouseConstants.PID_HEADER)
        egetIn.removeHeader(ClearingHouseConstants.SERVER)
        egetIn.removeHeader(ClearingHouseConstants.TYPE_HEADER)
        egetIn.removeHeader(IDS_PROTOCOL)
    }

    companion object {
        private val LOG = LoggerFactory.getLogger(ClearingHouseOutputProcessor::class.java)
    }
}
