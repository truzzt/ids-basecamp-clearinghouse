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

package de.truzzt.clearinghouse.edc.types;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.eclipse.edc.spi.EdcException;

import java.io.IOException;
import java.io.InputStream;

public class TypeManagerUtil {

    private final ObjectMapper mapper;

    public TypeManagerUtil(ObjectMapper mapper) {
        this.mapper = mapper;
    }

    public <T> T parse(InputStream inputStream, Class<T> type) {
        try {
            return mapper.readValue(inputStream, type);
        } catch (IOException e) {
            throw new EdcException("Error parsing to type " + type, e);
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
