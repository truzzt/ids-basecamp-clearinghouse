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
package de.fhg.aisec.ids.clearinghouse;

import de.fraunhofer.iais.eis.RequestMessage;
import org.apache.camel.Exchange;
import org.apache.camel.Processor;
import org.apache.http.entity.ContentType;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseCreatePidProcessor implements Processor {

  static final Logger LOG = LoggerFactory.getLogger(ClearingHouseCreatePidProcessor.class);

  @Override
  public void process(Exchange exchange) throws Exception {
    ClearingHouseParser parser =
        new ClearingHouseParser(exchange.getIn().getBody(InputStream.class), exchange, RequestMessage.class);
    // Get the IDS InfoModelManager and retrieve a JSON-LD-serialized self-description that will be sent as a multipart "header"
    ClearingHouseMessage converted = new ClearingHouseMessage();
    converted.setHeader(parser.getHeader());
    converted.setPayloadType(parser.getPayloadContentType());
    // validate payload: we require payload type: application/json with ISO_8859_1
    ContentType expectedContentType = ContentType.create("application/json");
    if (expectedContentType.getMimeType().equals(converted.payloadType)){
      LOG.debug("Received payload:", converted.toJson(), converted.payload, converted.payloadType);
      converted.setPayload(parser.getPayload());
    }
    else{
      converted.setPayload(null);
    }

    LOG.debug("Build CH message: {}, with payload {} and payload type {}", converted.toJson(), converted.payload, converted.payloadType);
    // Remove current Content-Type header before setting the new one
    exchange.getIn().removeHeader(TYPE_HEADER);
    // Copy Content-Type from payload part populate body with new payload
    exchange.getIn().setHeader(TYPE_HEADER, TYPE_JSON);
    exchange.getIn().setHeader(AUTH_HEADER, parser.getToken());
    exchange.getIn().setBody(converted.toJson());
  }
}
