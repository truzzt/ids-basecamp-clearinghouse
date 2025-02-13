package de.truzzt.clearinghouse.edc.handler;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.app.delegate.CreateProcessDelegate;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import okhttp3.ResponseBody;
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

class RequestMessageHandlerTest {

    @Mock
    private IdsId connectorId;
    @Mock
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private AppSender appSender;
    @Mock
    private ServiceExtensionContext context;

    @Mock
    private CreateProcessDelegate createProcessDelegate;
    @Mock
    private RequestMessageHandler requestMessageHandler;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        requestMessageHandler = spy(new RequestMessageHandler(connectorId, typeManagerUtil, appSender, context));
    }
    @Test
    void successfulCanHandle() {
        HandlerRequest request = TestUtils.getValidHandlerCreateProcessRequest(mapper);

        Boolean response = requestMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(true, response);
    }

    @Test
    public void invalidMessageTypeCanHandle(){

        HandlerRequest request = TestUtils.getInvalidHandlerRequest(mapper);

        Boolean response = requestMessageHandler.canHandle(request);

        assertNotNull(response);
        assertEquals(response, false);
    }

    @Test
    public void successfulHandleRequest(){
        HandlerRequest request = TestUtils.getValidHandlerRequest(mapper);
        doReturn(JWT.create().toString())
                .when(requestMessageHandler).buildJWTToken(any(SecurityToken.class), any(ServiceExtensionContext.class));

        doReturn(TestUtils.getValidCreateProcessResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl(), mapper))
                .when(createProcessDelegate).parseResponseBody(any(ResponseBody.class));

        doReturn(APP_BASE_URL_DEFAULT_VALUE+ "/process/" + request.getPid())
                .when(createProcessDelegate)
                .buildRequestUrl(any(String.class), any(HandlerRequest.class));

        doReturn(TestUtils.getValidCreateProcessRequest(request))
                .when(createProcessDelegate).buildRequestBody(any(HandlerRequest.class));

        HandlerResponse response = requestMessageHandler.handleRequest(request);

        assertNotNull(response);
        assertEquals(response.getHeader().getType(), "ids:MessageProcessedNotificationMessage");
    }

    @Test
    public void missingReferringConnectorBuildJwtToken() {
        EdcException exception = assertThrows(EdcException.class, () -> requestMessageHandler.buildJWTToken(
                TestUtils.getInvalidTokenHandlerRequest(mapper)
                        .getHeader()
                        .getSecurityToken(), context));

        assertEquals("JWT Token referringConnector is missing",exception.getMessage());
    }
    @Test
    public void successfulBuildJwtToken() {
        doReturn("1").when(context).getSetting(anyString(), anyString());
        var response = requestMessageHandler.buildJWTToken(
                TestUtils.getValidHandlerRequest(mapper)
                        .getHeader()
                        .getSecurityToken(), context);

        assertNotNull(response);
    }
}