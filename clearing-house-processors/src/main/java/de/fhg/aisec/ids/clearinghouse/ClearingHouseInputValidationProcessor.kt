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

import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.CAMEL_HTTP_PATH
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_ID_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_PAGE_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_PID_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_SIZE_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDSCP_SORT_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_PROTOCOL
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.PROTO_IDSCP2
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.TYPE_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.TYPE_JSON
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.QueryMessage
import de.fraunhofer.iais.eis.RequestMessage
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.apache.http.entity.ContentType
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component

@Component("chInputValidationProcessor")
class ClearingHouseInputValidationProcessor : Processor {
    override fun process(exchange: Exchange) {
        val egetIn = exchange.getIn()
        val headers = egetIn.headers
        val body = exchange.message.getBody(ByteArray::class.java)

        if (LOG.isTraceEnabled) {
            LOG.trace("[IN] ${ClearingHouseInputValidationProcessor::class.java.simpleName}")
            for (header in headers.keys) {
                LOG.trace("Found header '{}':'{}'", header, headers[header])
            }
       }

        // Prepare compound message for Clearing House Service API
        val idsHeader = exchange.message.getHeader(IDS_HEADER) as Message
        val contentTypeHeader = (headers[TYPE_HEADER] as String?)
        val chMessage = ClearingHouseMessage(idsHeader, contentTypeHeader, body)

        LOG.info("idsmessage: {}", idsHeader.id)

        // Input validation: check that payload type of create pid message is application/json
        if (chMessage.header is RequestMessage && idsHeader !is QueryMessage) {
            val expectedContentType = ContentType.create("application/json")
            if (expectedContentType.mimeType != chMessage.payloadType) {
                LOG.warn("Expected application/json, got {}", chMessage.payloadType)
                throw IllegalArgumentException("Expected content-type application/json")
            }
        }

        // Input validation: construct url from headers for IDSCP2
        if (headers[IDS_PROTOCOL] == PROTO_IDSCP2) {
            if (chMessage.header is QueryMessage) {
                val queryPath = if (headers.contains(IDSCP_ID_HEADER)) {
                    (headers[CAMEL_HTTP_PATH] as String) + "/" + (headers[IDSCP_ID_HEADER] as String)
                } else {
                    var paginationPath = if (headers.contains(IDSCP_PAGE_HEADER)) {
                        (headers[CAMEL_HTTP_PATH] as String) + "?page=" + exchange.message.getHeader(IDSCP_PAGE_HEADER)
                    } else {
                        (headers[CAMEL_HTTP_PATH] as String) + "?page=1"
                    }

                    if (headers.contains(IDSCP_SIZE_HEADER)) {
                        paginationPath = paginationPath + "&size=" + exchange.message.getHeader(IDSCP_SIZE_HEADER)
                    }
                    if (headers.contains(IDSCP_SORT_HEADER)) {
                        paginationPath = "$paginationPath?sort=desc"
                    }
                    paginationPath
                }
                exchange.getIn().setHeader(CAMEL_HTTP_PATH, queryPath)
            }
        }

        if (LOG.isTraceEnabled) {
            LOG.trace("Received payload: {}", chMessage.payload)
        }

        // store ids header for response processor and clean up idscp2 specific header
        exchange.getIn().removeHeader(IDSCP_ID_HEADER)
        exchange.getIn().removeHeader(IDSCP_PID_HEADER)

        // Remove current Content-Type header before setting the new one
        exchange.getIn().removeHeader(TYPE_HEADER)

        // Copy Content-Type from payload part populate body with new payload
        exchange.getIn().setHeader(TYPE_HEADER, TYPE_JSON)
        exchange.getIn().body = chMessage.toJson()
    }

    companion object {
        private val LOG: Logger = LoggerFactory.getLogger(ClearingHouseInputValidationProcessor::class.java)
    }
}
