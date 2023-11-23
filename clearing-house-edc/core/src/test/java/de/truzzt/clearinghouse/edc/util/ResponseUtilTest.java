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
    private TypeManagerUtil typeManagerUtil;
    @Mock
    private IdsId connectorId;

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    public void setUp() {
        MockitoAnnotations.openMocks(this);
        typeManagerUtil = new TypeManagerUtil(new ObjectMapper());
    }

    @Test
    public void createFormDataMultiPart() {

        var response = ResponseUtil.createFormDataMultiPart(typeManagerUtil,
                "Header Name",
                TestUtils.getValidHeader(new ObjectMapper()),
                "Payload",
                "Payload Value"
        );

        assertNotNull(response);
    }

    @Test
    public void testCreateFormDataMultiPart() {
        var response = ResponseUtil.createFormDataMultiPart(typeManagerUtil,
                "Header Name",
                TestUtils.getValidHeader(mapper)
        );

        assertNotNull(response);
    }

    @Test
    public void createMultipartResponse() {
        var response = ResponseUtil.createMultipartResponse(TestUtils.getValidHeader(mapper),
                "Payload Value");

        assertNotNull(response);
    }

    @Test
    public void messageProcessedNotification() {
        var response = ResponseUtil.messageProcessedNotification(TestUtils.getValidHeader(mapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void notAuthenticated() {
        var response = ResponseUtil.notAuthenticated(TestUtils.getValidHeader(mapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void malformedMessage() {
        var response = ResponseUtil.malformedMessage(TestUtils.getValidHeader(mapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void messageTypeNotSupported() {
        var response = ResponseUtil.messageTypeNotSupported(TestUtils.getValidHeader(mapper), connectorId);

        assertNotNull(response);
    }

    @Test
    public void internalRecipientError() {
        var response = ResponseUtil.internalRecipientError(TestUtils.getValidHeader(mapper), connectorId);

        assertNotNull(response);
    }
}