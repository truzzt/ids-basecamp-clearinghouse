package de.truzzt.clearinghouse.edc.multipart.controller;

import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.multipart.types.ids.Message;
import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.glassfish.jersey.media.multipart.FormDataBodyPart;
import org.glassfish.jersey.media.multipart.FormDataParam;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.jetbrains.annotations.NotNull;

import java.io.InputStream;

import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.malformedMessage;

@Consumes({MediaType.MULTIPART_FORM_DATA})
@Produces({MediaType.MULTIPART_FORM_DATA})
@Path("/")
public class MultipartController {

    private static final String HEADER = "header";
    private static final String PAYLOAD = "payload";

    private final IdsId connectorId;

    private final TypeManagerUtil typeManagerUtil;

    public MultipartController(@NotNull IdsId connectorId, TypeManagerUtil typeManagerUtil) {
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
    }

    @POST
    @Path("log")
    public FormDataMultiPart request(@FormDataParam(HEADER) InputStream headerInputStream,
                                     @FormDataParam(PAYLOAD) String payload) {

        if (headerInputStream == null) {
            return createFormDataMultiPart(malformedMessage(null, connectorId));
        }

        return null;
    }

    private FormDataMultiPart createFormDataMultiPart(Message header) {
        var multiPart = new FormDataMultiPart();
        if (header != null) {
            multiPart.bodyPart(new FormDataBodyPart(HEADER, typeManagerUtil.toJson(header), MediaType.APPLICATION_JSON_TYPE));
        }
        return multiPart;
    }

}
