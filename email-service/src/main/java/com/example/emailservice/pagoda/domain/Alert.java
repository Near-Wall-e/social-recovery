package com.example.emailservice.pagoda.domain;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Data;

@Data
public class Alert {
    @JsonProperty("payload")
    private Payload payload;
    @Data
    public static class Payload {
        @JsonProperty("Events")
        private Events events;
    }
}
