package com.example.emailservice.user;

public class UserMapper {
    public static User map(UserRequest userInfo) {
       User user = new User();
       user.setAccount(userInfo.getAccount());
       user.setEmail(userInfo.getEmail());
       return user;
    }
}
