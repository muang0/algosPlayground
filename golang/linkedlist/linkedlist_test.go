package linkedlist

import (
	"io/ioutil"
	"os"
	"strings"
	"testing"
)

func TestPrint(t *testing.T) {
	n3 := node{val: 3, next: nil}
	n2 := node{val: 12, next: &n3}
	n1 := node{val: 8, next: &n2}
	ll := linkedlist{head: &n1}
	// capture stdout of print receiver function
	// https://stackoverflow.com/questions/10473800/in-go-how-do-i-capture-stdout-of-a-function-into-a-string
	rescueStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w
	ll.print()
	w.Close()
	out, _ := ioutil.ReadAll(r)
	os.Stdout = rescueStdout
	outstr := strings.TrimSuffix(string(out), "\n") // trim newline from function stdout
	expectedstr := "[8 12 3]"
	if outstr != expectedstr {
		t.Errorf("expected %v but got %v", expectedstr, outstr)
	}
}
