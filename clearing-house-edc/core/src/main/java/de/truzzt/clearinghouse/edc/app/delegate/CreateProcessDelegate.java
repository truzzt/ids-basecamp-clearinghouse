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
import de.truzzt.clearinghouse.edc.app.types.Header;
import de.truzzt.clearinghouse.edc.app.types.SecurityToken;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;

public class CreateProcessDelegate implements AppSenderDelegate<CreateProcessResponse> {

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
    public CreateProcessResponse parseResponseBody(ResponseBody responseBody) {
        try {
            return new ObjectMapper().readValue(responseBody.byteStream(), CreateProcessResponse.class);
        } catch (Exception e){
            throw new EdcException("Error parsing byte to CreateProcessResponse", e);
        }
    }
}
