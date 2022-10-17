package com.example.emailservice.pagoda;

import com.example.emailservice.pagoda.domain.Alert;
import com.example.emailservice.pagoda.domain.EventType;
import com.example.emailservice.pagoda.domain.Events;
import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@AllArgsConstructor
@Service
public class AlertService {

    private final List<EventHandler> eventHandlers;

    public void handel(Alert alert) {
        Events events = Optional.ofNullable(alert).map(Alert::getPayload).map(Alert.Payload::getEvents).orElse(null);
        eventHandlers.stream().filter(eventHandler -> eventHandler.getEvenType() == Optional.ofNullable(events).map(Events::getEvent).map(EventType::from).orElse(null))
                .findFirst().ifPresent(eventHandler -> eventHandler.handel(events));
    }
}
