package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import de.fraunhofer.iais.eis.DynamicAttributeToken;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.jetbrains.annotations.NotNull;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.Date;

import static de.truzzt.clearinghouse.edc.util.SettingsConstants.*;

public abstract class AbstractHandler {
    protected Date convertLocalDateTime(LocalDateTime localDateTime) {
        return Date.from(localDateTime.atZone(ZoneId.systemDefault()).toInstant());
    }

    protected @NotNull String buildJWTToken(@NotNull DynamicAttributeToken securityToken, ServiceExtensionContext context) {

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
