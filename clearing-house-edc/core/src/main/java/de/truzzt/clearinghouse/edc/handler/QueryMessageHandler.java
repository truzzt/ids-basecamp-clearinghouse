package de.truzzt.clearinghouse.edc.handler;

import de.fraunhofer.iais.eis.QueryMessageImpl;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.QueryMessageDelegate;
import de.truzzt.clearinghouse.edc.app.message.AppSenderRequest;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import org.eclipse.edc.protocol.ids.api.multipart.handler.Handler;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartRequest;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartResponse;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.jetbrains.annotations.NotNull;

import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_SETTING;
import static org.eclipse.edc.protocol.ids.api.multipart.util.ResponseUtil.createMultipartResponse;
import static org.eclipse.edc.protocol.ids.api.multipart.util.ResponseUtil.messageProcessedNotification;

public class QueryMessageHandler extends AbstractHandler implements Handler {

    private final IdsId connectorId;
    private final AppSender appSender;
    private final QueryMessageDelegate senderDelegate;

    private final ServiceExtensionContext context;

    public QueryMessageHandler(IdsId connectorId,
                             AppSender appSender,
                             ServiceExtensionContext context) {
        this.connectorId = connectorId;
        this.appSender = appSender;
        this.context = context;

        this.senderDelegate = new QueryMessageDelegate();
    }

    @Override
    public boolean canHandle(@NotNull MultipartRequest multipartRequest) {
        return multipartRequest.getHeader().getClass().equals(QueryMessageImpl.class);
    }

    @Override
    public @NotNull MultipartResponse handleRequest(@NotNull MultipartRequest multipartRequest) {
        var handlerRequest = (HandlerRequest) multipartRequest;
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
