package de.truzzt.clearinghouse.edc.multipart;

import de.truzzt.clearinghouse.edc.multipart.handler.Handler;
import de.truzzt.clearinghouse.edc.multipart.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.multipart.handler.QueryMessageHandler;
import de.truzzt.clearinghouse.edc.multipart.handler.RequestMessageHandler;
import de.truzzt.clearinghouse.edc.multipart.sender.ClearingHouseAppSender;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import org.eclipse.edc.connector.api.management.configuration.ManagementApiConfiguration;
import org.eclipse.edc.protocol.ids.jsonld.JsonLd;
import org.eclipse.edc.runtime.metamodel.annotation.Extension;
import org.eclipse.edc.runtime.metamodel.annotation.Inject;
import org.eclipse.edc.runtime.metamodel.annotation.Requires;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.system.ServiceExtension;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.eclipse.edc.web.spi.WebService;

import java.util.LinkedList;

import static org.eclipse.edc.protocol.ids.util.ConnectorIdUtil.resolveConnectorId;

@Extension(value = ClearingHouseMultipartExtension.NAME)
@Requires(value = {
        WebService.class,
        ManagementApiConfiguration.class,
        EdcHttpClient.class
})
public class ClearingHouseMultipartExtension implements ServiceExtension {

    public static final String NAME = "Clearing House Multipart Extension";

    @Inject
    private WebService webService;

    @Inject
    private ManagementApiConfiguration managementApiConfig;

    @Inject
    private EdcHttpClient httpClient;

    @Override
    public String name() {
        return NAME;
    }

    @Override
    public void initialize(ServiceExtensionContext context) {
        var monitor = context.getMonitor();
        var connectorId = resolveConnectorId(context);
        var typeManagerUtil = new TypeManagerUtil(JsonLd.getObjectMapper());

        var clearingHouseAppSender = new ClearingHouseAppSender(monitor, httpClient, typeManagerUtil);

        var handlers = new LinkedList<Handler>();
        handlers.add(new RequestMessageHandler(monitor, connectorId, clearingHouseAppSender));
        handlers.add(new LogMessageHandler(monitor, connectorId, typeManagerUtil, clearingHouseAppSender));
        handlers.add(new QueryMessageHandler(monitor, connectorId, clearingHouseAppSender));

        var multipartController = new ClearingHouseMultipartController(monitor, connectorId, typeManagerUtil, handlers);
        webService.registerResource(managementApiConfig.getContextAlias(), multipartController);
    }

}