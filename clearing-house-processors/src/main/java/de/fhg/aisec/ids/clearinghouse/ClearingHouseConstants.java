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

final class ClearingHouseConstants {

  private ClearingHouseConstants() {}

  static final String MULTIPART_HEADER = "header";
  static final String MULTIPART_PAYLOAD = "payload";
  static final String CAMEL_MULTIPART_HEADER = "idsMultipartHeader";
  static final String CAMEL_HTTP_STATUS_CODE_HEADER = "CamelHttpResponseCode";
  static final String CAMEL_HTTP_PATH = "CamelHttpPath";
  static final String IDS_MESSAGE_HEADER = "idsc-header";
  static final String IDS_PROTOCOL = "ids-protocol";
  static final String PROTO_IDSCP2 = "idscp2";
  static final String PROTO_MULTIPART = "idsMultipart";
  static final String PID_HEADER = "pid";
  static final String IDSCP_PID_HEADER = "ch-ids-pid";
  static final String IDSCP_ID_HEADER = "ch-ids-id";
  static final String IDSCP_PAGE_HEADER = "ch-ids-page";
  static final String IDSCP_SIZE_HEADER = "ch-ids-size";
  static final String IDSCP_SORT_HEADER = "ch-ids-sort";
  static final String TYPE_HEADER = "Content-Type";
  static final String IDSCP2_IDS_HEADER = "idscp2-header";
  static final String AUTH_HEADER = "Authorization";
  static final String TYPE_JSON = "application/json";
  static final String SERVER = "Server";
  static final String BEARER = "Bearer ";
}
