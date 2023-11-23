package de.truzzt.clearinghouse.edc.tests;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.eclipse.edc.spi.EdcException;

import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;

public class BaseTestUtils {

    protected static <T> T parseFile(ObjectMapper mapper, Class<T> type, String path) {

        ClassLoader classLoader = BaseTestUtils.class.getClassLoader();
        var jsonResource = classLoader.getResource(path);

        if (jsonResource == null) {
            throw new EdcException("Header json file not found: " + path);
        }

        URI jsonUrl;
        try {
            jsonUrl = jsonResource.toURI();
        } catch (URISyntaxException e) {
            throw new EdcException("Error finding json file on classpath", e);
        }

        Path filePath = Path.of(jsonUrl);
        if (!Files.exists(filePath)) {
            throw new EdcException("Header json file not found: " + path);
        }

        T object;
        try {
            var jsonContents = Files.readAllBytes(filePath);
            object = mapper.readValue(jsonContents, type);

        } catch (IOException e){
            throw new EdcException("Error parsing json file", e);
        }

        return object;
    }

    protected static Path getFile(String path) {

        ClassLoader classLoader = BaseTestUtils.class.getClassLoader();
        var jsonResource = classLoader.getResource(path);

        if (jsonResource == null) {
            throw new EdcException("Header json file not found: " + path);
        }

        URI jsonUrl;
        try {
            jsonUrl = jsonResource.toURI();
        } catch (URISyntaxException e) {
            throw new EdcException("Error finding json file on classpath", e);
        }

        Path filePath = Path.of(jsonUrl);
        if (!Files.exists(filePath)) {
            throw new EdcException("Header json file not found: " + path);
        }

        return filePath;
    }

    protected static String readFile(String path) {
        var file = getFile(path);

        try {
            return Files.readString(file);
        } catch (IOException e) {
            throw new EdcException("Error reading file contents", e);
        }
    }
}
