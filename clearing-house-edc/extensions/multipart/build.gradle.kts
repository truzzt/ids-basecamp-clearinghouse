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

plugins {
    `java-library`
}

dependencies {
    implementation(edc.util)
    implementation(edc.core.connector)
    implementation(edc.sql.core)
    implementation(edc.spi.core)
}
