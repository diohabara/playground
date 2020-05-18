lazy val root = (project in file(".")).settings(
  name := "Flywheel RTB",
  scalaVersion := "2.12.11",
  scalacOptions ++= Seq("-deprecation", "-feature")
)
