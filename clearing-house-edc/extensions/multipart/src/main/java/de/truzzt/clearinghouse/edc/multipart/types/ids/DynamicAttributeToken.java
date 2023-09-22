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

import com.fasterxml.jackson.annotation.JsonAlias;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonTypeName;
import de.truzzt.clearinghouse.edc.multipart.types.ids.util.VocabUtil;
import org.jetbrains.annotations.NotNull;

import java.net.URI;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "@type")
@JsonIgnoreProperties(ignoreUnknown = true)
@JsonTypeName("ids:DynamicAttributeToken")
public class DynamicAttributeToken {

    @JsonProperty("@id")
    @JsonAlias({"@id", "id"})
    @NotNull
    private URI id;

    @NotNull
    @JsonAlias({"ids:tokenFormat", "tokenFormat"})
    private TokenFormat tokenFormat;

    @NotNull
    @JsonAlias({"ids:tokenValue", "tokenValue"})
    private String tokenValue;

    private DynamicAttributeToken() {
        id = VocabUtil.createRandomUrl("dynamicAttributeToken");
    }

    @JsonProperty("@id")
    public URI getId() {
        return id;
    }

    public void setId(URI id) {
        this.id = id;
    }

    public TokenFormat getTokenFormat() {
        return tokenFormat;
    }

    public void setTokenFormat(TokenFormat tokenFormat) {
        this.tokenFormat = tokenFormat;
    }

    public String getTokenValue() {
        return tokenValue;
    }

    public void setTokenValue(String tokenValue) {
        this.tokenValue = tokenValue;
    }
}

