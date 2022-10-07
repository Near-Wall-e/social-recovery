package com.example.emailservice.pagoda;


import com.example.emailservice.EmailService;
import com.example.emailservice.pagoda.domain.Alert;
import lombok.extern.log4j.Log4j;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@Slf4j
@RestController
public class AlertHook {

    private final EmailService emailService;

    public AlertHook(EmailService emailService) {
        this.emailService = emailService;
    }

    @PostMapping(value = "/email/webhook/recover/alert", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    public void start(@RequestBody Alert alert) {
        log.info("Got alert {}", alert);
        emailService.sendMessage("maximboguns@gmail.com", "WARNING!!Recovery process is started", "test");
    }

}
