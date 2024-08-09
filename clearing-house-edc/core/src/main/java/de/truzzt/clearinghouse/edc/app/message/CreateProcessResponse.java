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
package de.truzzt.clearinghouse.edc.app.message;

import com.fasterxml.jackson.annotation.JsonProperty;

public class CreateProcessResponse extends AbstractResponse {

    @JsonProperty("pid")
    private String pid;

    public CreateProcessResponse() {
    }
    public CreateProcessResponse(String pid) {
        this.pid = pid;
    }
    public CreateProcessResponse(int httpStatus) {
        super(httpStatus);
    }

    public String getPid() {
        return pid;
    }

}