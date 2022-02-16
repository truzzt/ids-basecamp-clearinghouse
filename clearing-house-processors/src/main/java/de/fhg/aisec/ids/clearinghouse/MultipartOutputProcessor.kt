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
import de.fraunhofer.iais.eis.ids.jsonld.Serializer
import org.apache.camel.Exchange
import org.apache.camel.Processor
import org.apache.http.entity.ContentType
import org.apache.http.entity.mime.HttpMultipartMode
import org.apache.http.entity.mime.MultipartEntityBuilder
import org.apache.http.entity.mime.content.StringBody
import org.slf4j.LoggerFactory
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.InputStream
import java.util.*

class MultipartOutputProcessor : Processor {

    override fun process(exchange: Exchange) {
        processMultipartOutput(exchange)
    }

    companion object {
        private val LOG = LoggerFactory.getLogger(MultipartOutputProcessor::class.java)
        private val SERIALIZER = Serializer()

        fun processMultipartOutput(exchange: Exchange) {
            val egetIn = exchange.getIn()
            val headers = egetIn.headers
            if (LOG.isTraceEnabled) {
                LOG.trace("[IN] ${MultipartOutputProcessor::class.java.simpleName}")
                for (header in headers.keys) {
                    LOG.trace("Found header '{}':'{}'", header, headers[header])
                }
            }

            // preparation
            val multipartBoundary = UUID.randomUUID().toString()
            val typeHeader = egetIn.getHeader(TYPE_HEADER).toString()
            val multipartEntityBuilder = MultipartEntityBuilder.create()
            multipartEntityBuilder.setMode(HttpMultipartMode.STRICT)
            multipartEntityBuilder.setBoundary(multipartBoundary)

            // ids header
            multipartEntityBuilder.addPart(
                MULTIPART_HEADER,
                StringBody(headers[CAMEL_MULTIPART_HEADER] as String, ContentType.APPLICATION_JSON)
            )

            // only output the body of the message if status code indicates "success"
            val statusCode = (headers[CAMEL_HTTP_STATUS_CODE_HEADER] as Int?)!!.toInt()
            val payload = exchange.getIn().getBody(String::class.java)
            when (statusCode) {
                200, 201 ->
                    //message from the Clearing House are small, so we use Strings instead of Streams
                    multipartEntityBuilder.addPart(
                        MULTIPART_PAYLOAD,
                        StringBody(payload, ContentType.create(typeHeader))
                    )
                else -> LOG.warn("Status Code: {} with Payload: {}", statusCode, payload)
            }

            // Clean up the headers
            exchange.getIn().removeHeader(AUTH_HEADER)
            exchange.getIn().removeHeader(PID_HEADER)
            exchange.getIn().removeHeader(SERVER)
            exchange.getIn().removeHeader(TYPE_HEADER)
            exchange.getIn().removeHeader(CAMEL_MULTIPART_HEADER)

            // Wrap up message
            val resultEntity = multipartEntityBuilder.build()
            // Set Content-Type for multipart message
            exchange.getIn().setHeader(TYPE_HEADER, resultEntity.contentType.value)
            val out = ByteArrayOutputStream()
            resultEntity.writeTo(out)
            val inputStream: InputStream = ByteArrayInputStream(out.toByteArray())
            exchange.getIn().body = inputStream
        }
    }
}