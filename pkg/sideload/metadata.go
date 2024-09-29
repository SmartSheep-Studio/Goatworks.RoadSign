package sideload

import (
	roadsign "git.solsynth.dev/goatworks/roadsign/pkg"
	"github.com/gofiber/fiber/v2"
)

func getMetadata(c *fiber.Ctx) error {
	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"server":  "RoadSign",
		"version": roadsign.AppVersion,
	})
}
