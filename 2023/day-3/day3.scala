import scala.io.Source

type EngineMap = Vector[Vector[Char]]

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

extension (ls: List[String])
  def get(pos: (Int, Int)): Char = ls(pos._1)(pos._2)

def findNumbersAndSymbolPosition(engineMap: List[String], symbols: Set[Char]): List[(Int, (Int, Int))] =
  assert(engineMap.nonEmpty)
  val (h, w) = (engineMap.length, engineMap.head.length)

  val isSymbol = (c: Char) => symbols contains c
  val getAdjacentTiles = adjacentTiles(h, w)
  engineMap.zipWithIndex.flatMap((line, y) =>
    raw"\d+".r.findAllMatchIn(line).flatMap(m =>
      val hasSymbol = (m.start until m.end).flatMap(x =>
        getAdjacentTiles(y, x).find(pos => isSymbol(engineMap.get(pos)))
      )
      if hasSymbol.nonEmpty then Some((m.toString.toInt, hasSymbol.head)) else None
    )
  )

@main
def main(): Unit =
  val engineMap = Source.fromFile("input.txt").getLines.toList
  val symbols = engineMap.flatten.filterNot(c => c.isDigit || c == '.').toSet
  val numbers = findNumbersAndSymbolPosition(engineMap, symbols)
  val partOne = numbers.map(_._1).sum
  val partTwo = numbers
    .filter((_, pos) => engineMap.get(pos) == '*') // only those adjacent to gears
    .groupBy(_._2)
    .filter(_._2.length == 2)
    .map(_._2.map(_._1).product).sum
  println(s"Part one: $partOne, Part two: $partTwo")
