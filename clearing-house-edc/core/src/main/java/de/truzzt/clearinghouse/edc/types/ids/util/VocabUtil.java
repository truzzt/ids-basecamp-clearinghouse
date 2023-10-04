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
package de.truzzt.clearinghouse.edc.types.ids.util;

import java.net.MalformedURLException;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.UUID;

public class VocabUtil {
    public static String randomUrlBase;

    public static URI createRandomUrl(String path) {
        try {
            if (randomUrlBase != null) {
                if (!randomUrlBase.endsWith("/")) {
                    randomUrlBase = randomUrlBase + "/";
                }

                return (new URL(randomUrlBase + path + "/" + UUID.randomUUID())).toURI();
            } else {
                return (new URL("https", "w3id.org", "/idsa/autogen/" + path + "/" + UUID.randomUUID())).toURI();
            }
        } catch (URISyntaxException | MalformedURLException var3) {
            throw new RuntimeException(var3);
        }
    }
}
