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
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.monitor.Monitor;

import java.io.IOException;

public class AppSender {
    private static final String JSON_CONTENT_TYPE = "application/json";

    private final Monitor monitor;
    private final EdcHttpClient httpClient;
    private final ObjectMapper objectMapper;

    public AppSender(Monitor monitor,
                     EdcHttpClient httpClient,
                     ObjectMapper objectMapper) {
        this.monitor = monitor;
        this.httpClient = httpClient;
        this.objectMapper = objectMapper;
    }

    public <R, P> P send(AppSenderRequest request, AppSenderDelegate<P> appSenderDelegate) {

        String json;
        try {
            json = objectMapper.writeValueAsString(request.getBody());
        } catch (JsonProcessingException jpe) {
            throw new EdcException("Error parsing request to Json", jpe);
        }

        var requestBody = RequestBody.create(json, MediaType.get(JSON_CONTENT_TYPE));

        var httpRequest = new Request.Builder()
                .url(request.getUrl())
                .addHeader("Ch-Service", request.getToken())
                .addHeader("Content-Type", JSON_CONTENT_TYPE)
                .post(requestBody)
                .build();

        try (var response = httpClient.execute(httpRequest)) {
            monitor.debug("Response received from Clearing House App. Status: " + response.code());

            if (response.isSuccessful()) {
                try (var body = response.body()) {
                    if (body == null) {
                        throw new EdcException("Received an empty response body from Clearing House App");
                    }
                    return appSenderDelegate.buildSuccessResponse(body);
                }
            } else {
                return appSenderDelegate.buildErrorResponse(response.code());
            }
        } catch (IOException e) {
            throw new EdcException("Error sending request to Clearing House App", e);
        }
    }
}
