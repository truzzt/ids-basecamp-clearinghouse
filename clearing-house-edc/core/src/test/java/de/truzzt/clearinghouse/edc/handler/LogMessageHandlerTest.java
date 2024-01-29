package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.DynamicAttributeToken;
import de.fraunhofer.iais.eis.LogMessage;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import okhttp3.ResponseBody;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartResponse;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_DEFAULT_VALUE;
import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;

class LogMessageHandlerTest {
    @Mock
    private IdsId connectorId;
    @Mock
    private AppSender appSender;
    @Mock
    private ServiceExtensionContext context;
    @Mock
    private LogMessageHandler logMessageHandler;
    @Mock
    private LoggingMessageDelegate senderDelegate;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate());
        logMessageHandler = spy(new LogMessageHandler(connectorId, appSender, context));
    }

    @Test
    public void successfulCanHandle(){

        HandlerRequest request = TestUtils.getValidHandlerRequest(mapper);

        Boolean response = logMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(response, true);
    }

    @Test
    public void invalidMessageTypeCanHandle(){

        HandlerRequest request = TestUtils.getInvalidHandlerRequest(mapper);

        Boolean response = logMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(response, false);
    }

    @Test
    public void successfulHandleRequest(){
        HandlerRequest request = TestUtils.getValidHandlerRequest(mapper);
        doReturn(JWT.create().toString())
                .when(logMessageHandler).buildJWTToken(any(DynamicAttributeToken.class), any(ServiceExtensionContext.class));
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl(), mapper))
                .when(senderDelegate).parseResponseBody(any(ResponseBody.class));
        doReturn(APP_BASE_URL_DEFAULT_VALUE+ "/headers/log/" + request.getPid())
                .when(senderDelegate)
                .buildRequestUrl(any(String.class), any(HandlerRequest.class));
        doReturn(TestUtils.getValidLoggingMessageRequest(request))
                .when(senderDelegate).buildRequestBody(any(HandlerRequest.class));

        MultipartResponse response = logMessageHandler.handleRequest(request);

        assertNotNull(response);
        var ok = response.getHeader() instanceof LogMessage;
        assertTrue(ok);
    }

    @Test
    public void missingSubjectBuildJwtToken() {
        EdcException exception = assertThrows(EdcException.class, () -> logMessageHandler.buildJWTToken(
                TestUtils.getInvalidTokenHandlerRequest(mapper)
                        .getHeader()
                        .getSecurityToken(), context));

        assertEquals("JWT Token subject is missing",exception.getMessage());
    }
    @Test
    public void successfulBuildJwtToken() {
        doReturn("1").when(context).getSetting(anyString(), anyString());
        var response = logMessageHandler.buildJWTToken(
                TestUtils.getValidHandlerRequest(mapper)
                        .getHeader()
                        .getSecurityToken(), context);

        assertNotNull(response);
    }
}