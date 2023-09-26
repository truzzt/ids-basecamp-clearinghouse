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

import java.net.URI;

public class RejectionMessage extends LogMessage {

    @JsonAlias({"https://w3id.org/idsa/core/rejectionReason", "ids:rejectionReason", "rejectionReason"})
    RejectionReason rejectionReason;

    public RejectionMessage() {
    }

    public RejectionMessage(URI id) {
        super(id);
    }

    public RejectionReason getRejectionReason() {
        return rejectionReason;
    }

    public void setRejectionReason(RejectionReason rejectionReason) {
        this.rejectionReason = rejectionReason;
    }
}
