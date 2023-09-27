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
 *       Microsoft Corporation - Initial implementation
 *
 */

package de.truzzt.clearinghouse.edc.multipart.types.ids;

import com.fasterxml.jackson.annotation.JsonProperty;
import de.truzzt.clearinghouse.edc.multipart.types.ids.util.VocabUtil;
import org.jetbrains.annotations.NotNull;

import java.net.URI;

public class SecurityToken {

    @JsonProperty("@type")
    @NotNull
    private String type;

    @JsonProperty("@id")
    @NotNull
    private URI id;

    @JsonProperty("ids:tokenFormat")
    @NotNull
    private TokenFormat tokenFormat;

    @JsonProperty("ids:tokenValue")
    @NotNull
    private String tokenValue;

    private SecurityToken() {
        id = VocabUtil.createRandomUrl("dynamicAttributeToken");
    }

    public String getType() {
        return type;
    }

    public URI getId() {
        return id;
    }

    public TokenFormat getTokenFormat() {
        return tokenFormat;
    }

    public String getTokenValue() {
        return tokenValue;
    }
}
