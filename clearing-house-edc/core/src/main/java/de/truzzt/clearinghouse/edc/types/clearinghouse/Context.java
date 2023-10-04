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
package de.truzzt.clearinghouse.edc.types.clearinghouse;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

public class Context {
    @JsonProperty("ids")
    @NotNull
    private final String ids;

    @JsonProperty("idsc")
    @NotNull
    private final String idsc;

    public Context(@NotNull String ids, @NotNull String idsc) {
        this.ids = ids;
        this.idsc = idsc;
    }

    public String getIds() {
        return ids;
    }

    public String getIdsc() {
        return idsc;
    }
}
