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

import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_ID_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_PID_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_PROTOCOL
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.PROTO_IDSCP2
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.TYPE_HEADER
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.RejectionMessageBuilder
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component

@Component("chExceptionProcessor")
class ClearingHouseExceptionProcessor : Processor {
    override fun process(exchange: Exchange) {
        val egetIn = exchange.getIn()
        val headers = egetIn.headers

        if (LOG.isTraceEnabled) {
            LOG.trace("[ERR] ${ClearingHouseExceptionProcessor::class.java.simpleName}")
            for (header in headers.keys) {
                LOG.trace("Found header '{}':'{}'", header, headers[header])
            }
        }

        val originalRequest = exchange.message.getHeader(IDS_HEADER) as Message

        val message = RejectionMessageBuilder()
            ._correlationMessage_(originalRequest.id)
            ._recipientAgent_(listOf(originalRequest.senderAgent))
            ._recipientConnector_(listOf(originalRequest.issuerConnector))

        val caused = exchange.getProperty(Exchange.EXCEPTION_CAUGHT, Throwable::class.java)

        exchange.getIn().body = caused.message

        // set the IDS header
        when (headers[IDS_PROTOCOL] as String) {
            PROTO_IDSCP2 -> egetIn.setHeader(IDS_HEADER, message)
        }

        // clean up headers
        egetIn.removeHeader(IDS_PROTOCOL)
        egetIn.removeHeader(IDSCP_ID_HEADER)
        egetIn.removeHeader(IDSCP_PID_HEADER)
        egetIn.removeHeader(TYPE_HEADER)
    }

    companion object {
        private val LOG: Logger = LoggerFactory.getLogger(ClearingHouseExceptionProcessor::class.java)
    }
}
