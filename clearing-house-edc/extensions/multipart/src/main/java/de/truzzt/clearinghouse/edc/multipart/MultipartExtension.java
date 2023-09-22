package de.truzzt.clearinghouse.edc.multipart;

import de.truzzt.clearinghouse.edc.multipart.controller.MultipartController;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import org.eclipse.edc.connector.api.management.configuration.ManagementApiConfiguration;
import org.eclipse.edc.protocol.ids.jsonld.JsonLd;
import org.eclipse.edc.runtime.metamodel.annotation.Extension;
import org.eclipse.edc.runtime.metamodel.annotation.Inject;
import org.eclipse.edc.runtime.metamodel.annotation.Requires;
import org.eclipse.edc.runtime.metamodel.annotation.Setting;
import org.eclipse.edc.spi.system.ServiceExtension;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.eclipse.edc.web.spi.WebService;

import static org.eclipse.edc.protocol.ids.util.ConnectorIdUtil.resolveConnectorId;

@Extension(value = MultipartExtension.NAME)
@Requires(value = {
        WebService.class,
        ManagementApiConfiguration.class
})
public class MultipartExtension implements ServiceExtension {

    @Setting
    public static final String EDC_IDS_ID = "edc.ids.id";
    public static final String DEFAULT_EDC_IDS_ID = "urn:connector:edc";

    public static final String NAME = "Clearing House Multipart Extension";

    @Inject
    private WebService webService;

    @Inject
    private ManagementApiConfiguration managementApiConfig;

    @Override
    public String name() {
        return NAME;
    }

    @Override
    public void initialize(ServiceExtensionContext context) {
        var connectorId = resolveConnectorId(context);
        var typeManagerUtil = new TypeManagerUtil(JsonLd.getObjectMapper());

        var multipartController = new MultipartController(connectorId, typeManagerUtil);
        webService.registerResource(managementApiConfig.getContextAlias(), multipartController);
    }

}
