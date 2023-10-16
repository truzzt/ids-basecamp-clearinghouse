package de.truzzt.clearinghouse.edc.app.delegate;

import de.truzzt.clearinghouse.edc.TestUtils;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import okhttp3.ResponseBody;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.spy;

class LoggingMessageDelegateTest {

    @Mock
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private LoggingMessageDelegate senderDelegate;
    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        senderDelegate = spy(new LoggingMessageDelegate(typeManagerUtil));
    }

    @Test
    public void successfulBuildRequestUrl() {

        HandlerRequest request = TestUtils.getValidHandlerRequest();

        String response = senderDelegate.buildRequestUrl(TestUtils.TEST_BASE_URL, request);

        assertNotNull(response);
        assertEquals(response, "http://localhost:8000/messages/log/" +request.getPid());
    }

    @Test
    public void successfulBuildRequestBody() {

        HandlerRequest request = TestUtils.getValidHandlerRequest();

        LoggingMessageRequest response = senderDelegate.buildRequestBody(request);

        assertNotNull(response);
    }

    @Test
    public void successfulParseResponseBody() {

        ResponseBody body = TestUtils.getValidResponseBody();
        doReturn(TestUtils.getValidLoggingMessageResponse(TestUtils.getValidAppSenderRequest().getUrl())).when(senderDelegate).parseResponseBody(any(ResponseBody.class));
        LoggingMessageResponse response = senderDelegate.parseResponseBody(body);

        assertNotNull(response);
    }

}