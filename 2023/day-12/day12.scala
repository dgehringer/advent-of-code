
import scala.io.Source
import scala.util.Using
import scala.collection.mutable

type Record = (String, Vector[Int])

def parseInput(lines: Iterator[String]): List[Record] =
  lines.map(line =>
    val patternAndGroups = raw"\s+".r.split(line)
    (patternAndGroups(0), raw"\d+".r.findAllIn(patternAndGroups(1)).map(_.toInt).toVector)
  ).toList

def memoize[I, O](f: I => O): I => O = new mutable.HashMap[I, O]() {
  override def apply(key: I) = getOrElseUpdate(key, f(key))
}

extension (b: Boolean)
  def intValue: Int = if b then 1 else 0

val countPossiblities: (Record) => Long = memoize {
  case (pattern, blocks) =>
    if blocks.isEmpty then pattern.forall(_ != '#').intValue // 1 if no # in pattern
    else if blocks.sum > pattern.length then 0
    else if pattern.head == '.' then countPossiblities((pattern.tail, blocks))
    else
      val numNextTile =
        if pattern.head == '?' then countPossiblities((pattern.tail, blocks)) else 0
      val currentGroupLength = blocks.head
      val charAtCurrBlockEnd =
        if pattern.length > currentGroupLength
        then pattern(currentGroupLength) else '.'
      val numThisTile =
        if pattern.take(blocks.head).forall(_ != '.') && charAtCurrBlockEnd != '#'
        then countPossiblities((pattern.drop(currentGroupLength + 1), blocks.tail)) else 0 // skip the current block
      numThisTile + numNextTile
}

def unfoldRecord(multiplicity: Int)(record: Record): Record =
  val multipliedPattern = ((record._1 + '?') * multiplicity).init // add a ? between the patterns
  (multipliedPattern, (0 until multiplicity).foldLeft(Vector[Int]())((l, _) => l ++ record._2))

@main
def main(): Unit =
  val springRecords = Using(Source.fromFile("input.txt"))(src => parseInput(src.getLines())).get
  val partOne = springRecords.map(countPossiblities).sum
  val partTwo = springRecords.map(unfoldRecord(5) andThen countPossiblities).sum
  println(s"Part one: $partOne, Part two: $partTwo")

