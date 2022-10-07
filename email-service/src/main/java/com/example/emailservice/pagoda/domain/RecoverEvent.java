package com.example.emailservice.pagoda.domain;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Data;
import lombok.NoArgsConstructor;

@NoArgsConstructor
@Data
public abstract class RecoverEvent {
    @JsonProperty("recover_pk")
    private String recoverPK;
    private String account;

    public RecoverEvent(String recoverPK, String account) {
        this.recoverPK = recoverPK;
        this.account = account;
    }

    abstract EventType getEvenType();
}
