package com.example.emailservice;


import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController("email/webhook/")
public class WebHook {

    private final EmailService emailService;

    public WebHook(EmailService emailService) {
        this.emailService = emailService;
    }

    @CrossOrigin("*")
    @GetMapping("start-recover")
    public String startRecover() {
        System.out.println("Get recover event");
        //toDo extract info from db
        emailService.sendMessage("maximboguns@gmail.com", "WARNING!!Recovery process is started", "test");
        return "call";
    }
}
