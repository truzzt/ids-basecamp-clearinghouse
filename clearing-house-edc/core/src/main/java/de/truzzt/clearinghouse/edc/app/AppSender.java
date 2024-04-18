/*
 *  Copyright (c) 2023 truzzt GmbH
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       truzzt GmbH - Initial implementation
 *
 */
package de.truzzt.clearinghouse.edc.app;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.delegate.AppSenderDelegate;
import de.truzzt.clearinghouse.edc.app.message.AppSenderRequest;
import okhttp3.MediaType;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.monitor.Monitor;

import static java.lang.String.format;

public class AppSender {
    private static final String JSON_CONTENT_TYPE = "application/json";

    private final Monitor monitor;
    private final EdcHttpClient httpClient;

    public AppSender(Monitor monitor,
                     EdcHttpClient httpClient) {
        this.monitor = monitor;
        this.httpClient = httpClient;
    }

    public <R, P> P send(AppSenderRequest request, AppSenderDelegate<P> appSenderDelegate) {

        try{
            var json = new ObjectMapper().writeValueAsString(request.getBody());
            var requestBody = RequestBody.create(json, MediaType.get(JSON_CONTENT_TYPE));

            var httpRequest = new Request.Builder()
                    .url(request.getUrl())
                    .addHeader("Ch-Service", request.getToken())
                    .addHeader("Content-Type", JSON_CONTENT_TYPE)
                    .post(requestBody)
                    .build();

            Response response = httpClient.execute(httpRequest);
            monitor.debug("Response received from Clearing House App. Status: " + response.code());

            if (response.isSuccessful()) {
                try (var body = response.body()) {
                    if (body == null) {
                        throw new EdcException("Received an empty response body from Clearing House App");
                    } else {
                        return appSenderDelegate.parseResponseBody(body);
                    }
                } catch (Exception e) {
                    throw new EdcException("Error reading Clearing House App response body", e);
                }
            } else {
                throw new EdcException(format("Received an error from Clearing House App. Status: %s, message: %s",
                        response.code(), response.message()));
            }
        } catch (JsonProcessingException jpe){
            throw new EdcException("Error parsing request to Json", jpe);
        } catch (java.io.IOException e) {
             throw new EdcException("Error sending request to Clearing House App", e);
         }
    }
}
