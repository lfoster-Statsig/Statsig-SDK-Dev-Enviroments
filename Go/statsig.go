package main

import (
	"fmt"

	"os"

	statsig "github.com/statsig-io/go-sdk"

	godotenv "github.com/joho/godotenv"
)

func main() {
	// Load .env from parent directory
	godotenv.Load("../.env")
	statsig.InitializeWithOptions(os.Getenv("SERVER_KEY"), &statsig.Options{Environment: statsig.Environment{Tier: "development"}})

	// Get the logged-in user's ID from environment variable
	userID := os.Getenv("USER")
	fmt.Println("User ID: " + userID)
	user := statsig.User{UserID: userID}
	feature := statsig.CheckGate(user, "new_feature_gate")

	status := "disabled"
	if feature {
		status = "enabled"
	}
	fmt.Println("Feature is " + status)

	statsig.LogEvent(statsig.Event{
		User: user,
		EventName: "app_started",
                Value: status,
		Metadata: nil,
	})

	// Shutdown to ensure all events are flushed before exiting
	statsig.Shutdown()
}