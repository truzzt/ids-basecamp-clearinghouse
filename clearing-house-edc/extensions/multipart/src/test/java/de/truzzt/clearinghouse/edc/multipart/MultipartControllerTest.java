package de.truzzt.clearinghouse.edc.multipart;

import de.truzzt.clearinghouse.edc.handler.Handler;
import de.truzzt.clearinghouse.edc.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import de.truzzt.clearinghouse.edc.types.ids.RejectionMessage;
import jakarta.ws.rs.core.Response;
import org.eclipse.edc.protocol.ids.spi.service.DynamicAttributeTokenService;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.ByteArrayInputStream;
import java.util.List;
import java.util.UUID;

import static org.mockito.Mockito.mock;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;

public class MultipartControllerTest {

    private static final String IDS_WEBHOOK_ADDRESSS = "http://localhost/callback";
    private static final String TEST_PAYLOAD = "Hello World";

    private MultipartController controller;
    private Monitor monitor;
    private IdsId connectorId;
    private TypeManagerUtil typeManagerUtil;
    private DynamicAttributeTokenService tokenService;
    private LogMessageHandler logMessageHandler = mock(LogMessageHandler.class);

    @BeforeEach
    public void setUp() {
        monitor = mock(Monitor.class);
        connectorId = mock(IdsId.class);
        typeManagerUtil = mock(TypeManagerUtil.class);
        tokenService = mock(DynamicAttributeTokenService.class);
        logMessageHandler = mock(LogMessageHandler.class);

        List<Handler> multipartHandlers = List.of(logMessageHandler);
        controller = new MultipartController(monitor, connectorId, typeManagerUtil, tokenService, IDS_WEBHOOK_ADDRESSS, multipartHandlers);
    }

    @Test
    public void missingHeaderError() {
        var pid = UUID.randomUUID().toString();

        var response = controller.request(pid, null, TEST_PAYLOAD);
        assertNotNull(response);
        assertEquals(Response.Status.BAD_REQUEST.getStatusCode(), response.getStatus());

        assertInstanceOf(FormDataMultiPart.class, response.getEntity());
        FormDataMultiPart multiPartResponse = (FormDataMultiPart) response.getEntity();
        // TODO Find a way to get the FormDataMultiPart header value
    }

}
