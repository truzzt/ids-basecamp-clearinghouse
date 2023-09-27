package de.truzzt.clearinghouse.edc.multipart.handler;

import com.auth0.jwt.JWT;

import com.auth0.jwt.algorithms.Algorithm;
import de.truzzt.clearinghouse.edc.multipart.message.ClearingHouseAppRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.sender.ClearingHouseAppSender;
import de.truzzt.clearinghouse.edc.multipart.sender.delegate.LoggingMessageSenderDelegate;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;

import de.truzzt.clearinghouse.edc.multipart.util.JWTUtil;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.jetbrains.annotations.NotNull;

import java.time.LocalDateTime;

import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.createMultipartResponse;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.messageProcessedNotification;

public class LogMessageHandler implements Handler {

    private final Monitor monitor;
    private final IdsId connectorId;
    private final ClearingHouseAppSender clearingHouseAppSender;
    private final TypeManagerUtil typeManagerUtil;
    private final LoggingMessageSenderDelegate senderDelegate;

    public LogMessageHandler(Monitor monitor,
                             IdsId connectorId,
                             TypeManagerUtil typeManagerUtil,
                             ClearingHouseAppSender clearingHouseAppSender) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
        this.clearingHouseAppSender = clearingHouseAppSender;

        this.senderDelegate = new LoggingMessageSenderDelegate(typeManagerUtil);
    }

    @Override
    public boolean canHandle(@NotNull MultipartRequest multipartRequest) {
        return multipartRequest.getHeader().getType().equals("ids:LogMessage");
    }

    @Override
    public @NotNull MultipartResponse handleRequest(@NotNull MultipartRequest multipartRequest) {
        var header = multipartRequest.getHeader();

        var baseUrl = "http://localhost:8000"; // TODO Move to a configuration
        var url = senderDelegate.buildRequestUrl(baseUrl, multipartRequest);

        var tokenValue = multipartRequest.getHeader().getSecurityToken().getTokenValue();

        // TODO Move to a shared class
        // TODO Validate if tokenFormat is JWT

        var decodedDat = JWT.decode(tokenValue);
        var claimedClientId = decodedDat.getSubject();

        // TODO Validate if token subject is null

        var issuedAt = LocalDateTime.now();
        var expiresAt = issuedAt.plusSeconds(60); // Config

        var jwtToken = JWT.create()
                .withAudience("1") // TODO Move to a configuration
                .withIssuer("1") // TODO Move to a configuration
                .withClaim("client_id", claimedClientId)
                .withIssuedAt(JWTUtil.convertLocalDateTime(issuedAt))
                .withExpiresAt(JWTUtil.convertLocalDateTime(expiresAt))
                .sign(Algorithm.HMAC256("123")); // TODO Move to a configuration

        var body = senderDelegate.buildRequestBody(multipartRequest);

        var request = ClearingHouseAppRequest.Builder.newInstance().url(url).token(jwtToken).body(body).build();

        var response = clearingHouseAppSender.send(request, senderDelegate);

        return createMultipartResponse(messageProcessedNotification(header, connectorId), response);
    }
}
