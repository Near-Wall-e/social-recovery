package com.example.emailservice.pagoda;


import com.example.emailservice.email.EmailService;
import com.example.emailservice.pagoda.domain.Alert;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@Slf4j
@RestController
public class AlertHook {

    private final EmailService emailService;
    private final AlertService alertService;
    public AlertHook(EmailService emailService, AlertService alertService) {
        this.emailService = emailService;
        this.alertService = alertService;
    }

    @PostMapping(value = "/email/webhook/recover/alert", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    public void start(@RequestBody Alert alert) {
        alertService.handel(alert);
    }

}
