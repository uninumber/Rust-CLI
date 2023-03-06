package main

import (
  "errors" 
  "strings"
)

type user struct {
  Username string `json:"username"`
  Password string `json:"-"`
}

var userList = []user {
  {Username: "user1", Password: "pass1"},
  {Username: "user2", Password: "pass2"},
  {Username: "user3", Password: "pass3"},
}

// Register a new user with the given username and password
func registerNewUser(username, password string) (*user, error) {
    if strings.TrimSpace(password) == "" {
        return nil, errors.New("The password can't be empty")
    } else if !isUsernameAvailable(username) {
        return nil, errors.New("The username isn't available")
    }

    u := user{Username: username, Password: password}

    userList = append(userList, u)

    return &u, nil
}

// Check if the supplied username is available
func isUsernameAvailable(username string) bool {
    for _, u := range userList {
        if u.Username == username {
            return false
        }
    }
    return true
}

func isUserValid(username, password string) bool {
    for _, u := range userList {
        if u.Username == username && u.Password == password {
            return true
        }
    }
    return false
}

