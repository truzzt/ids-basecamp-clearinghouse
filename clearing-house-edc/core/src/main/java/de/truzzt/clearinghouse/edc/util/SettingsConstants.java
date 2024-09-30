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
 *       truzzt GmbH - initial implementation
 *
 */
package de.truzzt.clearinghouse.edc.util;

public class SettingsConstants {

    public static final String JWT_AUDIENCE_SETTING = "truzzt.clearinghouse.jwt.audience";
    public static final String JWT_AUDIENCE_DEFAULT_VALUE = "1";

    public static final String JWT_ISSUER_SETTING = "truzzt.clearinghouse.jwt.issuer";
    public static final String JWT_ISSUER_DEFAULT_VALUE = "1";

    public static final String JWT_SIGN_SECRET_SETTING = "truzzt.clearinghouse.jwt.sign.secret";
    public static final String JWT_SIGN_SECRET_DEFAULT_VALUE = "123";

    public static final String JWT_EXPIRES_AT_SETTING  = "truzzt.clearinghouse.jwt.expires.at";
    public static final String JWT_EXPIRES_AT_DEFAULT_VALUE  = "30";

    public static final String APP_BASE_URL_SETTING = "truzzt.clearinghouse.app.base.url";
    public static final String APP_BASE_URL_DEFAULT_VALUE = "http://localhost:8000";

}
