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
  static final String IDS_HEADER = "IDS-Header";
  static final String PROCESS_FAILED = "process_failed";
  static final String PID_HEADER = "pid";
  static final String TYPE_HEADER = "Content-Type";
  static final String IDSCP2_PAYLOAD_HEADER = "Idscp2-payload";
  static final String IDSCP2_PAYLOAD_TYPE_HEADER = "Idscp2-payload-type";
  static final String AUTH_HEADER = "Authorization";
  static final String TYPE_JSON = "application/json";
  static final String SERVER = "Server";
  static final String BEARER = "Bearer ";
}
