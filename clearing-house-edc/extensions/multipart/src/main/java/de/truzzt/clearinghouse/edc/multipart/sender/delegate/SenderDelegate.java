/*
 *  Copyright (c) 2022 sovity GmbH
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       sovity GmbH - initial API and implementation
 *
 */
package de.truzzt.clearinghouse.edc.multipart.sender.delegate;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import okhttp3.ResponseBody;

public interface SenderDelegate<R, P> {

    String buildRequestUrl(String baseUrl, MultipartRequest request);

    R buildRequestBody(MultipartRequest request);

    P parseResponseBody(ResponseBody responseBody);
}
