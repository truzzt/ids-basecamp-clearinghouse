package de.truzzt.clearinghouse.edc.multipart;

import org.eclipse.edc.connector.api.management.configuration.ManagementApiConfiguration;
import org.eclipse.edc.runtime.metamodel.annotation.Extension;
import org.eclipse.edc.runtime.metamodel.annotation.Inject;
import org.eclipse.edc.runtime.metamodel.annotation.Requires;
import org.eclipse.edc.spi.system.ServiceExtension;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.eclipse.edc.web.spi.WebService;

@Extension(value = MultipartExtension.NAME)
@Requires(value = {
        WebService.class,
        ManagementApiConfiguration.class
})
public class MultipartExtension implements ServiceExtension {

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
        var multipartController = new MultipartController();
        webService.registerResource(managementApiConfig.getContextAlias(), multipartController);
    }
}
