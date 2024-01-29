/*
 *  Copyright (c) 2023 Microsoft Corporation
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       Microsoft Corporation - Initial implementation
 *       truzzt GmbH - EDC extension implementation
 *
 */
package de.truzzt.clearinghouse.edc.multipart;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.*;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.QueryMessageRequest;
import de.truzzt.clearinghouse.edc.multipart.dto.PaggingValidationResponse;
import de.truzzt.clearinghouse.edc.multipart.dto.RequestValidationResponse;
import de.truzzt.clearinghouse.edc.types.Pagging;

import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;

import jakarta.ws.rs.core.Response;
import org.eclipse.edc.protocol.ids.api.multipart.controller.AbstractMultipartController;
import org.eclipse.edc.protocol.ids.api.multipart.handler.Handler;
import org.eclipse.edc.protocol.ids.api.multipart.message.MultipartResponse;
import org.eclipse.edc.protocol.ids.spi.service.DynamicAttributeTokenService;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.eclipse.edc.util.string.StringUtils;
import org.glassfish.jersey.media.multipart.FormDataParam;
import org.jetbrains.annotations.NotNull;

import java.io.InputStream;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.List;

import static java.lang.String.format;
import static org.eclipse.edc.protocol.ids.api.multipart.util.ResponseUtil.*;

@Consumes({MediaType.MULTIPART_FORM_DATA})
@Produces({MediaType.MULTIPART_FORM_DATA})
@Path("/")
public class MultipartController extends AbstractMultipartController {

    private static final String HEADER = "header";
    private static final String PAYLOAD = "payload";
    private static final String PID = "pid";

    private static final String PAGE = "page";
    private static final String SIZE = "size";
    private static final String SORT = "sort";
    private static final String DATE_FROM = "date_from";
    private static final String DATE_TO = "date_to";

    private static final DateTimeFormatter dateParser = DateTimeFormatter.ofPattern("yyyy-MM-dd");

    private static final String LOG_ID = "InfrastructureController";

    private static final String MESSAGES_LOG_PID = "messages/log/{pid}";
    private static final String PROCESS_PID = "process/{pid}";
    private static final String MESSAGES_QUERY_PID = "messages/query/{pid}";

    public static final String JWT_TOKEN_FORMAT_IDS = "idsc:JWT";
    public static final String JWT_TOKEN_FORMAT_DSP = "https://w3id.org/idsa/code/JWT";

    public MultipartController(@NotNull Monitor monitor,
                               @NotNull IdsId connectorId,
                               @NotNull ObjectMapper objectMapper,
                               @NotNull DynamicAttributeTokenService tokenService,
                               @NotNull String idsWebhookAddress,
                               @NotNull List<Handler> multipartHandlers) {
        super(monitor, connectorId, objectMapper, tokenService, multipartHandlers, idsWebhookAddress);
    }

    @POST
    @Path(MESSAGES_LOG_PID)
    public Response logMessage(@PathParam(PID) String pid,
                               @FormDataParam(HEADER) InputStream headerInputStream,
                               @FormDataParam(PAYLOAD) String payload) {
        var response = validateRequest(pid, headerInputStream, MESSAGES_LOG_PID);
        if (response.fail())
            return response.getError();

        // Check if payload is missing
        if (payload == null) {
            monitor.severe(LOG_ID + ": Payload is missing");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        return processRequest(pid, response.getHeader(), payload);
    }

    @POST
    @Path(PROCESS_PID)
    public Response createProcess(@PathParam(PID) String pid,
                                  @FormDataParam(HEADER) InputStream headerInputStream,
                                  @FormDataParam(PAYLOAD) String payload) {

        var response = validateRequest(pid, headerInputStream, PROCESS_PID);
        if (response.fail())
            return response.getError();

        return processRequest(pid, response.getHeader(), payload);
    }

    @POST
    @Path(MESSAGES_QUERY_PID)
    public Response queryMessages(@PathParam(PID) String pid,
                                  @FormDataParam(HEADER) InputStream headerInputStream,
                                  @QueryParam(PAGE) String page,
                                  @QueryParam(SIZE) String size,
                                  @QueryParam(SORT) String sort,
                                  @QueryParam(DATE_FROM) String dateFrom,
                                  @QueryParam(DATE_TO) String dateTo) {

        var requestValidation = validateRequest(pid, headerInputStream, MESSAGES_QUERY_PID);
        if (requestValidation.fail())
            return requestValidation.getError();

        var paggingValidation = validatePagging(page, size, sort, dateTo, dateFrom);
        if (paggingValidation.fail())
            return paggingValidation.getError();

        return processRequest(pid, requestValidation.getHeader(), null, paggingValidation.getPagging());
    }

    RequestValidationResponse validateRequest(String pid, InputStream headerInputStream, String endpoint){



        // Check if pid is missing
        if (pid == null) {
            monitor.severe(LOG_ID + ": PID is missing");
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build());
        }

        // Check if header is missing
        if (headerInputStream == null) {
            monitor.severe(LOG_ID + ": Header is missing");
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build());
        }

