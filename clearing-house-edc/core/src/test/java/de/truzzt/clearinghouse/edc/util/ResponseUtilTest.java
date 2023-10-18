package de.truzzt.clearinghouse.edc.util;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static org.junit.jupiter.api.Assertions.assertNotNull;

class ResponseUtilTest {

    @Mock
    private ResponseUtil responseUtil;
    @Mock
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private IdsId connectorId;

    private ObjectMapper objectMapper;

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        typeManagerUtil = new TypeManagerUtil(new ObjectMapper());
        objectMapper = new ObjectMapper();
    }

    @Test
    public void createFormDataMultiPart() {

        var response = responseUtil.createFormDataMultiPart(typeManagerUtil,
                "Header Name",
                TestUtils.getValidHeader(new ObjectMapper()),
                "Payload",
                "Payload Value"
        );

        assertNotNull(response);
    }

    @Test
    public void testCreateFormDataMultiPart() {
        var response = responseUtil.createFormDataMultiPart(typeManagerUtil,
                "Header Name",
                TestUtils.getValidHeader(objectMapper)
        );

        assertNotNull(response);
    }

    @Test
    public void createMultipartResponse() {
        var response = responseUtil.createMultipartResponse(TestUtils.getValidHeader(objectMapper),
                "Payload Value");

        assertNotNull(response);
    }

    @Test
    public void messageProcessedNotification() {
        var response = responseUtil.messageProcessedNotification(TestUtils.getValidHeader(objectMapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void notAuthenticated() {
        var response = responseUtil.notAuthenticated(TestUtils.getValidHeader(objectMapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void malformedMessage() {
        var response = responseUtil.malformedMessage(TestUtils.getValidHeader(objectMapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void messageTypeNotSupported() {
        var response = responseUtil.messageTypeNotSupported(TestUtils.getValidHeader(objectMapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void internalRecipientError() {
        var response = responseUtil.internalRecipientError(TestUtils.getValidHeader(objectMapper), connectorId);

        assertNotNull(response);
    }
}