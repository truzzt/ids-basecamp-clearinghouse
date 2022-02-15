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

import de.fraunhofer.iais.eis.ids.jsonld.Serializer;
import de.fraunhofer.iais.eis.Message;
import org.apache.commons.fileupload.FileItem;
import org.apache.commons.fileupload.FileUpload;
import org.apache.commons.fileupload.FileUploadException;
import org.apache.commons.fileupload.UploadContext;
import org.apache.commons.fileupload.disk.DiskFileItemFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;

import static de.fhg.aisec.ids.clearinghouse.ClearingHouseConstants.*;

public class ClearingHouseParser implements UploadContext {

  private static final Logger LOG = LoggerFactory.getLogger(ClearingHouseParser.class);
  private static final Serializer SERIALIZER = new Serializer();
  private final InputStream multipartInput;
  private final String boundary;
  private Message header;
  private InputStream payload;
  private String payloadContentType;
  private String token;

  ClearingHouseParser(final InputStream multipartInput) throws FileUploadException, IOException {
    this.multipartInput = multipartInput;
    multipartInput.mark(10240);
    try (BufferedReader reader =
        new BufferedReader(new InputStreamReader(multipartInput, StandardCharsets.UTF_8))) {
      String boundaryLine = reader.readLine();
      if (boundaryLine == null) {
        throw new IOException("Message body appears to be empty, expected multipart boundary.");
      }
      this.boundary = boundaryLine.substring(2).trim();
      this.multipartInput.reset();
      for (FileItem i : new FileUpload(new DiskFileItemFactory()).parseRequest(this)) {
        String fieldName = i.getFieldName();
        LOG.debug("Field name: {}", fieldName);
        if (MULTIPART_HEADER.equals(fieldName)) {
          header = SERIALIZER.deserialize(
                  new String(i.getInputStream().readAllBytes(), StandardCharsets.UTF_8),
                  Message.class
          );
          if (header.getProperties() != null) {
            LOG.debug("keyset size: {}", header.getProperties().keySet().size());
            for (String key : header.getProperties().keySet()) {
              LOG.debug("Message Property: {} , value: {}", key, header.getProperties().get(key));
            }
          }
          LOG.debug("id: {}", header.getId());

          token = BEARER + header.getSecurityToken().getTokenValue();
        } else if (MULTIPART_PAYLOAD.equals(fieldName)) {
          payload = i.getInputStream();
          payloadContentType = i.getContentType();
          if (LOG.isDebugEnabled()) {
            LOG.debug("Found body with Content-Type \"{}\"", payloadContentType);
          }
        } else {
          throw new IOException("Unknown multipart field name detected: " + fieldName);
        }
      }
    }
  }

  @Override
  public String getCharacterEncoding() {
    return StandardCharsets.UTF_8.name();
  }

  @Override
  public int getContentLength() {
    return -1;
  }

  @Override
  public String getContentType() {
    return "multipart/form-data, boundary=" + this.boundary;
  }

  @Override
  public InputStream getInputStream() {
    return multipartInput;
  }

  @Override
  public long contentLength() {
    return -1;
  }

  public Message getHeader() {
    return header;
  }

  public String getToken() {
    return token;
  }

  public InputStream getPayload() {
    return payload;
  }

  public String getPayloadContentType() {
    return payloadContentType;
  }
}
