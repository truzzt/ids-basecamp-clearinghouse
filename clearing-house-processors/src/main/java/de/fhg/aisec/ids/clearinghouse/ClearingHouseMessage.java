package de.fhg.aisec.ids.clearinghouse;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.Message;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.xml.bind.DatatypeConverter;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;

public class ClearingHouseMessage {

    static final Logger LOG = LoggerFactory.getLogger(ClearingHouseMessage.class);

    Message header;
    String payload;
    String payloadType;

    public void setHeader(Message header) {
        this.header = header;
    }

    public void setPayload(InputStream payload) throws IOException {
        if (payload == null) {
            this.payloadType = "text/plain";
            this.payload = "";
        }
        else {
            boolean isPayloadTypeRecognized = false;
            if (this.payloadType != null) {
                LOG.debug("payloadType = ", this.payloadType);
                if (this.payloadType.equals("application/json") || this.payloadType.equals("text/plain")) {
                    this.payload = new String(payload.readAllBytes(), StandardCharsets.ISO_8859_1);
                    LOG.debug("payload :", this.payload);
                    isPayloadTypeRecognized = true;
                }
            }

            if (!isPayloadTypeRecognized) {
                this.payloadType = "application/octet-stream";
                this.payload = DatatypeConverter.printBase64Binary(payload.readAllBytes());
            }
        }
    }

    public void setPayloadType(String payloadType) {
        this.payloadType = payloadType;
    }

    public Message getHeader() {
        return this.header;
    }

    public String getPayload() {
        return this.payload;
    }

    public String getPayloadType() {
        return this.payloadType;
    }

    public String toJson() throws Exception {
        ObjectMapper objectMapper = new ObjectMapper();
        objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
        return objectMapper.writeValueAsString(this);
    }
}
