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
package de.truzzt.clearinghouse.edc.multipart.dto;

import de.truzzt.clearinghouse.edc.types.Pagging;
import jakarta.ws.rs.core.Response;
import org.jetbrains.annotations.NotNull;

public class PaggingValidationResponse {

    private Response error;
    private Pagging pagging;

    public PaggingValidationResponse(@NotNull Response error) {
        this.error = error;
    }
    public PaggingValidationResponse(@NotNull Pagging pagging) {
        this.pagging = pagging;
    }

    public Response getError() {
        return error;
    }

    public Pagging getPagging() {
        return pagging;
    }

    public Boolean fail() {
        return (error != null);
    }

}
