package de.truzzt.clearinghouse.edc.types;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.Message;
import de.truzzt.clearinghouse.edc.tests.TestUtils;
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
import static org.mockito.Mockito.mock;

class TypeManagerUtilTest {

    private final ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    void setUp() {
        MockitoAnnotations.openMocks(this);
    }

    @Test
    void successfulParse() throws IOException {
        InputStream is = new FileInputStream(TestUtils.getValidHeaderFile());

        Message msg = mapper.readValue(is, Message.class);
        assertNotNull(msg);
        assertEquals("ids:LogMessage", msg.getClass().getSimpleName());
    }

    @Test
    void typeErrorParse() {
        EdcException exception =
                assertThrows(EdcException.class,
                        () -> mapper.readValue(
                                new FileInputStream(TestUtils.getInvalidHeaderFile()),
                                Message.class)
                );
        assertEquals("Error parsing to type class de.truzzt.clearinghouse.edc.types.ids.Message", exception.getMessage());
    }

    @Test
    void successfulToJson() throws IOException {
        Message msgBefore = mapper.readValue(TestUtils.getValidHeaderFile(), Message.class);

        var json  = mapper.writeValueAsString(msgBefore);
        assertNotNull(json);

        InputStream is = new ByteArrayInputStream(json.getBytes());
        Message msgAfter = mapper.readValue(is, Message.class);

        assertEquals(msgBefore.getClass().getSimpleName(), msgAfter.getClass().getSimpleName());
    }

    @Test
    void errorConvertingToJson() throws IOException {

        var mockedMapper = mock(ObjectMapper.class);
        doThrow(new EdcException("Error converting to JSON"))
                .when(mockedMapper).writeValueAsString(anyString());

        EdcException exception =
                assertThrows(EdcException.class,
                        () -> mapper.writeValueAsString("fadsfsdafd")
                );

        assertEquals("Error converting to JSON",exception.getMessage() );
    }
}