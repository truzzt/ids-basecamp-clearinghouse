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
package de.truzzt.clearinghouse.edc.types;

import de.fraunhofer.iais.eis.Message;

import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartRequest;
import org.jetbrains.annotations.NotNull;

import java.util.Objects;

public class HandlerRequest extends MultipartRequest {

    private final String pid;
    private final Paging paging;

    private HandlerRequest(@NotNull String pid, @NotNull Message header, String payload, Paging paging) {
        super(header, payload, null);
        this.pid = pid;
        this.paging = paging;
    }

    public String getPid() {
        return pid;
    }

    public Paging getPaging() {
        return paging;
    }

    public static class Builder {

        private String pid;
        private Message header;
        private String payload;
        private Paging paging;

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

        public Builder payload(String payload) {
            this.payload = payload;
            return this;
        }

        public Builder paging(Paging paging) {
            this.paging = paging;
            return this;
        }

        public HandlerRequest build() {
            Objects.requireNonNull(pid, "Multipart request pid is null.");
            Objects.requireNonNull(header, "Multipart request header is null.");

            return new HandlerRequest(pid, header, payload, paging);
        }
    }
}
