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
package de.truzzt.clearinghouse.edc.dto;

import de.truzzt.clearinghouse.edc.types.ids.Message;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Objects;

public class HandlerRequest {

    private final String pid;
    private final Message header;
    private final String payload;

    private HandlerRequest(String pid, @NotNull Message header, @Nullable String payload) {
        this.pid = pid;
        this.header = header;
        this.payload = payload;
    }

    public String getPid() {
        return pid;
    }

    public Message getHeader() {
        return header;
    }

    public String getPayload() {
        return payload;
    }

    public static class Builder {

        private String pid;
        private Message header;
        private String payload;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder pid(@NotNull String pid) {
            this.pid = pid;
            return this;
        }

        public Builder header(@NotNull Message header) {
            this.header = header;
            return this;
        }

        public Builder payload(@NotNull String payload) {
            this.payload = payload;
            return this;
        }

        public HandlerRequest build() {
            Objects.requireNonNull(header, "Multipart request header is null.");

            return new HandlerRequest(pid, header, payload);
        }
    }
}
