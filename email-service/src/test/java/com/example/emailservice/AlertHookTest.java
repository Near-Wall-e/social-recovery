package com.example.emailservice;

import com.example.emailservice.pagoda.AlertHook;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.context.junit4.SpringRunner;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import org.springframework.web.context.WebApplicationContext;

import java.nio.file.Files;
import java.nio.file.Paths;

import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@RunWith(SpringRunner.class)
@WebMvcTest(AlertHook.class)
class AlertHookTest {
    @MockBean
    private EmailService emailService;
    @Autowired
    private MockMvc mvc;
    @Autowired
    private WebApplicationContext webApplicationContext;
    @Test
    void shouldReturnDefaultMessage() throws Exception {
         mvc.perform(MockMvcRequestBuilders
                        .post("/email/webhook/recover/alert")
                        .content(Files.readString(Paths.get("src/test/resources/cancel_recover.json")))
                        .contentType(MediaType.APPLICATION_JSON))
                .andExpect(status().isOk());
    }
}