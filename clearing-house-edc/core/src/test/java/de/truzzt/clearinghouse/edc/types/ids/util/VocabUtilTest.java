package de.truzzt.clearinghouse.edc.types.ids.util;

import de.truzzt.clearinghouse.edc.types.util.VocabUtil;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;

class VocabUtilTest {
    @Mock
    private VocabUtil vocabUtil;

    @BeforeEach
    void setUp() {
        MockitoAnnotations.openMocks(this);
    }

    @Test
    void successfulCreateRandomUrl() {
        vocabUtil.randomUrlBase = "http://www.test";
        var response = vocabUtil.createRandomUrl("test-successful");
        assertNotNull(response);
        assertEquals("http://www.test/test-successful/", response.toString().substring(0,32));
    }

    @Test
    void errorInvalidUrlCreateRandomUrl() {
        vocabUtil.randomUrlBase = "htt://.....";
        RuntimeException exception = assertThrows(RuntimeException.class, () ->
                vocabUtil.createRandomUrl("test-successful"));

        assertNotNull(exception);
        assertEquals("java.net.MalformedURLException: unknown protocol: htt", exception.getMessage());
    }

    @Test
    void successfulNullRandomUrlCreateRandomUrl() {
        vocabUtil.randomUrlBase = null;
        var response = vocabUtil.createRandomUrl("test-successful");
        assertNotNull(response);
        assertEquals("https://w3id.org/idsa/autogen/test-successful/", response.toString().substring(0,46));
    }
}