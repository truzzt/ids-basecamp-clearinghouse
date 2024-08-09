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
package de.truzzt.clearinghouse.edc.app.delegate;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.message.CreateProcessRequest;
import de.truzzt.clearinghouse.edc.app.message.CreateProcessResponse;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageResponse;
import de.truzzt.clearinghouse.edc.app.types.Header;
import de.truzzt.clearinghouse.edc.app.types.SecurityToken;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;

import java.io.IOException;

public class CreateProcessDelegate extends AppSenderDelegate<CreateProcessResponse> {

    private final Monitor monitor;

    public CreateProcessDelegate(Monitor monitor, ObjectMapper objectMapper) {
        super(objectMapper);
        this.monitor = monitor;
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {
        return baseUrl + "/process/" + handlerRequest.getPid();
    }

    public CreateProcessRequest buildRequestBody(HandlerRequest handlerRequest) {
        var header = handlerRequest.getHeader();

        var multipartSecurityToken = header.getSecurityToken();
        var securityToken = SecurityToken.Builder.newInstance().
                type(multipartSecurityToken).
                id(multipartSecurityToken.getId()).
                tokenValue(multipartSecurityToken.getTokenValue()).
                build();

        var requestHeader = Header.Builder.newInstance()
                .id(header.getId())
                .type(header)
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return new CreateProcessRequest(requestHeader, handlerRequest.getPayload());
    }

    @Override
    public CreateProcessResponse buildSuccessResponse(ResponseBody responseBody) {
        return parseSuccessResponse(responseBody, CreateProcessResponse.class);
    }

    @Override
    public CreateProcessResponse buildErrorResponse(int httpStatus) {
        return new CreateProcessResponse(httpStatus);
    }
}
