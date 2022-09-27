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
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.QueryMessage
import de.fraunhofer.iais.eis.RequestMessage
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.apache.http.entity.ContentType
import org.slf4j.LoggerFactory

class ClearingHouseIdscp2InputProcessor : Processor {
    override fun process(exchange: Exchange) {
        processIdsc2Input(exchange)
    }

    companion object {
        val LOG = LoggerFactory.getLogger(ClearingHouseIdscp2InputProcessor::class.java)

        fun processIdsc2Input(exchange: Exchange) {
            val egetIn = exchange.getIn()
            val headers = egetIn.headers

            if (LOG.isTraceEnabled) {
                LOG.trace("[IN] ${ClearingHouseIdscp2InputProcessor::class.java.simpleName}")
                for (header in headers.keys) {
                    LOG.trace("Found header '{}':'{}'", header, headers[header])
                }
            }

            // First step: store ids header for response processor so it's ready even if there's an exception later on
            val idsHeader = exchange.message.getHeader(IDSCP2_IDS_HEADER) as Message
            exchange.setProperty(IDS_MESSAGE_HEADER, idsHeader)
            exchange.getIn().setHeader(IDS_PROTOCOL, PROTO_IDSCP2)

            // Prepare compound message for Clearing House Service API
            val contentTypeHeader = (headers[TYPE_HEADER] as String?)
            val converted = ClearingHouseMessage(idsHeader, contentTypeHeader, exchange.message.body as ByteArray)

            if (ClearingHouseInfomodelParsingProcessor.LOG.isTraceEnabled) {
                ClearingHouseInfomodelParsingProcessor.LOG.trace("Received payload: {}", converted.payload)
            }
            // Input validation: check that payload type of create pid message is application/json
            if (converted.header is RequestMessage && converted.header !is QueryMessage) {
                val expectedContentType = ContentType.create("application/json")
                if (expectedContentType.mimeType != converted.payloadType) {
                    ClearingHouseInfomodelParsingProcessor.LOG.warn("Expected application/json, got {}", converted.payloadType)
                    throw IllegalArgumentException("Expected content-type application/json")
                }
            }
            // Input validation: check if there's a document id for query
            if (converted.header is QueryMessage){
                val queryPath = if (headers.contains(IDSCP_ID_HEADER)) {
                    (headers[CAMEL_HTTP_PATH] as String) + "/" + (headers[IDSCP_ID_HEADER] as String)
                } else{
                        var paginationPath = if (headers.contains(IDSCP_PAGE_HEADER)){
                            (headers[CAMEL_HTTP_PATH] as String) + "?page=" + exchange.message.getHeader(IDSCP_PAGE_HEADER)
                        } else{
                            (headers[CAMEL_HTTP_PATH] as String) + "?page=1"
                        }

                        if (headers.contains(IDSCP_SIZE_HEADER)){
                            paginationPath = paginationPath + "&size=" + exchange.message.getHeader(IDSCP_SIZE_HEADER)
                        }
                        if (headers.contains(IDSCP_SORT_HEADER)){
                            paginationPath = "$paginationPath?sort=desc"
                        }
                        paginationPath
                }
                exchange.getIn().setHeader(CAMEL_HTTP_PATH, queryPath)
            }

            // store ids header for response processor and clean up idscp2 specific header
            exchange.getIn().removeHeader(IDSCP2_IDS_HEADER)
            exchange.getIn().removeHeader(IDSCP_ID_HEADER)
            exchange.getIn().removeHeader(IDSCP_PID_HEADER)

            // Remove current Content-Type header before setting the new one
            exchange.getIn().removeHeader(TYPE_HEADER)

            // Copy Content-Type from payload part populate body with new payload
            exchange.getIn().setHeader(TYPE_HEADER, TYPE_JSON)
            exchange.getIn().body = converted.toJson()

        }
    }
}