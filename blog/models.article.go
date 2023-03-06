// models.article.go

package main

type article struct {
  ID int `json:"id"`
  Title string `json:"title"`
  Content string `json:"content"`
}

// For this demo, we're storing the article list in memory
// In a real application, this list will most likely be fetched
// from a database or from static files
var articleList = []article {
  article {ID: 1, Title: "Hacker News", Content: "Today we observed the most expensive attack in digital history." },
  article {ID: 2, Title: "QuantumComputers", Content: "Today hundreds QuantumComputers expoded the same time." },
}

// Return a list of all the articles
func getAllArticles() []article {
  return articleList
}
