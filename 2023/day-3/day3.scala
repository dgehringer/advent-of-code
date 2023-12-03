
import scala.io.Source


type EngineMap = Vector[Vector[Char]]

def parseEngineMap(lines: Iterator[String]): EngineMap =
  lines.filterNot(_.isEmpty).map(_.toVector).toVector

@inline
def inBounds(v: Int, b: Int): Boolean = v >= 0 && v < b

@inline
def charsToNum(ch: Vector[Char]): Int = ch.mkString("").toInt

def adjacentTiles(h: Int, w: Int)(y: Int, x: Int): Seq[(Int, Int)] =
  for
    dy <- -1 to 1
    dx <- -1 to 1
    if inBounds(y + dy, h) && inBounds(x + dx, w)
  yield (y + dy, x + dx)


def adjacentNumbers(engineMap: EngineMap): (Int, Int) =
  val (h, w) = (engineMap.length, engineMap(0).length)
  val symbols = engineMap.flatten.filterNot(c => c.isDigit || c == '.').toSet
  val isGear = (pos: (Int, Int)) => engineMap(pos._1)(pos._2) == '*'
  val isSymbol = (c: Char) => symbols contains c
  val getAdjacentTiles = adjacentTiles(h, w)
  var (partNumbers, currNum, symbol: Option[(Int, Int)]) = (List[Int](), Vector[Char](), None)
  var gearRatios = Map[(Int, Int), List[Int]]().withDefaultValue(Nil)
  for
    y <- 0 until h
    x <- 0 until w
  do {
    val char = engineMap(y)(x)
    if char.isDigit then
      currNum = currNum appended char
      if symbol.isEmpty then
        symbol = getAdjacentTiles(y, x).find(pos => isSymbol(engineMap(pos._1)(pos._2)))
    if isSymbol(char) || char == '.' || x == w - 1 then
      if symbol.isDefined && currNum.nonEmpty then
        val number = charsToNum(currNum)
        partNumbers = number :: partNumbers
        if isGear(symbol.get) then
          gearRatios = gearRatios updated(symbol.get, number :: gearRatios(symbol.get))
      symbol = None
      currNum = Vector()
  }

  val summedGearRatios = gearRatios.filter(_._2.length == 2).map((_, nums) => nums.product).sum
  (partNumbers.reverse.sum, summedGearRatios)

@main
def main(): Unit =
  val engineMap = parseEngineMap(Source.fromFile("input.txt").getLines)
  val (partOne, partTwo) = adjacentNumbers(engineMap)
  println(s"Part one: $partOne, Part two: $partTwo")
