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
package de.truzzt.clearinghouse.edc.app.message;

import com.fasterxml.jackson.annotation.JsonProperty;
import de.truzzt.clearinghouse.edc.app.types.Header;
import org.jetbrains.annotations.NotNull;

public class CreateProcessRequest {

    @JsonProperty("header")
    @NotNull
    private final Header header;

    @JsonProperty("payload")
    @NotNull
    private final String payload;

    public CreateProcessRequest(@NotNull Header header, @NotNull String payload) {
        this.header = header;
        this.payload = payload;
    }
}

