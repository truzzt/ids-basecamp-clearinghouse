package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import de.truzzt.clearinghouse.edc.TestUtils;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.AppSenderDelegate;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import okhttp3.Request;
import okhttp3.ResponseBody;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.monitor.Monitor;
import org.eclipse.edc.spi.system.ServiceExtensionContext;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import org.mockito.invocation.InvocationOnMock;
import org.mockito.stubbing.Answer;

import java.io.IOException;

import static de.truzzt.clearinghouse.edc.TestUtils.getBuildJwtToken;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.APP_BASE_URL_DEFAULT_VALUE;
import static de.truzzt.clearinghouse.edc.util.SettingsConstants.JWT_EXPIRES_AT_DEFAULT_VALUE;
import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doAnswer;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;
import static org.mockito.Mockito.when;

class LogMessageHandlerTest {
    @Mock
    private Monitor monitor;
    @Mock
    private IdsId connectorId;
    @Mock
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private AppSender appSender;
    @Mock
    private ServiceExtensionContext context;
    @Mock
    private LogMessageHandler logMessageHandler;
    @Mock
    private LoggingMessageDelegate senderDelegate;
    @Mock
    private EdcHttpClient httpClient;

    private AppSender sender;

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate(typeManagerUtil));
        logMessageHandler = spy(new LogMessageHandler(monitor, connectorId, typeManagerUtil, appSender, context));
        sender = new AppSender(monitor, httpClient ,typeManagerUtil);
    }

    @Test
    public void successfulCanHandle(){

        HandlerRequest request = TestUtils.getValidHandlerRequest();

        Boolean response = logMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(response, true);
    }

    @Test
    public void invalidMessageTypeCanHandle(){

        HandlerRequest request = TestUtils.getInvalidHandlerRequest();

        Boolean response = logMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(response, false);
    }

    @Test
    public void successfulHandleRequest(){
        HandlerRequest request = TestUtils.getValidHandlerRequest();
        doReturn(JWT.create().toString()).when(logMessageHandler).buildJWTToken(any(SecurityToken.class), any(ServiceExtensionContext.class));
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest().getUrl())).when(senderDelegate).parseResponseBody(any(ResponseBody.class));
        doReturn(APP_BASE_URL_DEFAULT_VALUE+"/messages/log/" + request.getPid()).when(senderDelegate).buildRequestUrl(any(String.class), any(HandlerRequest.class));
        doReturn(TestUtils.getValidLoggingMessageRequest(request)).when(senderDelegate).buildRequestBody(any(HandlerRequest.class));

        HandlerResponse response = logMessageHandler.handleRequest(request);

        assertNotNull(response);
        assertEquals(response.getHeader().getType(), "ids:MessageProcessedNotificationMessage");
    }

    @Test
    public void missingSubjectBuildJwtToken() {
        EdcException exception = assertThrows(EdcException.class, () -> logMessageHandler.buildJWTToken(
                TestUtils.getInvalidTokenHandlerRequest()
                        .getHeader()
                        .getSecurityToken(), context));

        assertEquals("JWT Token subject is missing",exception.getMessage());
    }

    @Test
    public void successfulBuildJwtToken() {
        var response = logMessageHandler.buildJWTToken(
                TestUtils.getValidHandlerRequest()
                        .getHeader()
                        .getSecurityToken(), context);

        assertNotNull(response);
    }

}