package com.example.emailservice.pagoda.domain;

import com.fasterxml.jackson.annotation.JsonTypeName;

import static com.example.emailservice.pagoda.domain.EventType.*;

@JsonTypeName("cancel_recover")
public class CancelRecoverEvent extends RecoverEvent {

    public CancelRecoverEvent() {
    }

    public CancelRecoverEvent(String recoverPK, String account) {
        super(recoverPK, account);
    }

    @Override
    EventType getEvenType() {
        return CANCEL;
    }
}
