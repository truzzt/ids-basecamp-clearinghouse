/*
 *  Copyright (c) 2021 Microsoft Corporation
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

package de.truzzt.clearinghouse.edc.multipart.types;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.multipart.types.ids.DynamicAttributeToken;
import de.truzzt.clearinghouse.edc.multipart.types.ids.LogMessage;
import de.truzzt.clearinghouse.edc.multipart.types.jwt.JwtPayload;
import org.eclipse.edc.spi.EdcException;

import java.io.IOException;
import java.io.InputStream;
import java.util.Base64;

public class TypeManagerUtil {

    private final ObjectMapper mapper;

    public TypeManagerUtil(ObjectMapper mapper) {
        this.mapper = mapper;
    }

    public LogMessage parseMessage(InputStream streamToken) {
        try {
            return mapper.readValue(streamToken, LogMessage.class);
        } catch (IOException e) {
            throw new EdcException("Error parsing Header to Message", e);
        }
    }

    public JwtPayload parseToken(DynamicAttributeToken token) {
        try {
            Base64.Decoder decoder = Base64.getUrlDecoder();
            String[] chunks = token.getTokenValue().split("\\.");
            return mapper.readValue(decoder.decode(chunks[1]), JwtPayload.class);

        } catch (IOException e) {
            throw new EdcException("Error parsing Token", e);
        }
    }

    public byte[] toJson(Object object) {
        try {
            return mapper.writeValueAsBytes(object);
        } catch (JsonProcessingException e) {
            throw new EdcException("Error converting to JSON", e);
        }
    }
}
