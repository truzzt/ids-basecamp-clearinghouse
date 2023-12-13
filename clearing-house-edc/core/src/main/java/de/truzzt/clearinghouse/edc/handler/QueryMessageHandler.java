package de.truzzt.clearinghouse.edc.handler;

import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.QueryMessageDelegate;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.jetbrains.annotations.NotNull;

import static de.truzzt.clearinghouse.edc.util.ResponseUtil.createMultipartResponse;
import static de.truzzt.clearinghouse.edc.util.ResponseUtil.messageProcessedNotification;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_SETTING;

public class QueryMessageHandler implements Handler{

    private final IdsId connectorId;
    private final AppSender appSender;
    private final QueryMessageDelegate senderDelegate;

    private final ServiceExtensionContext context;

    public QueryMessageHandler(IdsId connectorId,
                             TypeManagerUtil typeManagerUtil,
                             AppSender appSender,
                             ServiceExtensionContext context) {
        this.connectorId = connectorId;
        this.appSender = appSender;
        this.context = context;

        this.senderDelegate = new QueryMessageDelegate(typeManagerUtil);
    }
    @Override
    public boolean canHandle(@NotNull HandlerRequest handlerRequest) {
        return handlerRequest.getHeader().getType().equals("ids:QueryMessage");
    }

    @Override
    public @NotNull HandlerResponse handleRequest(@NotNull HandlerRequest handlerRequest) {
        var baseUrl = context.getSetting(APP_BASE_URL_SETTING, APP_BASE_URL_DEFAULT_VALUE);
        var header = handlerRequest.getHeader();

        var url = senderDelegate.buildRequestUrl(baseUrl, handlerRequest);
        var token = buildJWTToken(handlerRequest.getHeader().getSecurityToken(), context);
        var body = senderDelegate.buildRequestBody(handlerRequest);

        var request = AppSenderRequest.Builder.newInstance().url(url).token(token).body(body).build();

        var response = appSender.send(request, senderDelegate);
        return createMultipartResponse(messageProcessedNotification(header, connectorId), response);
    }
}
