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
package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.jetbrains.annotations.NotNull;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.Date;

import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_AUDIENCE_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_AUDIENCE_SETTING;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_EXPIRES_AT_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_EXPIRES_AT_SETTING;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_ISSUER_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_ISSUER_SETTING;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_SIGN_SECRET_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_SIGN_SECRET_SETTING;

public interface Handler {

    boolean canHandle(@NotNull HandlerRequest handlerRequest);

    @NotNull HandlerResponse handleRequest(@NotNull HandlerRequest handlerRequest);

    default Date convertLocalDateTime(LocalDateTime localDateTime) {
        return Date.from(localDateTime.atZone(ZoneId.systemDefault()).toInstant());
    }

    default @NotNull String buildJWTToken(@NotNull SecurityToken securityToken, ServiceExtensionContext context) {

        var tokenValue = securityToken.getTokenValue();
        var decodedToken = JWT.decode(tokenValue);

        var subject = decodedToken.getSubject();
        if (subject == null) {
            throw new EdcException("JWT Token subject is missing");
        }

        var issuedAt = LocalDateTime.now();
        var expiresAt = issuedAt.plusSeconds(
                Long.parseLong(context.getSetting(JWT_EXPIRES_AT_SETTING ,JWT_EXPIRES_AT_DEFAULT_VALUE)));

        var jwtToken = JWT.create()
                .withAudience(context.getSetting(JWT_AUDIENCE_SETTING, JWT_AUDIENCE_DEFAULT_VALUE))
                .withIssuer(context.getSetting(JWT_ISSUER_SETTING, JWT_ISSUER_DEFAULT_VALUE))
                .withClaim("client_id", subject)
                .withIssuedAt(convertLocalDateTime(issuedAt))
                .withExpiresAt(convertLocalDateTime(expiresAt));

        return jwtToken.sign(Algorithm.HMAC256(context.getSetting(JWT_SIGN_SECRET_SETTING ,JWT_SIGN_SECRET_DEFAULT_VALUE)));
    }
}
