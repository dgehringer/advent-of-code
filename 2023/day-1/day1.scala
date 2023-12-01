import scala.io.Source

val lines = Source.fromResource("input.txt").getLines.toList

def firstAndLastDigit(digits: Iterable[Int]): Int =
  if digits.isEmpty then 0
  else digits.head * 10 + digits.last


val partOne = lines.map(line =>
  firstAndLastDigit(line.flatMap(_.toString.toIntOption))
).sum

val digitNames = List("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
val digits =
  (digitNames zip (1 to digitNames.length)).toMap ++ ((1 to 9).map(d => (d.toString, d))).toMap


def findDigits(line: String): Iterable[Int] =
  def findDigitAt(pos: Int) = digits.keySet.find(line.startsWith(_, pos))
  line.indices flatMap findDigitAt map digits

val partTwo = (lines map (findDigits andThen firstAndLastDigit)).sum