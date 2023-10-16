package de.truzzt.clearinghouse.edc.types;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.LogMessage;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import org.eclipse.edc.spi.EdcException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.Spy;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;

class TypeManagerUtilTest {

    private static final String VALID_HEADER_JSON = "messages/valid-header.json";

    @Spy
    private ObjectMapper objectMapper;
    @Mock
    private TypeManagerUtil typeManagerUtil;

    @BeforeEach
    void setUp() {
        objectMapper = new ObjectMapper();
        typeManagerUtil = new TypeManagerUtil(objectMapper);
    }

    @Test
    void successfulParse() throws IOException {

        InputStream is = new FileInputStream(TestUtils.getValidHeaderFile());
        Message msg = typeManagerUtil.parse(is, Message.class);
        assertNotNull(msg);
        assertEquals("ids:LogMessage", msg.getType());
    }

    @Test
    void typeErrorParse() {

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

    void successfulToJson() throws IOException {

        Message msgBefore = objectMapper.readValue(TestUtils.getValidHeaderFile(), Message.class);

        byte[] json  = typeManagerUtil.toJson(msgBefore);
        assertNotNull(json);

        InputStream is = new ByteArrayInputStream(json);
        Message msgAfter = typeManagerUtil.parse(is, Message.class);

        assertEquals(msgBefore.getType(), msgAfter.getType());

    }
}