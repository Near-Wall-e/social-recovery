package com.example.emailservice.pagoda;

import com.example.emailservice.pagoda.domain.EventType;
import com.example.emailservice.pagoda.domain.Events;

public interface EventHandler {

     void handel(Events events);

     EventType getEvenType();
}
