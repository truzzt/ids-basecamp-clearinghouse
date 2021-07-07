package de.fhg.aisec.ids.clearinghouse;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.Message;

import javax.xml.bind.DatatypeConverter;
import java.io.InputStream;

public class ClearingHouseMessage {
    Message header;
    String payload;
    String payloadType;

    public void setHeader(Message header) {
        this.header = header;
    }

    public void setPayload(InputStream payload) throws Exception {
        this.payload = DatatypeConverter.printBase64Binary(payload.readAllBytes());
    }
    public void setPayload(String payload) throws Exception {
        this.payload = payload;
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
