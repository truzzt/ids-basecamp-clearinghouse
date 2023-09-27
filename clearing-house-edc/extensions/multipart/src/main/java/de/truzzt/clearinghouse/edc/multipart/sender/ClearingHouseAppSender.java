/*
 *  Copyright (c) 2022 sovity GmbH
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       sovity GmbH - initial API and implementation
 *
 */
package de.truzzt.clearinghouse.edc.multipart.sender;

import de.truzzt.clearinghouse.edc.multipart.message.ClearingHouseAppRequest;
import de.truzzt.clearinghouse.edc.multipart.sender.delegate.SenderDelegate;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import jakarta.ws.rs.core.MediaType;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.monitor.Monitor;

import static java.lang.String.format;

public class ClearingHouseAppSender {

    private final Monitor monitor;
    private final EdcHttpClient httpClient;
    private final TypeManagerUtil typeManagerUtil;

    public ClearingHouseAppSender(Monitor monitor,
                                  EdcHttpClient httpClient,
                                  TypeManagerUtil typeManagerUtil) {
        this.monitor = monitor;
        this.httpClient = httpClient;
        this.typeManagerUtil = typeManagerUtil;
    }

    public <R, P> P send(ClearingHouseAppRequest request, SenderDelegate<R, P> senderDelegate) {

        var json = typeManagerUtil.toJson(request.getBody());

        var requestBody = RequestBody.create(json,
                okhttp3.MediaType.get(MediaType.APPLICATION_JSON));

        var httpRequest = new Request.Builder()
                .url(request.getUrl())
                .addHeader("Ch-Service", request.getToken())
                .addHeader("Content-Type", MediaType.APPLICATION_JSON)
                .post(requestBody)
                .build();

        Response response;
        try {
            response = httpClient.execute(httpRequest);
            monitor.debug("Response received from Clearing House App. Status: " + response.code());

        } catch (java.io.IOException e) {
            throw new EdcException("Error sending request to Clearing House App", e);
        }

        if (response.isSuccessful()) {
            try (var body = response.body()) {
                if (body == null) {
                    throw new EdcException("Received an empty response body from Clearing House App");
                } else {
                    monitor.debug("Response received from Clearing House App. Body: " + body);

                    var responseBody = senderDelegate.parseResponseBody(body);
                    return responseBody;
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        } else {
            throw new EdcException(format("Received an error from Clearing House App. Status: %s, message: %s", response.code(), response.message()));
        }
    }
}
