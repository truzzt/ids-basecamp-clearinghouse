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

import java.io.IOException;
import java.io.InputStream;
import java.util.Map;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseMultipartInputProcessor implements Processor {

  static final Logger LOG = LoggerFactory.getLogger(ClearingHouseMultipartInputProcessor.class);

  @Override
  public void process(Exchange exchange) throws Exception {

    final var egetIn = exchange.getIn();

    Map<String, Object> headers = egetIn.getHeaders();
    for (String header: headers.keySet()){
      LOG.debug("Found header '{}':'{}'", header, headers.get(header));
    }

    try {
      // try to parse Infomodel header and build composite object
      ClearingHouseParser parser =
              new ClearingHouseParser(egetIn.getBody(InputStream.class));
      ClearingHouseMessage converted = new ClearingHouseMessage();
      converted.setHeader(parser.getHeader());
      converted.setPayloadType(parser.getPayloadContentType());

      // check that payload type of create pid message is application/json
      if (converted.header instanceof RequestMessage){
        ContentType expectedContentType = ContentType.create("application/json");
        if (expectedContentType.getMimeType().equals(converted.payloadType)){
          LOG.debug("Received payload:", converted.toJson(), converted.payload, converted.payloadType);
          converted.setPayload(parser.getPayload());
        }
        else{
          converted.setPayload(null);
        }
      }
      else{
        converted.setPayload(parser.getPayload());
      }

      LOG.debug("Build CH message: {}, with payload {} and payload type {}", converted.toJson(), converted.payload, converted.payloadType);
      // Remove current Content-Type header before setting the new one
      exchange.getIn().removeHeader(TYPE_HEADER);
      // Copy Content-Type from payload part populate body with new payload
      exchange.getIn().setHeader(TYPE_HEADER, TYPE_JSON);
      exchange.getIn().setHeader(IDS_HEADER, converted.header);
      exchange.getIn().setHeader(AUTH_HEADER, parser.getToken());
      exchange.getIn().setBody(converted.toJson());
    }
    catch (IOException io){
      LOG.error("Error parsing infomodel header");
      exchange.getIn().setHeader(PROCESS_FAILED, true);
    }
  }
}
