package main

// Testing normal types
type num int
type str string
type char rune
type maybe bool
type decimal float64

// Testing multiple types
type (
	some    int
	another float64
)

// Testing structs
type point struct {
	x, y int
}

type square struct {
	top_left     point
	bottom_right point
}


