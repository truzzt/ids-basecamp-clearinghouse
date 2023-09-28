package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import de.truzzt.clearinghouse.edc.types.ids.TokenFormat;
import org.eclipse.edc.spi.EdcException;
import org.jetbrains.annotations.NotNull;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.Date;

public interface Handler {

    boolean canHandle(@NotNull HandlerRequest handlerRequest);

    @NotNull HandlerResponse handleRequest(@NotNull HandlerRequest handlerRequest);

    default Date convertLocalDateTime(LocalDateTime localDateTime) {
        return Date.from(localDateTime.atZone(ZoneId.systemDefault()).toInstant());
    }

    default @NotNull String buildJWTToken(@NotNull SecurityToken securityToken) {

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
        var expiresAt = issuedAt.plusSeconds(60); // TODO Move to a configuration

        var jwtToken = JWT.create()
                .withAudience("1") // TODO Move to a configuration
                .withIssuer("1") // TODO Move to a configuration
                .withClaim("client_id", subject)
                .withIssuedAt(convertLocalDateTime(issuedAt))
                .withExpiresAt(convertLocalDateTime(expiresAt));

        return jwtToken.sign(Algorithm.HMAC256("123")); // TODO Move to a configuration
    }
}
