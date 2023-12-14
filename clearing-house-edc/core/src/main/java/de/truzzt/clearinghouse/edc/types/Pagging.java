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

import org.jetbrains.annotations.NotNull;

import java.time.LocalDate;

public class Pagging {

    private final Integer page;
    private final Integer size;
    private final Sort sort;
    private final LocalDate dateFrom;
    private final LocalDate dateTo;

    public enum Sort {
        ASC, DESC
    }

    private Pagging(@NotNull Integer page, Integer size, Sort sort, LocalDate dateFrom, LocalDate dateTo) {
        this.page = page;
        this.size = size;
        this.sort = sort;
        this.dateFrom = dateFrom;
        this.dateTo = dateTo;
    }

    public Integer getPage() {
        return page;
    }

    public Integer getSize() {
        return size;
    }

    public Sort getSort() {
        return sort;
    }

    public LocalDate getDateFrom() {
        return dateFrom;
    }

    public LocalDate getDateTo() {
        return dateTo;
    }


    public static class Builder {

        private Integer page;
        private Integer size;
        private Sort sort;
        private LocalDate dateFrom;
        private LocalDate dateTo;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder page(@NotNull Integer page) {
            this.page = page;
            return this;
        }

        public Builder size(@NotNull Integer size) {
            this.size = size;
            return this;
        }

        public Builder sort(@NotNull Sort sort) {
            this.sort = sort;
            return this;
        }

        public Builder dateFrom(@NotNull LocalDate dateFrom) {
            this.dateFrom = dateFrom;
            return this;
        }

        public Builder dateTo(@NotNull LocalDate dateTo) {
            this.dateTo = dateTo;
            return this;
        }

        public Pagging build() {
            return new Pagging(page, size, sort, dateFrom, dateTo);
        }
    }
}
