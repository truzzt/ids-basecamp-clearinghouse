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
package de.truzzt.clearinghouse.edc.util;

import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import de.truzzt.clearinghouse.edc.types.ids.RejectionMessage;
import de.truzzt.clearinghouse.edc.types.ids.RejectionReason;
import jakarta.ws.rs.core.MediaType;
import org.eclipse.edc.protocol.ids.spi.domain.IdsConstants;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.protocol.ids.spi.types.IdsType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import org.glassfish.jersey.media.multipart.FormDataBodyPart;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;

import javax.xml.datatype.DatatypeConfigurationException;
import javax.xml.datatype.DatatypeFactory;
import javax.xml.datatype.XMLGregorianCalendar;
import java.net.URI;
import java.time.ZonedDateTime;
import java.util.GregorianCalendar;
import java.util.UUID;

public class ResponseUtil {

    private static final String PROCESSED_NOTIFICATION_TYPE = "ids:MessageProcessedNotificationMessage";

    public static FormDataMultiPart createFormDataMultiPart(TypeManagerUtil typeManagerUtil,
                                                            String headerName,
                                                            Message headerValue,
                                                            String payloadName,
                                                            Object payloadValue) {
        var multiPart = createFormDataMultiPart(typeManagerUtil, headerName, headerValue);

        if (payloadValue != null) {
            multiPart.bodyPart(new FormDataBodyPart(payloadName, typeManagerUtil.toJson(payloadValue), MediaType.APPLICATION_JSON_TYPE));
        }

        return multiPart;
    }

    public static FormDataMultiPart createFormDataMultiPart(TypeManagerUtil typeManagerUtil, String headerName, Message headerValue) {
        var multiPart = new FormDataMultiPart();

        if (headerValue != null) {
            multiPart.bodyPart(new FormDataBodyPart(headerName, typeManagerUtil.toJson(headerValue), MediaType.APPLICATION_JSON_TYPE));
        }
        return multiPart;
    }

    public static HandlerResponse createMultipartResponse(@NotNull Message header, @NotNull Object payload) {
        return HandlerResponse.Builder.newInstance()
                .header(header)
                .payload(payload)
                .build();
    }

    public static Message messageProcessedNotification(@NotNull Message correlationMessage,
                                                       @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        Message message =  new Message(messageId);
        message.setContext(correlationMessage.getContext());
        message.setType(PROCESSED_NOTIFICATION_TYPE);
        message.setSecurityToken(correlationMessage.getSecurityToken());
        message.setIssuerConnector(connectorId.toUri());
        message.setModelVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        message.setIssued(gregorianNow());
        message.setSenderAgent(connectorId.toUri());

        return message;
    }

    @NotNull
    public static RejectionMessage notAuthenticated(@NotNull Message correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.NOT_AUTHENTICATED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage malformedMessage(@Nullable Message correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MALFORMED_MESSAGE);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage messageTypeNotSupported(@NotNull Message correlationMessage,
                                                           @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MESSAGE_TYPE_NOT_SUPPORTED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage internalRecipientError(@NotNull Message correlationMessage,
                                                 @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage =  createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.INTERNAL_RECIPIENT_ERROR);

        return rejectionMessage;
    }

    @NotNull
    private static RejectionMessage createRejectionMessage(@Nullable Message correlationMessage,
                                                           @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        var rejectionMessage = new RejectionMessage(messageId);
        rejectionMessage.setModelVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        rejectionMessage.setIssued(gregorianNow());
        rejectionMessage.setIssuerConnector(connectorId.toUri());
        rejectionMessage.setSenderAgent(connectorId.toUri());
        rejectionMessage.setCorrelationMessage(correlationMessage);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage createRejectionMessage(@NotNull RejectionReason reason,
                                                          @Nullable Message correlationMessage,
                                                          @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage =  createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(reason);

        return rejectionMessage;
    }

    private static URI getMessageId() {
        return IdsId.Builder.newInstance().value(UUID.randomUUID().toString()).type(IdsType.MESSAGE).build().toUri();
    }

    private static XMLGregorianCalendar gregorianNow() {
        try {
            GregorianCalendar gregorianCalendar = GregorianCalendar.from(ZonedDateTime.now());
            return DatatypeFactory.newInstance().newXMLGregorianCalendar(gregorianCalendar);
        } catch (DatatypeConfigurationException e) {
            throw new RuntimeException(e);
        }
    }
}
