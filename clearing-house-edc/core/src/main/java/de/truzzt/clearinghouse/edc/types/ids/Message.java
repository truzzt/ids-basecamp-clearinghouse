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
package de.truzzt.clearinghouse.edc.types.ids;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;

import javax.xml.datatype.XMLGregorianCalendar;
import java.net.URI;
import java.util.List;

public class Message {

    @JsonProperty("@context")
    @NotNull
    private Context context;

    @JsonProperty("@id")
    @NotNull
    private URI id;

    @JsonProperty("@type")
    @NotNull
    private String type;

    @JsonProperty("ids:securityToken")
    @NotNull
    private SecurityToken securityToken;

    @JsonProperty("ids:issuerConnector")
    @NotNull
    private URI issuerConnector;

    @JsonProperty("ids:recipientConnector")
    @NotNull
    private List<URI> recipientConnector;

    @JsonProperty("ids:modelVersion")
    @NotNull
    String modelVersion;

    @JsonProperty("ids:issued")
    @JsonFormat(shape = JsonFormat.Shape.STRING, pattern = "yyyy-MM-dd'T'HH:mm:ss.SSS")
    @NotNull
    XMLGregorianCalendar issued;

    @JsonProperty("ids:senderAgent")
    @NotNull
    private URI senderAgent;

    @JsonProperty("ids:correlationMessage")
    private Message correlationMessage;

    public Message() {
    }

    public Message(URI id) {
        this.id = id;
    }

    public URI getId() {
        return id;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    public URI getIssuerConnector() {
        return issuerConnector;
    }

    public void setIssuerConnector(URI issuerConnector) {
        this.issuerConnector = issuerConnector;
    }

    public List<URI> getRecipientConnector() {
        return recipientConnector;
    }

    public void setRecipientConnector(List<URI> recipientConnector) {
        this.recipientConnector = recipientConnector;
    }

    public String getModelVersion() {
        return modelVersion;
    }

    public void setModelVersion(String modelVersion) {
        this.modelVersion = modelVersion;
    }

    public XMLGregorianCalendar getIssued() {
        return issued;
    }

    public void setIssued(XMLGregorianCalendar issued) {
        this.issued = issued;
    }

    public SecurityToken getSecurityToken() {
        return securityToken;
    }

    public void setSecurityToken(SecurityToken securityToken) {
        this.securityToken = securityToken;
    }

    public URI getSenderAgent() {
        return senderAgent;
    }

    public void setSenderAgent(URI senderAgent) {
        this.senderAgent = senderAgent;
    }

    public Context getContext() {
        return context;
    }

    public void setContext(Context context) {
        this.context = context;
    }

    public Message getCorrelationMessage() {
        return correlationMessage;
    }

    public void setCorrelationMessage(Message correlationMessage) {
        this.correlationMessage = correlationMessage;
    }
}
