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

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

public class CreateProcessResponse {

    @JsonProperty("pid")
    @NotNull
    private String pid;

    public String getPid() {
        return pid;
    }

}