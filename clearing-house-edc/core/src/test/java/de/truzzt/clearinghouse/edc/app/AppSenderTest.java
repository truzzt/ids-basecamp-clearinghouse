package de.truzzt.clearinghouse.edc.app;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import okhttp3.Request;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.http.EdcHttpClient;
import org.eclipse.edc.spi.monitor.Monitor;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;

public class AppSenderTest {

    private AppSender sender;
    @Mock
    private Monitor monitor;
    @Mock
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private LoggingMessageDelegate senderDelegate;
    @Mock
    private EdcHttpClient httpClient;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate(typeManagerUtil));
        sender = new AppSender(monitor, httpClient ,typeManagerUtil);
    }

    @Test
    public void sendSuccessful() throws IOException {

        doReturn(TestUtils.getValidHandlerRequest(mapper).toString())
                .when(typeManagerUtil).toJson(any(Object.class));
        doReturn(TestUtils.getValidResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl(), mapper))
                .when(senderDelegate).parseResponseBody(any(ResponseBody.class));

        var response = sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate);

        assertNotNull(response);
    }

    @Test
    public void sendWithHttpRequestError() throws IOException {

        doReturn(TestUtils.getValidHandlerRequest(mapper).toString())
                .when(typeManagerUtil).toJson(any(Object.class));

        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () ->
                sender.send(TestUtils.getInvalidUrlAppSenderRequest(mapper), senderDelegate));

        assertEquals("Expected URL scheme 'http' or 'https'", exception.getMessage().substring(0,37));
    }

    @Test
    public void sendWithUnsuccessfulResponseBodyError() throws IOException {

        doReturn(TestUtils.getValidHandlerRequest(mapper).toString())
                .when(typeManagerUtil).toJson(any(Object.class));
        doReturn(TestUtils.getUnsuccessfulResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl(), mapper))
                .when(senderDelegate).parseResponseBody(any(ResponseBody.class));

        EdcException exception = assertThrows(EdcException.class, () ->
                sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate));

        assertEquals("Received an error from Clearing House App. Status: 401, message: Unauthorized", exception.getMessage());
    }

    @Test
    public void sendWithNullResponseBodyError() throws IOException {

        doReturn(TestUtils.getValidHandlerRequest(mapper).toString())
                .when(typeManagerUtil).toJson(any(Object.class));
        doReturn(TestUtils.getResponseWithoutBody(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl(), mapper))
                .when(senderDelegate).parseResponseBody(any(ResponseBody.class));

        EdcException exception = assertThrows(EdcException.class, () ->
                sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate));

        assertEquals("Error reading Clearing House App response body", exception.getMessage());
    }
}