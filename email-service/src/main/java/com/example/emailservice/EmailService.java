package com.example.emailservice;

import org.springframework.mail.SimpleMailMessage;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.stereotype.Service;

@Service
public class EmailService {

    private final JavaMailSender emailSender;

    public EmailService(JavaMailSender emailSender) {
        this.emailSender = emailSender;
    }
    public void sendMessage(
            String to, String subject, String text) {
        SimpleMailMessage message = new SimpleMailMessage();
        message.setFrom("wallemetabuild");
        message.setTo(to);
        message.setSubject(subject);
        message.setText("Some user activated funcation of recovery");
        emailSender.send(message);
    }
}
