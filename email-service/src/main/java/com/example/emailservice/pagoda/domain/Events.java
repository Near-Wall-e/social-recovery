package com.example.emailservice.pagoda.domain;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import lombok.Data;

import static com.fasterxml.jackson.annotation.JsonTypeInfo.As.EXTERNAL_PROPERTY;
import static com.fasterxml.jackson.annotation.JsonTypeInfo.Id.NAME;

@JsonIgnoreProperties(ignoreUnknown = true)
@Data
public class Events {
    @JsonProperty("block_hash")
    private String blockHash;
    @JsonProperty("receipt_id")
    private String receiptId;
    @JsonProperty("transaction_hash")
    private String transactionHash;
    private String event;
    private String standard;
    private String version;
    @JsonTypeInfo(use = NAME, property = "event", include = EXTERNAL_PROPERTY)
    @JsonSubTypes(value = {
            @JsonSubTypes.Type(value = CancelRecoverEvent.class, name = "cancel_recover"),
            @JsonSubTypes.Type(value = RecoverAccountEvent.class, name = "recover_account")
    })
    private RecoverEvent data;
}
