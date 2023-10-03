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

package de.truzzt.clearinghouse.edc.dto;

import de.truzzt.clearinghouse.edc.types.ids.Message;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Objects;

public class HandlerResponse {

    private final Message header;
    private final Object payload;

    private HandlerResponse(@NotNull Message header, @Nullable Object payload) {
        this.header = header;
        this.payload = payload;
    }

    @NotNull
    public Message getHeader() {
        return header;
    }

    @Nullable
    public Object getPayload() {
        return payload;
    }

    public static class Builder {

        private Message header;
        private Object payload;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder header(@Nullable Message header) {
            this.header = header;
            return this;
        }

        public Builder payload(@Nullable Object payload) {
            this.payload = payload;
            return this;
        }

        public HandlerResponse build() {
            Objects.requireNonNull(header, "Multipart response header is null.");
            return new HandlerResponse(header, payload);
        }
    }
}
