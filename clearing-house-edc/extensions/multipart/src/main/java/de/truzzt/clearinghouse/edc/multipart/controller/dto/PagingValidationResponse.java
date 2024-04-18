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
package de.truzzt.clearinghouse.edc.multipart.controller.dto;

import de.truzzt.clearinghouse.edc.types.Paging;
import jakarta.ws.rs.core.Response;
import org.jetbrains.annotations.NotNull;

public class PagingValidationResponse {

    private Response error;
    private Paging paging;

    public PagingValidationResponse(@NotNull Response error) {
        this.error = error;
    }
    public PagingValidationResponse(@NotNull Paging paging) {
        this.paging = paging;
    }

    public Response getError() {
        return error;
    }

    public Paging getPaging() {
        return paging;
    }

    public Boolean fail() {
        return (error != null);
    }

}
