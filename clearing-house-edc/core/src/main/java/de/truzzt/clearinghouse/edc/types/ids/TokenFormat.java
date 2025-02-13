/*
 *  Copyright (c) 2023 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       Microsoft Corporation - Initial implementation
 *       truzzt GmbH - EDC extension implementation
 *
 */
package de.truzzt.clearinghouse.edc.types.ids;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

import java.net.URI;

public class TokenFormat {

    public static final String JWT_TOKEN_FORMAT_IDS = "idsc:JWT";
    public static final String JWT_TOKEN_FORMAT_DSP = "https://w3id.org/idsa/code/JWT";

    @JsonProperty("@id")
    @NotNull
    private URI id;

    public URI getId() {
        return id;
    }

    public static boolean isValid(String id) {
        return id.equals(JWT_TOKEN_FORMAT_IDS) || id.equals(JWT_TOKEN_FORMAT_DSP);
    }
}
