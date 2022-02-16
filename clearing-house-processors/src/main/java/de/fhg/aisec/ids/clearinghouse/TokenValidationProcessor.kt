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
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.slf4j.LoggerFactory

/**
 * This processor validates the JWT token in the IDS header
 */
class TokenValidationProcessor : Processor {
    override fun process(exchange: Exchange) {
       processInfoModelInput(exchange)
    }

    companion object {
        val LOG = LoggerFactory.getLogger(TokenValidationProcessor::class.java)

        fun processInfoModelInput(exchange: Exchange){
            val egetIn = exchange.getIn()
            val headers = egetIn.headers

            if (LOG.isTraceEnabled) {
                LOG.trace("[IN] ${TokenValidationProcessor::class.java.simpleName}")
                for (header in headers.keys) {
                    LOG.trace("Found header '{}':'{}'", header, headers[header])
                }
            }

            val idsHeader = exchange.getProperty(IDS_MESSAGE_HEADER, Message::class.java)
            //TODO: validate that token aki:ski and certificate aki:ski match
            if (true){

            }
            else{
                LOG.warn("Connector with id: {} sent token with id:{}")
                throw SecurityException("Access Token did not match presented certificate!")
            }

            // extract security token from IDS header and set auth header
            val token = BEARER + (idsHeader?.securityToken?.tokenValue ?: "")
            exchange.getIn().setHeader(AUTH_HEADER, token)
        }
    }
}