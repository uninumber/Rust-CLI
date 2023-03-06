
// main.go

package main

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

var router *gin.Engine

func main() {
 //  	r := gin.Default()
	// r.GET("/", func(c *gin.Context) {
 //    c.HTML(http.StatusOK, "main.tmpl", "something")
	// })
	// r.Run() // listen and serve on 0.0.0.0:8080

  router = gin.Default()

  router.LoadHTMLGlob("./templates/*")

  router.Static("/css", "./css/")

  initializeRoutes()

	router.Run()


	// router.GET("/", func(c *gin.Context) {
	//
	// 	c.HTML(http.StatusOK, "main.tmpl", gin.H{
	//
	// 		"title": "Main website",
	//
	// 	})
	//
	// })
	// router.GET("/login", func(c *gin.Context) {
	//
	// 	c.HTML(http.StatusOK, "login.tmpl", gin.H{
	//
	// 		"title": "login page",
	//
	// 	})
	//
	// })
}

func render(c *gin.Context, data gin.H, templateName string) {

  switch c.Request.Header.Get("Accept") {
  case "application/json":
    c.JSON(http.StatusOK, data["payload"])

  case "application/xml":
    c.XML(http.StatusOK,  data["payload"])
  
  default: 
    c.HTML(http.StatusOK, templateName, data)
  }
}

