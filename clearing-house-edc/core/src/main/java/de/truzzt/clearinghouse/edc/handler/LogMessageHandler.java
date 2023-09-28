package de.truzzt.clearinghouse.edc.handler;

import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.jetbrains.annotations.NotNull;

import static de.truzzt.clearinghouse.edc.util.ResponseUtil.createMultipartResponse;
import static de.truzzt.clearinghouse.edc.util.ResponseUtil.messageProcessedNotification;

public class LogMessageHandler implements Handler {

    private final Monitor monitor;
    private final IdsId connectorId;
    private final TypeManagerUtil typeManagerUtil;
    private final AppSender appSender;
    private final LoggingMessageDelegate senderDelegate;

    public LogMessageHandler(Monitor monitor,
                             IdsId connectorId,
                             TypeManagerUtil typeManagerUtil,
                             AppSender appSender) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
        this.appSender = appSender;

        this.senderDelegate = new LoggingMessageDelegate(typeManagerUtil);
    }

    @Override
    public boolean canHandle(@NotNull HandlerRequest handlerRequest) {
        return handlerRequest.getHeader().getType().equals("ids:LogMessage");
    }

    @Override
    public @NotNull HandlerResponse handleRequest(@NotNull HandlerRequest handlerRequest) {
        var baseUrl = "http://localhost:8000"; // TODO Move to a configuration
        var header = handlerRequest.getHeader();

        var url = senderDelegate.buildRequestUrl(baseUrl, handlerRequest);
        var token = buildJWTToken(handlerRequest.getHeader().getSecurityToken());
        var body = senderDelegate.buildRequestBody(handlerRequest);

        var request = AppSenderRequest.Builder.newInstance().url(url).token(token).body(body).build();

        var response = appSender.send(request, senderDelegate);
        return createMultipartResponse(messageProcessedNotification(header, connectorId), response);
    }
}
