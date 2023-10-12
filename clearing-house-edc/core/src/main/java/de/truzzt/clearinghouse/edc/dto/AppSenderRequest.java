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
package de.truzzt.clearinghouse.edc.dto;

import org.jetbrains.annotations.NotNull;

import java.util.Objects;

public class AppSenderRequest<B> {

    private final String url;
    private final String token;
    private final B body;

    public AppSenderRequest(@NotNull  String url, @NotNull String token, @NotNull B body) {
        this.url = url;
        this.token = token;
        this.body = body;
    }

    public String getUrl() {
        return url;
    }

    public String getToken() {
        return token;
    }

    public B getBody() {
        return body;
    }

    public static class Builder<R> {

        private String url;
        private String token;
        private R body;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder url(@NotNull String url) {
            this.url = url;
            return this;
        }

        public Builder token(@NotNull String token) {
            this.token = token;
            return this;
        }

        public Builder body(@NotNull R body) {
            this.body = body;
            return this;
        }

        public AppSenderRequest build() {
            Objects.requireNonNull(url, "ClearingHouse request url is null.");
            Objects.requireNonNull(token, "ClearingHouse request token is null.");
            Objects.requireNonNull(body, "ClearingHouse request body is null.");

            return new AppSenderRequest(url, token, body);
        }
    }

}
