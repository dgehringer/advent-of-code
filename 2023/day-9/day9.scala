
import scala.io.Source

def extrapolate(history: Array[Long]): BigInt =
  if history.filter(_ != 0).sum == 0 then 0 else history.last + extrapolate(history.tail.zip(history).map(_ - _))

@main
def main(): Unit =
  val histories = Source.fromFile("input.txt")
    .getLines
    .map(line => raw"-?\d+".r.findAllIn(line).map(_.toLong).toArray)
    .toList
  val partOne = histories.map(extrapolate).sum
  val partTwo = histories.map(h => extrapolate(h.reverse)).sum
  println(s"Print one: $partOne, Part Two: $partTwo")
