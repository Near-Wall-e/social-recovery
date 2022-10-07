package com.example.emailservice;


import org.springframework.web.bind.annotation.*;

@CrossOrigin("*")
@RestController("/email/")
public class EmailController {

    @PostMapping("email")
    public void addUserData(@RequestBody UserInfo userInfo) {
        //save to db
    }

    @GetMapping("test")
    public String test() {
        return "test";
    }
}
