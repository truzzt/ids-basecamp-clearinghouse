package de.fhg.aisec.ids.clearinghouse;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.Message;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ClearingHouseMessage {

    static final Logger LOG = LoggerFactory.getLogger(ClearingHouseMessage.class);

    Message header;
    String payload;
    String payloadType;

    public void setHeader(Message header) {
        this.header = header;
    }

    public void setPayloadType(String payloadType) {
        this.payloadType = payloadType;
    }

    public void setPayload(String payload){
        this.payload = payload;
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
