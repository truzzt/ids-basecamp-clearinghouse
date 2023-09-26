package de.truzzt.clearinghouse.edc.multipart.handler;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.multipart.types.jwt.JwtPayload;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;
import org.jetbrains.annotations.NotNull;

import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.badParameters;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.createMultipartResponse;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.messageProcessedNotification;

public class LogHandler implements Handler {

    private final Monitor monitor;
    private final IdsId connectorId;
    private final TypeManagerUtil typeManagerUtil;

    public LogHandler(Monitor monitor, IdsId connectorId, TypeManagerUtil typeManagerUtil) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
    }

    @Override
    public boolean canHandle(@NotNull MultipartRequest multipartRequest) {
        return multipartRequest.getHeader().getType().equals("ids:LogMessage");
    }

    @Override
    public @NotNull MultipartResponse handleRequest(@NotNull MultipartRequest multipartRequest) {

        var header = multipartRequest.getHeader();

        JwtPayload jwt;
        try {
            jwt = typeManagerUtil.parseToken(header.getSecurityToken());
        } catch (EdcException e) {
            monitor.severe("LogMessage: Security Token is invalid", e);
            return createMultipartResponse(badParameters(header, connectorId));
        }


        return createMultipartResponse(messageProcessedNotification(header, connectorId));
    }
}
