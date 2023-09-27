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

package de.truzzt.clearinghouse.edc.multipart.types.clearinghouse;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

import java.net.URI;
import java.util.Objects;

public class SecurityToken {

    @JsonProperty("@type")
    @NotNull
    private String type;

    @JsonProperty("@id")
    @NotNull
    private String id;

    @JsonProperty("tokenFormat")
    @NotNull
    private TokenFormat tokenFormat;

    @JsonProperty("tokenValue")
    @NotNull
    private String tokenValue;

    private SecurityToken(@NotNull String type,
                         @NotNull String id,
                         @NotNull TokenFormat tokenFormat,
                         @NotNull String tokenValue) {
        this.type = type;
        this.id = id;
        this.tokenFormat = tokenFormat;
        this.tokenValue = tokenValue;
    }

    public String getType() {
        return type;
    }

    public String getId() {
        return id;
    }

    public TokenFormat getTokenFormat() {
        return tokenFormat;
    }

    public String getTokenValue() {
        return tokenValue;
    }

    public static class Builder {

        private String type;
        private String id;
        private TokenFormat tokenFormat;
        private String tokenValue;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder type(@NotNull String type) {
            this.type = type;
            return this;
        }

        public Builder id(@NotNull URI id) {
            this.id = id.toString();
            return this;
        }

        public Builder tokenFormat(@NotNull TokenFormat tokenFormat) {
            this.tokenFormat = tokenFormat;
            return this;
        }

        public Builder tokenValue(@NotNull String tokenValue) {
            this.tokenValue = tokenValue;
            return this;
        }

        public SecurityToken build() {
            Objects.requireNonNull(type, "Security token type is null.");
            Objects.requireNonNull(id, "Security token id is null.");
            Objects.requireNonNull(tokenFormat, "Security token tokenFormat is null.");
            Objects.requireNonNull(tokenValue, "Security token tokenValue is null.");

            return new SecurityToken(type, id, tokenFormat, tokenValue);
        }
    }
}
