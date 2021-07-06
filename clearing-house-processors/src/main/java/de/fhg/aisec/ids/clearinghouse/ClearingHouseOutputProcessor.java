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

import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriver;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityProfile;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements;
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessage;
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessageBuilder;
import de.fraunhofer.iais.eis.ids.jsonld.Serializer;
import org.apache.camel.Exchange;
import org.apache.camel.Processor;
import org.apache.http.HttpEntity;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.mime.HttpMultipartMode;
import org.apache.http.entity.mime.MultipartEntityBuilder;
import org.apache.http.entity.mime.content.StringBody;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Paths;
import java.util.Map;
import java.util.UUID;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseOutputProcessor implements Processor {

  private final Logger LOG = LoggerFactory.getLogger(ClearingHouseOutputProcessor.class);
  private static final Serializer SERIALIZER = new Serializer();

  private MessageProcessedNotificationMessage resultMessage;

  @Override
  public void process(Exchange exchange) throws Exception {
    String boundary = UUID.randomUUID().toString();
    final var egetIn = exchange.getIn();
    final var typeHeader = egetIn.getHeader(TYPE_HEADER).toString();
    final var idsHeader = egetIn.getHeader(IDS_HEADER).toString();
    final var securityRequirements = new SecurityRequirements.Builder()
            .setRequiredSecurityLevel(SecurityProfile.TRUSTED)
            .build();
    final var dapsConfig = new AisecDapsDriverConfig.Builder()
            .setKeyStorePath(Paths.get("/root/etc/consumer-server-keystore.jks"))
            .setTrustStorePath(Paths.get("/root/etc/consumer-server-truststore.jks"))
            .setKeyAlias("1.0.1")
            .setSecurityRequirements(securityRequirements)
            .build();

    final var dapsDriver = new AisecDapsDriver(dapsConfig);

    Map<String, Object> headers = egetIn.getHeaders();
    for (String header: headers.keySet()){
      LOG.debug("Found header '{}':'{}'", header, headers.get(header));
    }

    dapsDriver.getToken();

    // preparation
    MultipartEntityBuilder multipartEntityBuilder = MultipartEntityBuilder.create();
    multipartEntityBuilder.setMode(HttpMultipartMode.STRICT);
    multipartEntityBuilder.setBoundary(boundary);

    // handling IDS header
    if (!idsHeader.isEmpty()) {
      if (LOG.isDebugEnabled()) {
        LOG.debug("IDS-Header is not empty, using header from original message");
      }

      //If we need to add a security token, this would be the place. Deserialize the token and add it.
      resultMessage = SERIALIZER.deserialize(idsHeader, MessageProcessedNotificationMessage.class);
      MessageProcessedNotificationMessageBuilder resultMessageBuilder = new MessageProcessedNotificationMessageBuilder();

      multipartEntityBuilder.addPart(
            ClearingHouseConstants.MULTIPART_HEADER,
            new StringBody(idsHeader, ContentType.APPLICATION_JSON));
    }
    else{
      if (LOG.isDebugEnabled()) {
        LOG.warn("IDS-Header is empty, using header from self description");
      }
      //TODO: Actually this is not supposed to happen. But we should be defensive about this and add it
      //The following code is from tc multipart processor
      //InfoModel infoModel: InfoModel = MultiPartComponent.infoModelManager
      //val rdfHeader = infoModel.connectorAsJsonLd
      multipartEntityBuilder.addPart(
              ClearingHouseConstants.MULTIPART_HEADER,
              new StringBody(idsHeader, ContentType.APPLICATION_JSON));
    }

    // get the body of the Clearing House message and put it into the payload
    String payload = exchange.getIn().getBody(String.class);
    if (payload != null) {
      if (LOG.isDebugEnabled()) {
          LOG.debug("Payload is not empty");
      }
      //message from the Clearing House are small, so we use Strings instead of Streams
      multipartEntityBuilder.addPart(
          ClearingHouseConstants.MULTIPART_PAYLOAD,
          new StringBody(payload, ContentType.create(typeHeader)));

    }
    else {
      if (LOG.isDebugEnabled()) {
        LOG.debug("Payload is empty");
      }
    }

    // Clean up the headers
    exchange.getIn().removeHeader(AUTH_HEADER);
    exchange.getIn().removeHeader(IDS_HEADER);
    exchange.getIn().removeHeader(PID_HEADER);
    exchange.getIn().removeHeader(SERVER);
    // Remove current Content-Type header before setting the new one
    exchange.getIn().removeHeader(TYPE_HEADER);
    // Set Content-Type for multipart message
    exchange.getIn().setHeader(TYPE_HEADER, "multipart/form-data; boundary=" + boundary);

    // Wrap up message
    HttpEntity entity = multipartEntityBuilder.build();
    LOG.debug("Created entity: {}", entity.getContent());
    exchange.getIn().setBody(entity.getContent());
  }
}
