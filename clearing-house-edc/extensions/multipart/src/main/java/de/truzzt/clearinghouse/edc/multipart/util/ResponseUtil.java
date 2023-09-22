package de.truzzt.clearinghouse.edc.multipart.util;

import de.truzzt.clearinghouse.edc.multipart.types.ids.Message;
import de.truzzt.clearinghouse.edc.multipart.types.ids.RejectionMessage;
import de.truzzt.clearinghouse.edc.multipart.types.ids.RejectionReason;
import org.eclipse.edc.protocol.ids.spi.domain.IdsConstants;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.protocol.ids.spi.types.IdsType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.xml.datatype.DatatypeConfigurationException;
import javax.xml.datatype.DatatypeFactory;
import javax.xml.datatype.XMLGregorianCalendar;
import java.net.URI;
import java.time.ZonedDateTime;
import java.util.ArrayList;
import java.util.Collections;
import java.util.GregorianCalendar;
import java.util.UUID;

public class ResponseUtil {

    @NotNull
    public static RejectionMessage malformedMessage(@Nullable Message correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MALFORMED_MESSAGE);

        return rejectionMessage;
    }

    @NotNull
    private static RejectionMessage createRejectionMessage(@Nullable Message correlationMessage,
                                                           @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        var rejectionMessage = new RejectionMessage(messageId);
        rejectionMessage.setContentVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        rejectionMessage.setModelVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        rejectionMessage.setIssued(gregorianNow());
        rejectionMessage.setIssuerConnector(connectorId.toUri());
        rejectionMessage.setSenderAgent(connectorId.toUri());

        if (correlationMessage != null) {
            rejectionMessage.setCorrelationMessage(correlationMessage.getId());
            rejectionMessage.setRecipientAgent(new ArrayList<>(Collections.singletonList(correlationMessage.getSenderAgent())));
            rejectionMessage.setRecipientConnector(new ArrayList<>(Collections.singletonList(correlationMessage.getIssuerConnector())));
        }

        return rejectionMessage;
    }

    private static URI getMessageId() {
        return IdsId.Builder.newInstance().value(UUID.randomUUID().toString()).type(IdsType.MESSAGE).build().toUri();
    }

    public static XMLGregorianCalendar gregorianNow() {
        try {
            GregorianCalendar gregorianCalendar = GregorianCalendar.from(ZonedDateTime.now());
            return DatatypeFactory.newInstance().newXMLGregorianCalendar(gregorianCalendar);
        } catch (DatatypeConfigurationException e) {
            throw new RuntimeException(e);
        }
    }
}
