// handlers.user.go

package main

import (
    "math/rand"
    "net/http"
    "strconv"

    "github.com/gin-gonic/gin"
)

func generateSessionToken() string {
    // We're using a random 16 character string as the session token
    // This is NOT a secure way of generating session tokens
    // DO NOT USE THIS IN PRODUCTION
    return strconv.FormatInt(rand.Int63(), 16)
}

func showRegistrationPage(c *gin.Context) {
    // Call the render function with the name of the template to render
    render(c, gin.H{
        "title": "Register"}, "register.tmpl")
}

func register(c *gin.Context) {
    // Obtain the POSTed username and password values
    username := c.PostForm("username")
    password := c.PostForm("password")

    if _, err := registerNewUser(username, password); err == nil {
        // If the user is created, set the token in a cookie and log the user in
        token := generateSessionToken()
        c.SetCookie("token", token, 3600, "", "", false, true)
        c.Set("is_logged_in", true)

        render(c, gin.H{
            "title": "Successful registration & Login"}, "login-successful.tmpl")

    } else {
        // If the username/password combination is invalid,
        // show the error message on the login page
        c.HTML(http.StatusBadRequest, "register.tmpl", gin.H{
            "ErrorTitle":   "Registration Failed",
            "ErrorMessage": err.Error()})

    }
}

func showLoginPage(c *gin.Context) {
    render(c, gin.H{
        "title": "Login",
    }, "login.tmpl")
}

func performLogin(c *gin.Context) {
    username := c.PostForm("username")
    password := c.PostForm("password")

    if isUserValid(username, password) {
        token := generateSessionToken()
        c.SetCookie("token", token, 3600, "", "", false, true)

        render(c, gin.H{
            "title": "Successful Login"}, "login-successful.tmpl")

    } else {
        c.HTML(http.StatusBadRequest, "login.tmpl", gin.H{
            "ErrorTitle":   "Login Failed",
            "ErrorMessage": "Invalid credentials provided"})
    }
}

func logout(c *gin.Context) {
    c.SetCookie("token", "", -1, "", "", false, true)

    c.Redirect(http.StatusTemporaryRedirect, "/")
}
