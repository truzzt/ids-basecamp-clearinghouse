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

package de.truzzt.clearinghouse.edc.types.ids;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;
import org.jetbrains.annotations.NotNull;

import java.net.URI;

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonTypeName("ids:RejectionReason")
public class RejectionReason {

    @JsonProperty("@id")
    @NotNull
    private URI id;

    public RejectionReason() {
    }

    public RejectionReason(@NotNull URI id) {
        this.id = id;
    }

    public static final RejectionReason BAD_PARAMETERS = new RejectionReason(URI.create("https://w3id.org/idsa/code/BAD_PARAMETERS"));

    public static final RejectionReason INTERNAL_RECIPIENT_ERROR =
            new RejectionReason(URI.create("https://w3id.org/idsa/code/INTERNAL_RECIPIENT_ERROR"));

    public static final RejectionReason MALFORMED_MESSAGE =
            new RejectionReason(URI.create("https://w3id.org/idsa/code/MALFORMED_MESSAGE"));

    public static final RejectionReason MESSAGE_TYPE_NOT_SUPPORTED =
            new RejectionReason(URI.create("https://w3id.org/idsa/code/MESSAGE_TYPE_NOT_SUPPORTED"));

    public static final RejectionReason NOT_AUTHENTICATED =
            new RejectionReason(URI.create("https://w3id.org/idsa/code/NOT_AUTHENTICATED"));

    public URI getId() {
        return id;
    }
}
