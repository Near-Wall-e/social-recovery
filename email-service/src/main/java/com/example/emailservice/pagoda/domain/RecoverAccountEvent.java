package com.example.emailservice.pagoda.domain;

import com.fasterxml.jackson.annotation.JsonTypeName;

import static com.example.emailservice.pagoda.domain.EventType.RECOVER;

@JsonTypeName("recover_account")
public class RecoverAccountEvent extends RecoverEvent {

    private String recoverer;

    public RecoverAccountEvent(String recoverer) {
        this.recoverer = recoverer;
    }

    public RecoverAccountEvent(String recoverPK, String account, String recoverer) {
        super(recoverPK, account);
        this.recoverer = recoverer;
    }

    @Override
    EventType getEvenType() {
        return RECOVER;
    }

}
