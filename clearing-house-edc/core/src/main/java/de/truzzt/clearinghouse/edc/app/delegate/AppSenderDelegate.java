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
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;

import java.io.IOException;

public abstract class AppSenderDelegate<B> {

    protected final ObjectMapper objectMapper;

    protected AppSenderDelegate(ObjectMapper objectMapper) {
        this.objectMapper = objectMapper;
    }

    public abstract B buildSuccessResponse(ResponseBody responseBody);

    public abstract B buildErrorResponse(int httpStatus);

    protected B parseSuccessResponse(ResponseBody responseBody, Class<B> type) {
        try {
            return objectMapper.readValue(responseBody.byteStream(), type);
        } catch (IOException e){
            throw new EdcException("Error parsing response body to " + type.getName(), e);
        }
    }
}
