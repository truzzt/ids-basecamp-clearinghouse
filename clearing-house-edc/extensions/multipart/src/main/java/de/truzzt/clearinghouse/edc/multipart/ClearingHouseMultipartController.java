package de.truzzt.clearinghouse.edc.multipart;

import de.truzzt.clearinghouse.edc.multipart.handler.Handler;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.multipart.types.ids.Message;

import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.PathParam;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

import jakarta.ws.rs.core.Response;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.glassfish.jersey.media.multipart.FormDataBodyPart;
import org.glassfish.jersey.media.multipart.FormDataParam;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.jetbrains.annotations.NotNull;

import java.io.InputStream;
import java.util.List;

import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.malformedMessage;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.messageTypeNotSupported;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.notAuthenticated;
import static java.lang.String.format;

@Consumes({MediaType.MULTIPART_FORM_DATA})
@Produces({MediaType.MULTIPART_FORM_DATA})
@Path("/")
public class ClearingHouseMultipartController {

    private static final String HEADER = "header";
    private static final String PAYLOAD = "payload";
    private static final String PID = "pid";
    private static final String LOG_ID = "InfrastructureController";

    private final Monitor monitor;
    private final IdsId connectorId;

    private final List<Handler> multipartHandlers;

    private final TypeManagerUtil typeManagerUtil;

    public ClearingHouseMultipartController(@NotNull Monitor monitor,
                                            @NotNull IdsId connectorId,
                                            @NotNull TypeManagerUtil typeManagerUtil,
                                            @NotNull List<Handler> multipartHandlers) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
        this.multipartHandlers = multipartHandlers;
    }

    @POST
    @Path("messages/log/{pid}")
    public Response request(@PathParam(PID) String pid,
                            @FormDataParam(HEADER) InputStream headerInputStream,
                            @FormDataParam(PAYLOAD) String payload) {

        // Check if header is missing
        if (headerInputStream == null) {
            monitor.warning(LOG_ID + ": Header is missing");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Convert header to message
        Message header;
        try {
            header = typeManagerUtil.parse(headerInputStream, Message.class);
        } catch (Exception e) {
            monitor.warning(format(LOG_ID + ": Header parsing failed: %s", e.getMessage()));
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Check if any required header field missing
        if (header.getId() == null
                || header.getType() == null
                || header.getModelVersion() == null
                || header.getIssued() == null
                || header.getIssuerConnector() == null
                || header.getSenderAgent() == null) {
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(header, connectorId)))
                    .build();
        }

        // Check if DAT present
        var dynamicAttributeToken = header.getSecurityToken();
        if (dynamicAttributeToken == null || dynamicAttributeToken.getTokenValue() == null) {
            monitor.warning(LOG_ID + ": Token is missing in header");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(notAuthenticated(header, connectorId)))
                    .build();
        }

        // Check if payload is missing
        if (payload == null) {
            monitor.warning(LOG_ID + ": Payload is missing");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Build the multipart request
        var multipartRequest = MultipartRequest.Builder.newInstance()
                .pid(pid)
                .header(header)
                .payload(payload)
                .build();

        // Send to handler processing
        var multipartResponse = multipartHandlers.stream()
                .filter(h -> h.canHandle(multipartRequest))
                .findFirst()
                .map(it -> it.handleRequest(multipartRequest))
                .orElseGet(() -> MultipartResponse.Builder.newInstance()
                        .header(messageTypeNotSupported(header, connectorId))
                        .build());

        // Build response
        return Response.status(Response.Status.CREATED)
                .entity(createFormDataMultiPart(multipartResponse.getHeader(), multipartResponse.getPayload()))
                .build();
    }

    private FormDataMultiPart createFormDataMultiPart(Message header, Object payload) {
        var multiPart = createFormDataMultiPart(header);

        if (payload != null) {
            multiPart.bodyPart(new FormDataBodyPart(PAYLOAD, typeManagerUtil.toJson(payload), MediaType.APPLICATION_JSON_TYPE));
        }

        return multiPart;
    }

    private FormDataMultiPart createFormDataMultiPart(Message header) {
        var multiPart = new FormDataMultiPart();
        if (header != null) {
            multiPart.bodyPart(new FormDataBodyPart(HEADER, typeManagerUtil.toJson(header), MediaType.APPLICATION_JSON_TYPE));
        }
        return multiPart;
    }

}
