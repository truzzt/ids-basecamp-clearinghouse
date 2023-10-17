package de.truzzt.clearinghouse.edc.types;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import org.eclipse.edc.spi.EdcException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import java.io.ByteArrayInputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.Mockito.doThrow;

class TypeManagerUtilTest {

    @Mock
    private ObjectMapper objectMapper;
    @Mock
    private TypeManagerUtil typeManagerUtil;

    @BeforeEach
    void setUp() {
        MockitoAnnotations.openMocks(this);
        typeManagerUtil = new TypeManagerUtil(objectMapper);
    }

    @Test
    void successfulParse() throws IOException, InstantiationException, IllegalAccessException {
        typeManagerUtil = new TypeManagerUtil(new ObjectMapper());
        InputStream is = new FileInputStream(TestUtils.getValidHeaderFile());

        Message msg = typeManagerUtil.parse(is, Message.class);
        assertNotNull(msg);
        assertEquals("ids:LogMessage", msg.getType());
    }

    @Test
    void typeErrorParse() {
        typeManagerUtil = new TypeManagerUtil(new ObjectMapper());
        EdcException exception =
                assertThrows(EdcException.class,
                        () -> typeManagerUtil.parse(
                                new FileInputStream(TestUtils.getInvalidHeaderFile()),
                                Message.class)
                );
                assertEquals(
                        "Error parsing to type class de.truzzt.clearinghouse.edc.types.ids.Message",
                        exception.getMessage()
                );

    }

    @Test
    void successfulToJson() throws IOException {
        objectMapper = new ObjectMapper();
        typeManagerUtil = new TypeManagerUtil(objectMapper);
        Message msgBefore = objectMapper.readValue(TestUtils.getValidHeaderFile(), Message.class);

        byte[] json  = typeManagerUtil.toJson(msgBefore);
        assertNotNull(json);

        InputStream is = new ByteArrayInputStream(json);
        Message msgAfter = typeManagerUtil.parse(is, Message.class);

        assertEquals(msgBefore.getType(), msgAfter.getType());

    }

    @Test
    void errorConvertingToJson() throws IOException {
        doThrow(new EdcException("Error converting to JSON")).when(objectMapper).writeValueAsBytes(anyString());

        EdcException exception =
                assertThrows(EdcException.class,
                        () -> typeManagerUtil.toJson("")
                );

        assertEquals("Error converting to JSON",exception.getMessage() );
    }
}