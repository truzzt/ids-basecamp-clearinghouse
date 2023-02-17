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

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.IDS_HEADER
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.SERVICE_CLAIM
import de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.SERVICE_HEADER
import de.fraunhofer.iais.eis.Message
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component
import java.util.Date

/**
 * This processor validates the JWT token in the IDS header
 */
@Component("chSharedSecretProcessor")
class SharedSecretProcessor : Processor {
    override fun process(exchange: Exchange) {
        val eIn = exchange.getIn()
        val headers = eIn.headers

//        if (LOG.isDebugEnabled) {
            LOG.debug("[IN] ${SharedSecretProcessor::class.java.simpleName}")
            for (header in headers.keys) {
                LOG.debug("Found header '{}':'{}'", header, headers[header])
            }
//        }

        val idsHeader = exchange.message.getHeader(IDS_HEADER) as Message
        //val idsHeader = exchange.getProperty(IDS_HEADER, Message::class.java)
        //    ?: throw RuntimeException("No IDS header provided!")
        val dat = idsHeader.securityToken?.tokenValue ?: throw RuntimeException("No DAT provided!")

        val decodedDat = JWT.decode(dat)
        val claimedClientId = decodedDat.subject
        val now = System.currentTimeMillis()
        val serviceToken = JWT.create()
            .withAudience(Configuration.serviceIdLog)
            .withIssuer(Configuration.serviceIdTc)
            .withClaim(SERVICE_CLAIM, claimedClientId)
            .withIssuedAt(Date(now))
            .withExpiresAt(Date(now + 60000))
            .sign(Algorithm.HMAC256(Configuration.serviceSecret))
        exchange.getIn().setHeader(SERVICE_HEADER, serviceToken)
    }

    companion object {
        val LOG: Logger = LoggerFactory.getLogger(SharedSecretProcessor::class.java)
    }
}
