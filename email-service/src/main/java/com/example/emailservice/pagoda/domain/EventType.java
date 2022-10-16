package com.example.emailservice.pagoda.domain;

import lombok.Data;

import java.util.stream.Stream;

public enum EventType {
    RECOVER("recover"),
    CANCEL("cancel_recover");

    private final String name;

    EventType(String name) {
        this.name = name;
    }

    public static EventType from(String value) {
        return Stream.of(values())
                .filter(eventType -> eventType.name.equals(value))
                .findFirst()
                .orElseThrow(IllegalAccessError::new);
    }
}
