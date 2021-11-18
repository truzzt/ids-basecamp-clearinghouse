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
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityProfile;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements;
import de.fraunhofer.iais.eis.*;
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

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.UUID;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseQueryOutputProcessor implements Processor {

  private final Logger LOG = LoggerFactory.getLogger(ClearingHouseQueryOutputProcessor.class);
  private static final Serializer SERIALIZER = new Serializer();
  private static final Integer STATUS_CODE_OK = 200;
  private static final Integer STATUS_CODE_CREATED = 201;

  private ResultMessage resultMessage;

  @Override
  public void process(Exchange exchange) throws Exception {
    String boundary = UUID.randomUUID().toString();
    final var egetIn = exchange.getIn();
    final var typeHeader = egetIn.getHeader(TYPE_HEADER).toString();
    final var idsHeader = egetIn.getHeader(IDS_HEADER).toString();
    final var securityRequirements = new SecurityRequirements.Builder()
            .setRequiredSecurityLevel(SecurityProfile.TRUSTED)
            .build();
    final var dapsConfig = Configuration.createDapsConfig(securityRequirements);
    final var dapsDriver = new AisecDapsDriver(dapsConfig);

    Map<String, Object> headers = egetIn.getHeaders();
    for (String header: headers.keySet()){
      LOG.debug("Found header '{}':'{}'", header, headers.get(header));
    }

    final var statusCode = ((Integer) headers.get("CamelHttpResponseCode")).intValue();

    // preparation
    MultipartEntityBuilder multipartEntityBuilder = MultipartEntityBuilder.create();
    multipartEntityBuilder.setMode(HttpMultipartMode.STRICT);
    multipartEntityBuilder.setBoundary(boundary);

    // handling IDS header
    if (!idsHeader.isEmpty()) {
      if (LOG.isDebugEnabled()) {
        LOG.debug("IDS-Header is not empty, using header from original message");
      }

      // We'll need this to store the modified idsHeader
      String headerWithDat;
      // The real token
      DynamicAttributeToken dapsToken = new DynamicAttributeTokenBuilder()
              ._tokenFormat_(TokenFormat.JWT)
              ._tokenValue_(new String(dapsDriver.getToken(), StandardCharsets.UTF_8))
              .build();
      // To add the DAT we need to deserialize the IDS message in the header and add the DAT
      // Depending on the status code we get different IDS messages
      LOG.debug("status code is: {}", statusCode);
      if (statusCode == STATUS_CODE_OK || statusCode == STATUS_CODE_CREATED){
        LOG.debug("Status code was ok");
        resultMessage = SERIALIZER.deserialize(idsHeader, ResultMessage.class);
        // InfoModel does not allow changing the message directly, so we use reflection
        Field securityToken = resultMessage.getClass().getDeclaredField("_securityToken");
        securityToken.setAccessible(true);
        securityToken.set(resultMessage, dapsToken);
        headerWithDat = SERIALIZER.serialize(resultMessage);
      }
      else{
        LOG.debug("Status code was not ok");
        RejectionMessage rejectionMessage = SERIALIZER.deserialize(idsHeader, RejectionMessage.class);
        // InfoModel does not allow changing the message directly, so we use reflection
        Field securityToken = rejectionMessage.getClass().getDeclaredField("_securityToken");
        securityToken.setAccessible(true);
        securityToken.set(rejectionMessage, dapsToken);
        headerWithDat = SERIALIZER.serialize(rejectionMessage);
      }

      multipartEntityBuilder.addPart(
            ClearingHouseConstants.MULTIPART_HEADER,
            new StringBody(headerWithDat, ContentType.APPLICATION_JSON));
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

    // Wrap up message
    HttpEntity resultEntity = multipartEntityBuilder.build();
    // Set Content-Type for multipart message
    exchange.getIn().setHeader(TYPE_HEADER, resultEntity.getContentType().getValue());
    ByteArrayOutputStream out = new ByteArrayOutputStream();
    resultEntity.writeTo(out);
    InputStream inputStream = new ByteArrayInputStream(out.toByteArray());
    exchange.getIn().setBody(inputStream);

  }
}
