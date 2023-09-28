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

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

import java.net.URI;

public class RejectionMessage extends Message {

    @JsonProperty("ids:rejectionReason")
    @NotNull
    RejectionReason rejectionReason;

    public RejectionMessage() {
    }

    public RejectionMessage(@NotNull URI id) {
        super(id);
    }

    public RejectionReason getRejectionReason() {
        return rejectionReason;
    }

    public void setRejectionReason(@NotNull RejectionReason rejectionReason) {
        this.rejectionReason = rejectionReason;
    }
}
