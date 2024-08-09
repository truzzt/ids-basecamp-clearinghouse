package de.truzzt.clearinghouse.edc.app;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.app.delegate.LoggingMessageDelegate;
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

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;

public class AppSenderTest {

    private AppSender sender;

    @Mock
    private Monitor monitor;
    @Mock
    private LoggingMessageDelegate senderDelegate;
    @Mock
    private EdcHttpClient httpClient;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate(monitor, mapper));
        sender = new AppSender(monitor, httpClient, mapper);
    }

    @Test
    public void sendSuccessful() throws IOException {

        doReturn(TestUtils.getValidResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse())
                .when(senderDelegate).buildSuccessResponse(any(ResponseBody.class));

        var response = sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate);

        assertNotNull(response);
        assertTrue(response.isSuccess());
        assertNull(response.getHttpStatus());
    }

    @Test
    public void sendWithHttpRequestError() {

        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () ->
                sender.send(TestUtils.getInvalidUrlAppSenderRequest(mapper), senderDelegate));

        assertEquals("Expected URL scheme 'http' or 'https'", exception.getMessage().substring(0,37));
    }

    @Test
    public void sendWithUnsuccessfulResponseBodyError() throws IOException {

        doReturn(TestUtils.getUnsuccessfulResponse(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse())
                .when(senderDelegate).buildSuccessResponse(any(ResponseBody.class));

        var response = sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate);

        assertNotNull(response);
        assertFalse(response.isSuccess());
        assertEquals(401, response.getHttpStatus());
    }

    @Test
    public void sendWithNullResponseBodyError() throws IOException {

        doReturn(TestUtils.getResponseWithoutBody(TestUtils.getValidAppSenderRequest(mapper).getUrl()))
                .when(httpClient).execute(any(Request.class));
        doReturn(TestUtils.getValidLoggingMessageResponse())
                .when(senderDelegate).buildSuccessResponse(any(ResponseBody.class));

        EdcException exception = assertThrows(EdcException.class, () ->
                sender.send(TestUtils.getValidAppSenderRequest(mapper), senderDelegate));

        assertEquals("Received an empty response body from Clearing House App", exception.getMessage());
    }
}