/*
 *  Copyright (c) 2023 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       Microsoft Corporation - Initial implementation
 *       truzzt GmbH - EDC extension implementation
 *
 */
package de.truzzt.clearinghouse.edc.multipart;

import de.truzzt.clearinghouse.edc.handler.Handler;
import de.truzzt.clearinghouse.edc.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import org.eclipse.edc.connector.api.management.configuration.ManagementApiConfiguration;
import org.eclipse.edc.protocol.ids.api.configuration.IdsApiConfiguration;
import org.eclipse.edc.protocol.ids.jsonld.JsonLd;
import org.eclipse.edc.protocol.ids.spi.service.DynamicAttributeTokenService;
import org.eclipse.edc.runtime.metamodel.annotation.Extension;
import org.eclipse.edc.runtime.metamodel.annotation.Inject;
import org.eclipse.edc.runtime.metamodel.annotation.Requires;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.system.ServiceExtension;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.eclipse.edc.web.spi.WebService;

import java.util.LinkedList;

import static org.eclipse.edc.protocol.ids.util.ConnectorIdUtil.resolveConnectorId;

@Extension(value = MultipartExtension.NAME)
@Requires(value = {
        WebService.class,
        ManagementApiConfiguration.class,
        EdcHttpClient.class
})
public class MultipartExtension implements ServiceExtension {

    public static final String NAME = "Clearing House Multipart Extension";

    @Inject
    private WebService webService;

    @Inject
    private ManagementApiConfiguration managementApiConfig;

    @Inject
    private EdcHttpClient httpClient;

    @Inject
    private DynamicAttributeTokenService tokenService;

    @Inject
    private IdsApiConfiguration idsApiConfiguration;

    @Override
    public String name() {
        return NAME;
    }

    @Override
    public void initialize(ServiceExtensionContext context) {
        var monitor = context.getMonitor();
        var connectorId = resolveConnectorId(context);
        var typeManagerUtil = new TypeManagerUtil(JsonLd.getObjectMapper());

        var clearingHouseAppSender = new AppSender(monitor, httpClient, typeManagerUtil);

        var handlers = new LinkedList<Handler>();
        handlers.add(new LogMessageHandler(monitor, connectorId, typeManagerUtil, clearingHouseAppSender, context));

        var multipartController = new MultipartController(monitor,
                connectorId,
                typeManagerUtil,
                tokenService,
                idsApiConfiguration.getIdsWebhookAddress(),
                handlers);
        webService.registerResource(managementApiConfig.getContextAlias(), multipartController);
    }
}
