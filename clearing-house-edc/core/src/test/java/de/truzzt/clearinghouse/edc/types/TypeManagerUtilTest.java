package de.truzzt.clearinghouse.edc.types;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.TestUtils;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.Spy;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStream;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

class TypeManagerUtilTest {

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

        File file = new File(TestUtils.LOG_MESSAGE_JSON_PATH);
        file.createNewFile();

        InputStream is = new FileInputStream(file);
        Message msg = typeManagerUtil.parse(is, Message.class);
        assertNotNull(msg);
        assertEquals("ids:LogMessage", msg.getType());
    }

    @Test
    void successfulToJson() throws IOException {
        File file = new File(TestUtils.LOG_MESSAGE_JSON_PATH);
        file.createNewFile();

        Message msgBefore = objectMapper.readValue(file, Message.class);

        byte[] json  = typeManagerUtil.toJson(msgBefore);
        assertNotNull(json);

        InputStream is = new ByteArrayInputStream(json);
        Message msgAfter = typeManagerUtil.parse(is, Message.class);

        assertEquals(msgBefore.getType(), msgAfter.getType());

    }
}