        // Convert header to message
        Message header;
        try {
            header = objectMapper.readValue(headerInputStream, Message.class);
        } catch (Exception e) {
            monitor.severe(format(LOG_ID + ": Header parsing failed: %s", e.getMessage()));
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build());
        }


        if(endpoint == MESSAGES_LOG_PID){
            if(!(header instanceof LogMessage)){
                monitor.severe(format(LOG_ID + ": Wrong endpoint for message: %s", header.getClass().getSimpleName()));
                return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        } else if(endpoint == PROCESS_PID){
            if(!(header instanceof RequestMessage)){
                monitor.severe(format(LOG_ID + ": Wrong endpoint for message: %s", header.getClass().getSimpleName()));
                return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        } else if(endpoint == MESSAGES_QUERY_PID){
            if(!(header instanceof QueryMessageRequest)){
                monitor.severe(format(LOG_ID + ": Wrong endpoint for message: %s", header.getClass().getSimpleName()));
                return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        // Check if any required header field missing
        if (header.getId() == null
                || (header.getId() != null && StringUtils.isNullOrBlank(header.getId().toString()))
                || StringUtils.isNullOrBlank(header.getClass().getSimpleName())
                || StringUtils.isNullOrBlank(header.getModelVersion())
                || header.getIssued() == null
                || header.getIssuerConnector() == null
                || (header.getIssuerConnector() != null && StringUtils.isNullOrBlank(header.getIssuerConnector().toString()))
                || header.getSenderAgent() == null
                || (header.getSenderAgent() != null && StringUtils.isNullOrBlank(header.getSenderAgent().toString()))
        ) {
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(header, connectorId)))
                    .build());
        }

        // Check if security token is present
        var securityToken = header.getSecurityToken();
        if (securityToken == null || securityToken.getTokenValue() == null) {
            monitor.severe(LOG_ID + ": Token is missing in header");
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(notAuthenticated(header, connectorId)))
                    .build());
        }

        // Check the security token type
        var tokenFormat = securityToken.getTokenFormat().getId().toString();
        if (!isValid(tokenFormat)) {
            monitor.severe(LOG_ID + ": Invalid security token type: " + tokenFormat);
            return new RequestValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build());
        }

        // Validate DAT
        if (!validateToken(header)) {
            return new RequestValidationResponse(Response.status(Response.Status.FORBIDDEN)
                    .entity(createFormDataMultiPart(notAuthenticated(header, connectorId)))
                    .build());
        }

      return new RequestValidationResponse(header);
    }

    PaggingValidationResponse validatePagging(String page, String size, String sort, String dateTo, String dateFrom) {
        var builder = Pagging.Builder.newInstance();

        if (!StringUtils.isNullOrBlank(page)) {
            try {
                builder =  builder.page(Integer.parseInt(page));
            } catch (NumberFormatException e) {
                monitor.severe(LOG_ID + ": Invalid page number: " + page);
                return new PaggingValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        if (!StringUtils.isNullOrBlank(size)) {
            try {
                builder = builder.size(Integer.parseInt(size));
            } catch (NumberFormatException e) {
                monitor.severe(LOG_ID + ": Invalid page size: " + size);
                return new PaggingValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        if (sort != null) {
            try {
                builder = builder.sort(Pagging.Sort.valueOf(sort.toUpperCase()));
            } catch (IllegalArgumentException e) {
                monitor.severe(LOG_ID + ": Invalid sort: " + sort);
                return new PaggingValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        if (!StringUtils.isNullOrBlank(dateFrom)) {
            try {
                builder = builder.dateFrom(LocalDate.parse(dateFrom, dateParser));
            } catch (DateTimeParseException e) {
                monitor.severe(LOG_ID + ": Invalid dateFrom: " + dateFrom);
                return new PaggingValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        if (!StringUtils.isNullOrBlank(dateTo)) {
            try {
                builder = builder.dateTo(LocalDate.parse(dateTo, dateParser));
            } catch (DateTimeParseException e) {
                monitor.severe(LOG_ID + ": Invalid dateTo: " + dateTo);
                return new PaggingValidationResponse(Response.status(Response.Status.BAD_REQUEST)
                        .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                        .build());
            }
        }

        return new PaggingValidationResponse(builder.build());
    }

    Response processRequest(@NotNull String pid, @NotNull Message header, String payload, Pagging pagging) {

        // Build the multipart request
        var handlerRequest = HandlerRequest.Builder.newInstance()
                .pid(pid)
                .header(header)
                .payload(payload)
                .pagging(pagging)
                .build();

        // Send to handler processing
        MultipartResponse multipartResponse;
        try {
            multipartResponse = multipartHandlers.stream()
                    .filter(h -> h.canHandle(handlerRequest))
                    .findFirst()
                    .map(it -> it.handleRequest(handlerRequest))
                    .orElseGet(() -> MultipartResponse.Builder.newInstance()
                            .header(messageTypeNotSupported(header, connectorId))
                            .build()
                    );

        } catch (Exception e) {
            monitor.severe(LOG_ID + ": Error in message handler processing", e);
            return Response.status(Response.Status.INTERNAL_SERVER_ERROR)
                    .entity(createFormDataMultiPart(internalRecipientError(header, connectorId)))
                    .build();
        }

        // Get the response token
        if (!getResponseToken(header, multipartResponse)) {
            return Response.status(Response.Status.INTERNAL_SERVER_ERROR)
                    .entity(createFormDataMultiPart(internalRecipientError(header, connectorId)))
                    .build();
        }

        // Build the response
        if (multipartResponse.getHeader() instanceof RejectionMessage) {
            var rejectionMessage = (RejectionMessage) multipartResponse.getHeader();

            return Response.status(Response.Status.INTERNAL_SERVER_ERROR)
                    .entity(createFormDataMultiPart(createRejectionMessage(rejectionMessage.getRejectionReason(), header, connectorId)))
                    .build();
        } else {
            return Response.status(Response.Status.CREATED)
                    .entity(createFormDataMultiPart(multipartResponse.getHeader(), multipartResponse.getPayload()))
                    .build();
        }
    }

    Response processRequest(@NotNull String pid, @NotNull Message header, @NotNull String payload) {
        return processRequest(pid, header, payload, null);
    }

    private boolean validateToken(Message header) {

        var dynamicAttributeToken = new DynamicAttributeTokenBuilder().
                _tokenValue_(header.getSecurityToken().getTokenValue()).
                _tokenFormat_(de.fraunhofer.iais.eis.TokenFormat.JWT)
                .build();

        var verificationResult = tokenService
                .verifyDynamicAttributeToken(dynamicAttributeToken, header.getIssuerConnector(), idsWebhookAddress);

        if (verificationResult.failed()) {
            monitor.warning(format("MultipartController: Token validation failed %s", verificationResult.getFailure().getMessages()));
            return false;
        } else {
            return true;
        }
    }

    private boolean getResponseToken(Message header, MultipartResponse multipartResponse) {

        multipartResponse.getHeader().setSecurityToken(header.getSecurityToken());
        return true;

        /*if ((header.getRecipientConnector() == null) || (header.getRecipientConnector().isEmpty())) {
            monitor.severe(LOG_ID + ": Recipient connector is missing");
            return false;
        }

        var recipient = header.getRecipientConnector().get(0);
        var tokenResult = tokenService.obtainDynamicAttributeToken(recipient.toString());

        if (tokenResult.succeeded()) {
            var responseToken = tokenResult.getContent();
            SecurityToken securityToken = new SecurityToken();
            securityToken.setType(header.getSecurityToken().getType());
            securityToken.setTokenFormat(header.getSecurityToken().getTokenFormat());
            securityToken.setTokenValue(responseToken.getTokenValue());

            handlerResponse.getHeader().setSecurityToken(securityToken);
            return true;

        } else {
            monitor.severe(LOG_ID + ": Failed to get response token: " + tokenResult.getFailureDetail());
            return false;
        }*/
    }

    public static boolean isValid(String id) {
        return id.equals(JWT_TOKEN_FORMAT_IDS) || id.equals(JWT_TOKEN_FORMAT_DSP);
    }

}
