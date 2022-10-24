package com.example.emailservice.user;


import lombok.AllArgsConstructor;
import lombok.SneakyThrows;
import org.springframework.core.io.UrlResource;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.core.io.Resource;

import java.nio.file.Path;
import java.nio.file.Paths;

@AllArgsConstructor
@RestController
@RequestMapping("/contract/")
public class СontractController {

    private final UserRepository userRepository;

    @CrossOrigin("*")
    @SneakyThrows
    @GetMapping("download")
    public ResponseEntity<Resource> download() {
        Path root = Paths.get("contract");
        Path file = root.resolve("recovery_contract.wasm");
        Resource resource = new UrlResource(file.toUri());
        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION, "attachment; filename=\"" + file.getFileName() + "\"").body(resource);
    }

    @GetMapping("health")
    public String test() {
        return "I am alive";
    }
}
