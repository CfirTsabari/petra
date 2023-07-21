package defs_test

import (
	"testing"

	"petra/red_golang/pkg/defs"
)

func compareString(t *testing.T, got, want string) {
	t.Helper()

	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
}

func compareNumber(t *testing.T, got, want int) {
	t.Helper()

	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
}

func Test_name(t *testing.T) {
	t.Parallel()
	compareString(t, defs.Gabbro, "Gabbro")
	compareString(t, defs.Marble, "Marble")
	compareString(t, defs.Metamorphic, "Metamorphic")
	compareNumber(t, defs.ApplesCount, 236)
	compareNumber(t, defs.OrangesCount, 454588979794318)
}
