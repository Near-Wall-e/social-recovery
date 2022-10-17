package com.example.emailservice.user;


import lombok.AllArgsConstructor;
import org.springframework.web.bind.annotation.*;

@AllArgsConstructor
@CrossOrigin("*")
@RestController
@RequestMapping("/user/")
public class UserController {

    private final UserRepository userRepository;

    @PostMapping
    public void add(@RequestBody UserRequest userRequest) {
        userRepository.save(UserMapper.map(userRequest));
    }

    @GetMapping("health")
    public String test() {
        return "I am alive";
    }
}
