package de.truzzt.clearinghouse.edc.multipart;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.DynamicAttributeToken;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.handler.Handler;
import de.truzzt.clearinghouse.edc.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.handler.RequestMessageHandler;
import de.truzzt.clearinghouse.edc.multipart.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import de.truzzt.clearinghouse.edc.types.ids.RejectionMessage;
import de.truzzt.clearinghouse.edc.types.ids.RejectionReason;
import jakarta.ws.rs.core.Response;
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

import java.io.ByteArrayInputStream;
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
    private TypeManagerUtil typeManagerUtil;

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
        typeManagerUtil = new TypeManagerUtil(new ObjectMapper());

        List<Handler> multipartHandlers = List.of(logMessageHandler, requestMessageHandler);
        controller = new MultipartController(monitor, connectorId, typeManagerUtil, tokenService, IDS_WEBHOOK_ADDRESS, multipartHandlers);
    }

    private <T> T extractHeader(Response response, Class<T> type) {

        assertInstanceOf(FormDataMultiPart.class, response.getEntity());
        FormDataMultiPart multiPartResponse = (FormDataMultiPart) response.getEntity();

        var header = multiPartResponse.getField("header");
        assertNotNull(header);

        assertInstanceOf(String.class, header.getEntity());
        var entity = (String) header.getEntity();
        return typeManagerUtil.parse(new ByteArrayInputStream(entity.getBytes()), type);
    }

    private <T> T extractPayload(Response response, Class<T> type) {

        assertInstanceOf(FormDataMultiPart.class, response.getEntity());
        FormDataMultiPart multiPartResponse = (FormDataMultiPart) response.getEntity();

        var payload = multiPartResponse.getField("payload");
        assertNotNull(payload);

        assertInstanceOf(String.class, payload.getEntity());
        var entity = (String) payload.getEntity();
        return typeManagerUtil.parse(new ByteArrayInputStream(entity.getBytes()), type);
    }

    @Test
    public void logMessageSuccess() {
        var responseHeader = TestUtils.getValidResponseHeader(mapper);
        var responsePayload = TestUtils.getValidResponsePayload(mapper);

        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));
        doReturn(true)
                .when(logMessageHandler).canHandle(any(HandlerRequest.class));
        doReturn(HandlerResponse.Builder.newInstance().header(responseHeader).payload(responsePayload).build())
                .when(logMessageHandler).handleRequest(any(HandlerRequest.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);

        var response = controller.logMessage(pid, header, PAYLOAD);

        assertNotNull(response);
        assertEquals(Response.Status.CREATED.getStatusCode(), response.getStatus());

        var message = extractHeader(response, Message.class);
        assertEquals("ids:LogMessage", message.getType());

        var payload = extractPayload(response, LoggingMessageResponse.class);
        assertNotNull(payload.getData());
    }

    @Test
    public void createProcessSuccess() {
        var responseHeader = TestUtils.getResponseHeader(mapper, TestUtils.VALID_CREATE_PROCESS_HEADER_JSON);
        var responsePayload = TestUtils.getValidResponsePayload(mapper);

        doReturn(Result.success())
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));
        doReturn(true)
                .when(requestMessageHandler).canHandle(any(HandlerRequest.class));
        doReturn(HandlerResponse.Builder.newInstance().header(responseHeader).payload(responsePayload).build())
                .when(requestMessageHandler).handleRequest(any(HandlerRequest.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_CREATE_PROCESS_HEADER_JSON);

        var response = controller.createProcess(pid, header, CREATE_PROCESS_PAYLOAD);

        assertNotNull(response);
        assertEquals(Response.Status.CREATED.getStatusCode(), response.getStatus());

        var message = extractHeader(response, Message.class);
        assertEquals("ids:RequestMessage", message.getType());

        var payload = extractPayload(response, LoggingMessageResponse.class);
        assertNotNull(payload.getData());
    }

    @Test
    public void missingPIDError() {
        var header = TestUtils.getHeaderInputStream(TestUtils.VALID_HEADER_JSON);

        var response = controller.validateRequest(null, header);

        assertNotNull(response);
        assertTrue(response.fail());

        var message = extractHeader(response.getError(), RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingHeaderError() {
        var pid = UUID.randomUUID().toString();

        var response = controller.validateRequest(pid, null);

        assertNotNull(response);
        assertTrue(response.fail());

       var message = extractHeader(response.getError(), RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void invalidHeaderError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.INVALID_HEADER_JSON);

        var response = controller.validateRequest(pid, header);

        assertNotNull(response);
        assertTrue(response.fail());

        var message = extractHeader(response.getError(), RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingHeaderFieldsError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.MISSING_FIELDS_HEADER_JSON);

        var response = controller.validateRequest(pid, header);

        assertNotNull(response);
        assertTrue(response.fail());

        var message = extractHeader(response.getError(), RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MALFORMED_MESSAGE.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void invalidSecurityTokenError() {
        doReturn(Result.failure("Invalid token"))
                .when(tokenService).verifyDynamicAttributeToken(any(DynamicAttributeToken.class), any(URI.class), any(String.class));

        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.INVALID_TOKEN_HEADER_JSON);

        var response = controller.validateRequest(pid, header);

        assertNotNull(response);
        assertTrue(response.fail());

        var message = extractHeader(response.getError(), RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.NOT_AUTHENTICATED.getId(), message.getRejectionReason().getId());
    }

    @Test
    public void missingSecurityTokenError() {
        var pid = UUID.randomUUID().toString();
        var header = TestUtils.getHeaderInputStream(TestUtils.MISSING_TOKEN_HEADER_JSON);

        var response = controller.validateRequest(pid, header);

        assertNotNull(response);
        assertTrue(response.fail());

        var message = extractHeader(response.getError(), RejectionMessage.class);

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

        var message = extractHeader(response, RejectionMessage.class);

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

        var message = extractHeader(response, RejectionMessage.class);

        assertNotNull(message.getRejectionReason());
        assertEquals(RejectionReason.MESSAGE_TYPE_NOT_SUPPORTED.getId(), message.getRejectionReason().getId());
    }

}
