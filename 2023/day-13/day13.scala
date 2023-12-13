
import scala.io.Source
import scala.util.Using

type Reflection = (Int, Int)
type Pattern = Array[Array[Char]]

def parseInput(str: String): List[Pattern] =
  str
    .split("\n\n")
    .map(block => block.split("\n").map(_.toArray))
    .toList

def findReflection(fixSmugde: Boolean)(p: Pattern): Int =
  val width = p.head.length
  val axesAndErrors =
    p.indices zip p.indices.tail map {
      case (start, stop) =>
        val numErrors =
          (for x <- 0 until width
               (up, down) <- (0 to start).reverse zip (stop until p.length)
               if p(up)(x) != p(down)(x)
              yield 1).sum
        (stop, numErrors)
    }
  axesAndErrors.find(_._2 == (if fixSmugde then 1 else 0)).getOrElse((0, 0))._1

def summarize(fixSmudge: Boolean)(p: Pattern): Long =
  val getReflection = findReflection(fixSmudge)
  getReflection(p) * 100L + getReflection(p.transpose)

@main
def main(): Unit =
  val patterns = Using(Source.fromFile("input.txt"))(src => parseInput(src.mkString)).get
  val partOne = patterns.map(summarize(false)).sum
  val partTwo = patterns.map(summarize(true)).sum
  println(s"Part one: $partOne, Part two: $partTwo")
