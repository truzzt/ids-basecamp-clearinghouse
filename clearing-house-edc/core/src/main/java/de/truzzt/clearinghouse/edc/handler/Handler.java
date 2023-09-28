package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import de.truzzt.clearinghouse.edc.types.ids.TokenFormat;
import org.eclipse.edc.runtime.metamodel.annotation.Setting;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.jetbrains.annotations.NotNull;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.Date;

public interface Handler {

    @Setting
    String JWT_AUDIENCE = "edc.truzzt.jwt.audience";

    @Setting
    String JWT_ISSUER = "edc.truzzt.jwt.issuer";

    @Setting
    String JWT_SIGN_SECRET = "edc.truzzt.jwt.sign.secret";

    @Setting
    String JWT_EXPIRES_AT  = "edc.truzzt.jwt.expires.at";


    boolean canHandle(@NotNull HandlerRequest handlerRequest);

    @NotNull HandlerResponse handleRequest(@NotNull HandlerRequest handlerRequest);

    default Date convertLocalDateTime(LocalDateTime localDateTime) {
        return Date.from(localDateTime.atZone(ZoneId.systemDefault()).toInstant());
    }

    default @NotNull String buildJWTToken(@NotNull SecurityToken securityToken, ServiceExtensionContext context) {

        var tokenFormat = securityToken.getTokenFormat().getId().toString();
        if (!tokenFormat.equals(TokenFormat.JWT_TOKEN_FORMAT)) {
            throw new EdcException("Invalid security token format: " + securityToken.getTokenFormat().getId());
        }

        var tokenValue = securityToken.getTokenValue();
        var decodedToken = JWT.decode(tokenValue);

        var subject = decodedToken.getSubject();
        if (subject == null) {
            throw new EdcException("JWT Token subject is missing");
        }

        var issuedAt = LocalDateTime.now();
        var expiresAt = issuedAt.plusSeconds(
                Long.valueOf(context.getSetting(JWT_EXPIRES_AT,JWT_EXPIRES_AT)));

        var jwtToken = JWT.create()
                .withAudience(context.getSetting(JWT_AUDIENCE, JWT_AUDIENCE))
                .withIssuer(context.getSetting(JWT_ISSUER, JWT_ISSUER))
                .withClaim("client_id", subject)
                .withIssuedAt(convertLocalDateTime(issuedAt))
                .withExpiresAt(convertLocalDateTime(expiresAt));

        return jwtToken.sign(Algorithm.HMAC256(context.getSetting(JWT_SIGN_SECRET,JWT_SIGN_SECRET)));
    }
}
