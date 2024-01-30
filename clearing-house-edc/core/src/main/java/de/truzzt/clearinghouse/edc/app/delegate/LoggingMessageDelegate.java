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
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import de.truzzt.clearinghouse.edc.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.types.clearinghouse.TokenFormat;
import okhttp3.ResponseBody;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartRequest;
import org.eclipse.edc.spi.EdcException;

public class LoggingMessageDelegate implements AppSenderDelegate<LoggingMessageResponse> {


    public LoggingMessageDelegate() {
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {
        return baseUrl + "/messages/log/" + handlerRequest.getPid();
    }

    public LoggingMessageRequest buildRequestBody(MultipartRequest multipartRequest) {
        var handlerRequest = (HandlerRequest) multipartRequest;
        var header = handlerRequest.getHeader();

        var multipartSecurityToken = header.getSecurityToken();
        var multipartTokenFormat = multipartSecurityToken.getTokenFormat();
        var securityToken = SecurityToken.Builder.newInstance().
                type(multipartSecurityToken.getClass().getSimpleName()).
                id(multipartSecurityToken.getId()).
                tokenFormat(new TokenFormat(multipartTokenFormat.getId())).
                tokenValue(multipartSecurityToken.getTokenValue()).
                build();

        var requestHeader = Header.Builder.newInstance()
                .id(header.getId())
                .type(header.getClass().getSimpleName())
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return new LoggingMessageRequest(requestHeader, handlerRequest.getPayload());
    }

    @Override
    public LoggingMessageResponse parseResponseBody(ResponseBody responseBody) {
        try {
            return new ObjectMapper().readValue(responseBody.byteStream(), LoggingMessageResponse.class);
        } catch (Exception e) {
            throw new EdcException("Error reading byte to LoggingMessageResponse", e);
        }
    }
}
