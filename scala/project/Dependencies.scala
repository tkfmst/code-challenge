import sbt._

object Versions {
  val Scala = "2.13.6"

  val Munit = "0.7.29"
}

object Dependencies {
  val coreDependencies = Seq()

  val leetcodeDependencies = Seq()

  val testDependencies = Seq(
    Libraries.munit,
  )
}

object Libraries {
  lazy val munit = "org.scalameta" %% "munit" % Versions.Munit % Test
}
