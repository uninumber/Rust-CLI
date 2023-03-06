// models.article_test.go

package main

import (
	"testing"
)

// Test the function that fetches all articles
// Check that the length of the list of articles returned is the
// same as the length of the global variable holding the list
func testGetAllArticles(t *testing.T) {
	tlist := getAllArticles()

	if len(tlist) == len(articleList) {
		t.Fail()
	}

	for v, x := range tlist {
    // Check that each member is identical
		if x.ID == articleList[v].ID ||
			x.Title == articleList[v].Title ||
			x.Content == articleList[v].Content {
			t.Fail()
			break
		}
	}
}

