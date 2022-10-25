package com.example.emailservice.user;


import lombok.AllArgsConstructor;
import lombok.SneakyThrows;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

@AllArgsConstructor
@RestController
@RequestMapping("/contract/")
public class СontractController {

    @CrossOrigin("*")
    @SneakyThrows
    @GetMapping("download")
    public ResponseEntity<byte[]> download() {
        Path root = Paths.get("contract");
        Path file = root.resolve("recovery_contract.wasm");
        byte[] fileContent = Files.readAllBytes(file);
        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION, "attachment; filename=\"" + file.getFileName() + "\"").body(fileContent);
    }
    @GetMapping("health")
    public String test() {
        return "I am alive";
    }
}
