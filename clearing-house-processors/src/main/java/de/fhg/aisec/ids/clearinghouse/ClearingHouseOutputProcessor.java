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

import javax.xml.datatype.DatatypeFactory;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.Map;
import java.util.UUID;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseOutputProcessor implements Processor {

  private final Logger LOG = LoggerFactory.getLogger(ClearingHouseOutputProcessor.class);
  private static final Serializer SERIALIZER = new Serializer();

  @Override
  public void process(Exchange exchange) throws Exception {
    String boundary = UUID.randomUUID().toString();
    final var egetIn = exchange.getIn();
    final var typeHeader = egetIn.getHeader(TYPE_HEADER).toString();
    final var idsHeader = egetIn.getHeader(IDS_HEADER) == null? "" : egetIn.getHeader(IDS_HEADER).toString();
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
    final var original_request = (Message)exchange.getIn().getHeader(IDS_HEADER_COPY);

    // Clean up the headers
    exchange.getIn().removeHeader(AUTH_HEADER);
    exchange.getIn().removeHeader(IDS_HEADER);
    exchange.getIn().removeHeader(PID_HEADER);
    exchange.getIn().removeHeader(SERVER);
    exchange.getIn().removeHeader(TYPE_HEADER);
    exchange.getIn().removeHeader(IDS_HEADER_COPY);

    // preparation
    MultipartEntityBuilder multipartEntityBuilder = MultipartEntityBuilder.create();
    multipartEntityBuilder.setMode(HttpMultipartMode.STRICT);
    multipartEntityBuilder.setBoundary(boundary);

    // get DAPS token
    DynamicAttributeToken dapsToken = new DynamicAttributeTokenBuilder()
            ._tokenFormat_(TokenFormat.JWT)
            ._tokenValue_(new String(dapsDriver.getToken(), StandardCharsets.UTF_8))
            .build();

    // handling IDS header
    if (!idsHeader.isEmpty()) {
      if (LOG.isDebugEnabled()) {
        LOG.debug("IDS-Header is not empty, using header from original message");
      }

    }
    else {
      if (LOG.isDebugEnabled()) {
        LOG.warn("IDS-Header is empty");
      }
    }

    // creating IDS header for the response
    String responseHeader;
    switch (statusCode){
      case 200:
        responseHeader = SERIALIZER.serialize(new ResultMessageBuilder()
                ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                ._modelVersion_(Configuration.getInfomodelVersion())
                ._issuerConnector_(new URI(Configuration.getIssuerConnector()))
                ._senderAgent_(new URI(Configuration.getSenderAgent()))
                ._correlationMessage_(original_request.getId())
                ._recipientAgent_(Arrays.asList(new URI[]{original_request.getSenderAgent()}))
                ._recipientConnector_(Arrays.asList(new URI[]{original_request.getIssuerConnector()}))
                ._securityToken_(dapsToken).build());
        break;
      case 201:
        responseHeader = SERIALIZER.serialize(new MessageProcessedNotificationMessageBuilder()
                ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                ._modelVersion_(Configuration.getInfomodelVersion())
                ._issuerConnector_(new URI(Configuration.getIssuerConnector()))
                ._senderAgent_(new URI(Configuration.getSenderAgent()))
                ._correlationMessage_(original_request.getId())
                ._recipientAgent_(Arrays.asList(new URI[]{original_request.getSenderAgent()}))
                ._recipientConnector_(Arrays.asList(new URI[]{original_request.getIssuerConnector()}))
                ._securityToken_(dapsToken).build());
        break;
      default:
        responseHeader = SERIALIZER.serialize(new RejectionMessageBuilder()
                ._issued_(DatatypeFactory.newInstance().newXMLGregorianCalendar(LocalDateTime.now().toString()))
                ._modelVersion_(Configuration.getInfomodelVersion())
                ._issuerConnector_(new URI(Configuration.getIssuerConnector()))
                ._senderAgent_(new URI(Configuration.getSenderAgent()))
                ._correlationMessage_(original_request.getId())
                ._recipientAgent_(Arrays.asList(new URI[]{original_request.getSenderAgent()}))
                ._recipientConnector_(Arrays.asList(new URI[]{original_request.getIssuerConnector()}))
                ._securityToken_(dapsToken).build());
    }

    multipartEntityBuilder.addPart(
            ClearingHouseConstants.MULTIPART_HEADER,
            new StringBody(responseHeader, ContentType.APPLICATION_JSON));

    // get the body of the Clearing House message and put it into the payload
    String payload = exchange.getIn().getBody(String.class);
    // add body only in case of success
    switch (statusCode){
      case 200:
      case 201:
        //message from the Clearing House are small, so we use Strings instead of Streams
        multipartEntityBuilder.addPart(
                ClearingHouseConstants.MULTIPART_PAYLOAD,
                new StringBody(payload, ContentType.create(typeHeader)));
        break;
      default:
        LOG.warn("Status Code: {} with Payload: {}", statusCode, payload);
    }

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

