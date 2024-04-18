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
package de.truzzt.clearinghouse.edc.app.types;

import com.fasterxml.jackson.annotation.JsonProperty;
import de.fraunhofer.iais.eis.*;
import org.eclipse.edc.spi.EdcException;
import org.jetbrains.annotations.NotNull;

import java.net.URI;
import java.util.Objects;

public class SecurityToken {

    @JsonProperty("@type")
    @NotNull
    private final String type;

    @JsonProperty("@id")
    @NotNull
    private final String id;

    @JsonProperty("tokenValue")
    @NotNull
    private final String tokenValue;

    private SecurityToken(@NotNull String type,
                         @NotNull String id,
                         @NotNull String tokenValue) {
        this.type = type;
        this.id = id;
        this.tokenValue = tokenValue;
    }

    public String getType() {
        return type;
    }

    public String getId() {
        return id;
    }

    public String getTokenValue() {
        return tokenValue;
    }

    public static class Builder {

        private String type;
        private String id;
        private String tokenValue;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder type(@NotNull Token token) {
            if (token instanceof DynamicAttributeTokenImpl)
                this.type = Constants.IDS_TYPE_PREFIX + DynamicAttributeToken.class.getSimpleName();
            else
                throw new EdcException("Unsupported Token Type:" + token.getClass().getSimpleName());

            return this;
        }

        public Builder id(@NotNull URI id) {
            this.id = id.toString();
            return this;
        }

        public Builder tokenValue(@NotNull String tokenValue) {
            this.tokenValue = tokenValue;
            return this;
        }

        public SecurityToken build() {
            Objects.requireNonNull(type, "Security token type is null.");
            Objects.requireNonNull(id, "Security token id is null.");
            Objects.requireNonNull(tokenValue, "Security token tokenValue is null.");

            return new SecurityToken(type, id, tokenValue);
        }
    }
}
