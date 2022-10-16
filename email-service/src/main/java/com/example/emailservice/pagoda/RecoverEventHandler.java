package com.example.emailservice.pagoda;

import com.example.emailservice.email.EmailService;
import com.example.emailservice.pagoda.domain.*;
import com.example.emailservice.user.User;
import com.example.emailservice.user.UserRepository;
import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;

import static com.example.emailservice.pagoda.domain.EventType.CANCEL;

@Service
@AllArgsConstructor
public class RecoverEventHandler implements EventHandler {

    private final EmailService emailService;
    private final UserRepository userRepository;

    @Override
    public void handel(Events events) {
        RecoverAccountEvent recoverEvent = (RecoverAccountEvent) events.getData();
        User user = userRepository.findByAccount(recoverEvent.getAccount());
        emailService.sendMessage(user.getEmail(), "Process recover is started", String.format("The process is started for user %s by %s", user.getAccount(), recoverEvent.getRecoverer()));
    }

    @Override
    public EventType getEvenType() {
        return CANCEL;
    }
}
