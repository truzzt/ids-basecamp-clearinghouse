package de.truzzt.clearinghouse.edc.multipart.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.*;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.handler.RequestMessageHandler;
import de.truzzt.clearinghouse.edc.multipart.tests.TestUtils;
import jakarta.ws.rs.core.Response;
import org.eclipse.edc.protocol.ids.api.multipart.handler.Handler;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartResponse;
import org.eclipse.edc.protocol.ids.spi.service.DynamicAttributeTokenService;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.protocol.ids.spi.types.IdsType;
import org.eclipse.edc.spi.monitor.Monitor;
import org.eclipse.edc.spi.result.Result;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import java.io.IOException;
import java.net.URI;
import java.util.List;
import java.util.UUID;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doReturn;

public class MultipartControllerTest {

    private static final String IDS_WEBHOOK_ADDRESS = "http://localhost/callback";
    private static final String PAYLOAD = "Hello World";
    private static final String CREATE_PROCESS_PAYLOAD = "{ \"owners\": [\"1\", \"2\"]}";

    private MultipartController controller;

    private IdsId connectorId;

    @Mock
    private Monitor monitor;
    @Mock
    private DynamicAttributeTokenService tokenService;
    @Mock
    private LogMessageHandler logMessageHandler;

    @Mock
    private RequestMessageHandler requestMessageHandler;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);

        connectorId = IdsId.Builder.newInstance().type(IdsType.CONNECTOR).value("http://test.connector").build();


        List<Handler> multipartHandlers = List.of(logMessageHandler, requestMessageHandler);
        controller = new MultipartController(monitor, connectorId, mapper, tokenService, IDS_WEBHOOK_ADDRESS, multipartHandlers);
    }

    private <T> T extractHeader(Response response, Class<T> type) throws IOException {

        assertInstanceOf(FormDataMultiPart.class, response.getEntity());
        FormDataMultiPart multiPartResponse = (FormDataMultiPart) response.getEntity();

        var header = multiPartResponse.getField("header");
        assertNotNull(header);

        assertInstanceOf(String.class, header.getEntity());
        var entity = (String) header.getEntity();
        return mapper.readValue(entity.getBytes(), type);
    }

    private <T> T extractPayload(Response response, Class<T> type) throws IOException {

        assertInstanceOf(FormDataMultiPart.class, response.getEntity());
        FormDataMultiPart multiPartResponse = (FormDataMultiPart) response.getEntity();

        var payload = multiPartResponse.getField("payload");
        assertNotNull(payload);

        assertInstanceOf(String.class, payload.getEntity());
        var entity = (String) payload.getEntity();
        return mapper.readValue(entity.getBytes(), type);
    }

    @Test
    public void logMessageSuccess() throws IOException {
        var responseHeader = TestUtils.getValidResponseHeader(mapper);
        var responsePayload = TestUtils.getValidResponsePayload(mapper);

        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));
        doReturn(true)
                .when(logMessageHandler).canHandle(any(HandlerRequest.class));
        doReturn(MultipartResponse.Builder.newInstance().header(responseHeader).payload(responsePayload).build())
                .when(logMessageHandler).handleRequest(any(HandlerRequest.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);

        var response = controller.logMessage(pid, header, PAYLOAD);

        assertNotNull(response);
        assertEquals(Response.Status.CREATED.getStatusCode(), response.getStatus());

        var message = extractHeader(response, Message.class);

        assertTrue(message instanceof LogMessage);

        var payload = extractPayload(response, LoggingMessageResponse.class);
        assertNotNull(payload.getData());
    }

    @Test
    public void createProcessSuccess() throws IOException {
        var responseHeader = TestUtils.getResponseHeader(mapper, TestUtils.VALID_CREATE_PROCESS_HEADER_JSON);
        var responsePayload = TestUtils.getValidResponsePayload(mapper);

        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));
        doReturn(true)
                .when(requestMessageHandler).canHandle(any(HandlerRequest.class));
        doReturn(MultipartResponse.Builder.newInstance().header(responseHeader).payload(responsePayload).build())
                .when(requestMessageHandler).handleRequest(any(HandlerRequest.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_CREATE_PROCESS_HEADER_JSON);

        var response = controller.createProcess(pid, header, CREATE_PROCESS_PAYLOAD);

        assertNotNull(response);
        assertEquals(Response.Status.CREATED.getStatusCode(), response.getStatus());

        var message = extractHeader(response, Message.class);
        assertTrue(message instanceof RequestMessage);

        var payload = extractPayload(response, LoggingMessageResponse.class);
        assertNotNull(payload.getData());
    }

    @Test
    public void invalidMessageToEndpointError() {
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);
        var pid = UUID.randomUUID().toString();

        var response = controller.validateRequest(pid, header, "messages/query/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());
        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingPIDError() {
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);

        var response = controller.validateRequest(null, header, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingHeaderError() {
        var pid = UUID.randomUUID().toString();

        var response = controller.validateRequest(pid, null, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void invalidHeaderError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.INVALID_HEADER_JSON);

        var response = controller.validateRequest(pid, header, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingHeaderFieldsError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.MISSING_FIELDS_HEADER_JSON);

        var response = controller.validateRequest(pid, header, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void invalidSecurityTokenError() {
        doReturn(Result.failure("Invalid token"))
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.INVALID_TOKEN_HEADER_JSON);

        var response = controller.validateRequest(pid, header, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.NOT_AUTHENTICATED.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingSecurityTokenError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.MISSING_TOKEN_HEADER_JSON);

        var response = controller.validateRequest(pid, header, "messages/log/{pid}");

        assertNotNull(response);
        assertTrue(response.fail());

        var message = (RejectionMessage) response.getHeader();

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.NOT_AUTHENTICATED.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingPayloadError() {
        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);

        var response = controller.logMessage(pid, header, null);

        assertNotNull(response);
        assertEquals(Response.Status.BAD_REQUEST.getStatusCode(), response.getStatus());

        var message = (RejectionMessage) response;

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void invalidMessageTypeError() {
        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));
        doReturn(false)
                .when(logMessageHandler).canHandle(any(HandlerRequest.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getResponseHeader(new ObjectMapper(), TestUtils.INVALID_TYPE_HEADER_JSON);

        var response = controller.processRequest(pid, header, PAYLOAD);

        assertNotNull(response);
        assertEquals(Response.Status.INTERNAL_SERVER_ERROR.getStatusCode(), response.getStatus());

        var message = (RejectionMessage) response;

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MESSAGE_TYPE_NOT_SUPPORTED.getId(), message.getRejectionReason().getId());
    }

}
