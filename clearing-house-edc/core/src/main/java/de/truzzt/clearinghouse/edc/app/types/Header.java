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
package de.truzzt.clearinghouse.edc.app.types;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;
import de.fraunhofer.iais.eis.*;
import org.eclipse.edc.spi.EdcException;
import org.jetbrains.annotations.NotNull;

import javax.xml.datatype.XMLGregorianCalendar;
import java.net.URI;
import java.util.Objects;

public class Header {

    @JsonProperty("@context")
    @NotNull
    private final Context context;

    @JsonProperty("@id")
    @NotNull
    private final String id;

    @JsonProperty("@type")
    @NotNull
    private final String type;

    @JsonProperty("securityToken")
    @NotNull
    private final SecurityToken securityToken;

    @JsonProperty("issuerConnector")
    @NotNull
    private final String issuerConnector;

    @JsonProperty("modelVersion")
    @NotNull
    private final String modelVersion;

    @JsonFormat(shape = JsonFormat.Shape.STRING, pattern = Constants.IDS_TIMESTAMP_FORMAT)
    @NotNull
    @JsonProperty("issued")
    private final XMLGregorianCalendar issued;

    @JsonProperty("senderAgent")
    @NotNull
    private final String senderAgent;

    private Header(@NotNull String id,
                   @NotNull String type,
                   @NotNull SecurityToken securityToken,
                   @NotNull String issuerConnector,
                   @NotNull String modelVersion,
                   @NotNull XMLGregorianCalendar issued,
                   @NotNull String senderAgent) {
        this.context = new Context();
        this.id = id;
        this.type = type;
        this.securityToken = securityToken;
        this.issuerConnector = issuerConnector;
        this.modelVersion = modelVersion;
        this.issued = issued;
        this.senderAgent = senderAgent;
    }

    public Context getContext() {
        return context;
    }

    public String getId() {
        return id;
    }

    public String getType() {
        return type;
    }

    public SecurityToken getSecurityToken() {
        return securityToken;
    }

    public String getIssuerConnector() {
        return issuerConnector;
    }

    public String getModelVersion() {
        return modelVersion;
    }

    public XMLGregorianCalendar getIssued() {
        return issued;
    }

    public String getSenderAgent() {
        return senderAgent;
    }

    public static class Builder {

        private String id;
        private String type;
        private SecurityToken securityToken;
        private String issuerConnector;
        private String modelVersion;
        private XMLGregorianCalendar issued;
        private String senderAgent;

        private Builder() {
        }

        public static Builder newInstance() {
            return new Builder();
        }

        public Builder id(@NotNull URI id) {
            this.id = id.toString();
            return this;
        }

        public Builder type(@NotNull Message message) {
            if (message instanceof RequestMessageImpl)
                this.type = Constants.IDS_TYPE_PREFIX + RequestMessage.class.getSimpleName();
            else if (message instanceof LogMessageImpl)
                this.type = Constants.IDS_TYPE_PREFIX + LogMessage.class.getSimpleName();
            else if (message instanceof QueryMessageImpl)
                this.type = Constants.IDS_TYPE_PREFIX + QueryMessage.class.getSimpleName();
            else
                throw new EdcException("Unsupported Message Type:" + message.getClass().getSimpleName());

            return this;
        }

        public Builder securityToken(@NotNull SecurityToken securityToken) {
            this.securityToken = securityToken;
            return this;
        }

        public Builder issuerConnector(@NotNull URI issuerConnector) {
            this.issuerConnector = issuerConnector.toString();
            return this;
        }

        public Builder modelVersion(@NotNull String modelVersion) {
            this.modelVersion = modelVersion;
            return this;
        }

        public Builder issued(@NotNull XMLGregorianCalendar issued) {
            this.issued = issued;
            return this;
        }

        public Builder senderAgent(@NotNull URI senderAgent) {
            this.senderAgent = senderAgent.toString();
            return this;
        }

        public Header build() {
            Objects.requireNonNull(id, "Logging message request header id is null.");
            Objects.requireNonNull(type, "Logging message request header type is null.");
            Objects.requireNonNull(securityToken, "Logging message request header security token is null.");

            Objects.requireNonNull(issuerConnector, "Logging message request header issuer connector is null.");
            Objects.requireNonNull(modelVersion, "Logging message request header model version is null.");
            Objects.requireNonNull(issued, "Logging message request header issued is null.");
            Objects.requireNonNull(senderAgent, "Logging message request header sender agent is null.");

            return new Header(id, type, securityToken, issuerConnector, modelVersion, issued, senderAgent);
        }
    }
}

