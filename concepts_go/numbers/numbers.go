package main

import (
	crand "crypto/rand"
	"fmt"
	"math"
	"math/big"
	mrand "math/rand"
	"time"
)

func rand() {
	// math.rand is psuedo random (deterministic if same seed is used)
	// not safe for cryptographic use but ok for games, simulations, etc. where randomness is not too critical
	// make rand generation non-deterministic on each run
	// Seed func deprecated in 1.20.0
	// mrand.Seed(time.Now().UnixNano())
	mrand.New(mrand.NewSource(time.Now().UnixNano()))
	fmt.Println(mrand.Intn(100))
	fmt.Println(mrand.Float64())

	// crypto rand generates from non deterministic such as os's crypto ran gen
	// suitable for cryptography work
	max := new(big.Int)                                                  // 0
	max.Exp(big.NewInt(2), big.NewInt(130), nil).Sub(max, big.NewInt(1)) // 2^130 - 1
	fmt.Println("Big Int:", max)
	bigInt, err := crand.Int(crand.Reader, max)
	if err != nil {
		panic(err)
	}
	fmt.Println(bigInt)
}

func mathOps() {
	// Abs returns the absolute value of x.
	fmt.Println(math.Abs(-10)) // Output: 10
	fmt.Println(math.Abs(10))  // Output: 10

	// Round returns the nearest integer, rounding half away from zero.
	fmt.Println(math.Round(1.4))  // Output: 1
	fmt.Println(math.Round(1.5))  // Output: 2
	fmt.Println(math.Round(-1.4)) // Output: -1
	fmt.Println(math.Round(-1.5)) // Output: -2

	// Ceil returns the least integer value greater than or equal to x.
	fmt.Println(math.Ceil(1.1))  // Output: 2
	fmt.Println(math.Ceil(-1.1)) // Output: -1

	// Floor returns the greatest integer value less than or equal to x.
	fmt.Println(math.Floor(1.9))  // Output: 1
	fmt.Println(math.Floor(-1.9)) // Output: -2

	// Sqrt returns the square root of x.
	fmt.Println(math.Sqrt(16)) // Output: 4

	// Pow returns x**y, the base-x exponential of y.
	fmt.Println(math.Pow(2, 3)) // Output: 8

	// Sin returns the sine of the radian argument x.
	fmt.Println(math.Sin(math.Pi / 2)) // Output: 1

	// Cos returns the cosine of the radian argument x.
	fmt.Println(math.Cos(math.Pi)) // Output: -1

	// Tan returns the tangent of the radian argument x.
	fmt.Println(math.Tan(math.Pi / 4)) // Output: 1

	// Log returns the natural logarithm of x.
	fmt.Println(math.Log(math.E)) // Output: 1

	// Exp returns e**x, the base-e exponential of x.
	fmt.Println(math.Exp(1)) // Output: 2.718281828459045

	fmt.Println(math.Min(2, 3.3))                   // 2
	fmt.Println(math.Max(100, 2))                   // 100
	fmt.Println(math.Max(math.Inf(0), math.Inf(1))) // +Inf
	fmt.Println(math.MaxInt8, math.MinInt8, math.MaxUint8, math.MaxFloat64, math.Inf(10))
}

func main() {
	mathOps()
	//rand()
}
