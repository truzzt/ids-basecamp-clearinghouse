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
import de.truzzt.clearinghouse.edc.app.message.CreateProcessResponse;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageResponse;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.app.types.Header;
import de.truzzt.clearinghouse.edc.app.types.SecurityToken;
import okhttp3.ResponseBody;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartRequest;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;

import java.io.IOException;

public class LoggingMessageDelegate extends AppSenderDelegate<LoggingMessageResponse> {

    private final Monitor monitor;

    public LoggingMessageDelegate(Monitor monitor, ObjectMapper objectMapper) {
        super(objectMapper);
        this.monitor = monitor;
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {
        return baseUrl + "/messages/log/" + handlerRequest.getPid();
    }

    public LoggingMessageRequest buildRequestBody(MultipartRequest multipartRequest) {
        var handlerRequest = (HandlerRequest) multipartRequest;
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

        return new LoggingMessageRequest(requestHeader, handlerRequest.getPayload());
    }

    @Override
    public LoggingMessageResponse buildSuccessResponse(ResponseBody responseBody) {
        return parseSuccessResponse(responseBody, LoggingMessageResponse.class);
    }

    @Override
    public LoggingMessageResponse buildErrorResponse(int httpStatus) {
        return new LoggingMessageResponse(httpStatus);
    }
}
