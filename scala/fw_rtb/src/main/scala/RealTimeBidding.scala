import scala.io.Source
import java.io.{FileNotFoundException, IOException}
import java.lang.String

object RealTimeBidding extends App {
  val fileName = "src/main/resources/download.txt"
  val encode = "UTF-8"
  try {
    val source = Source.fromFile(fileName, encode)
    val lines = source.getLines().toList.map(_.split(" "))
    var n = lines(0)(0).toInt
    var m = lines(0)(1).toInt
    var curLine = 1
    while (n != 0 || m != 0) {
      for (i <- curLine until curLine + n) {
        // TODO: DSP
        val ad = lines(i)(0)
        val bid = lines(i)(1).toInt
        val frequency_cap = lines(i)(2).toInt
        val limit = lines(i)(3).toInt
      }
      curLine += n
      for (j <- curLine until curLine + m) {
        // TODO: bid request
        val user = lines(j)(0)
        val price = lines(j)(1).toInt
      }
      curLine += m
      n = lines(curLine)(0).toInt
      m = lines(curLine)(1).toInt
      curLine += 1
    }
    source.close
  } catch {
    case e: FileNotFoundException => println(s"Couldn't find $fileName")
    case e: IOException           => println(s"Got an IOException $fileName")
  }

}
