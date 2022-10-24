package com.example.emailservice.pagoda;

import com.example.emailservice.email.EmailService;
import com.example.emailservice.pagoda.domain.CancelRecoverEvent;
import com.example.emailservice.pagoda.domain.EventType;
import com.example.emailservice.pagoda.domain.Events;
import com.example.emailservice.user.User;
import com.example.emailservice.user.UserRepository;
import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.Optional;

import static com.example.emailservice.pagoda.domain.EventType.CANCEL;

@Service
@AllArgsConstructor
public class CancelRecoverEventHandler implements EventHandler {

    private final EmailService emailService;
    private final UserRepository userRepository;

    @Override
    public void handel(Events events) {
        CancelRecoverEvent recoverEvent = (CancelRecoverEvent) events.getData();
        User user = userRepository.findByAccount(recoverEvent.getAccount());
        emailService.sendMessage(user.getEmail(), "Recover is canceled", String.format("The recover is canceled for user %s", user.getAccount()));
    }

    @Override
    public EventType getEvenType() {
        return CANCEL;
    }


}
