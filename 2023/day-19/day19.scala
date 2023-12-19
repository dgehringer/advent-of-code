
import scala.io.Source
import scala.util.Using
import scala.annotation.tailrec
import scala.collection.parallel.CollectionConverters._

type Part = Array[Int]
type Range = (Int, Int)
type Rule = (Char, Char, Int, String)
type RuleCollection = Map[String, (List[Rule], String)]

extension (r: Rule)
  def apply(p: Part): Option[String] =
    val value = p("xmas" indexOf r._1)
    val isFulfilled = if r._2 == '<' then value < r._3 else value > r._3
    if isFulfilled then Some(r._4) else None

def makeRule(ruleString: String): Rule =
  val comparator = ruleString.tail.head
  val value = raw"\d+".r.findFirstIn(ruleString).get.toInt
  val nextRule = ruleString.split(":").last
  (ruleString.head, comparator, value, nextRule)

def parseInput(string: String): (RuleCollection, Array[Part]) =
  val blocks = string.split("\n\n")
  val rules =
    blocks.head.split("\n").map(line =>
      val splitted = line.split("\\{")
      val (ruleName, rawRules) = (splitted.head, splitted.tail.head.init)
      val splittedRules = rawRules.split(",")
      val default = splittedRules.last
      val rules = splittedRules.init map makeRule
      (ruleName, (rules.toList, default))
    ).toMap
  val parts = blocks.tail.head.split("\n").map(line => raw"\d+".r.findAllIn(line).map(_.toInt).toArray)
  (rules, parts)

def processPart(rules: RuleCollection)(part: Part): Char =
  @tailrec
  def workflow(ruleName: String): String =
    if "RA" contains ruleName then ruleName
    else
      val (rls, default)  = rules(ruleName)
      rls.flatMap(_ apply part) match
        case Nil => workflow(default) // none of the rules was satisfied
        case x :: xs => workflow(x)
  workflow("in").head


def constrainRange(compare: String, value: Int, range: Range): Range =
  if compare.startsWith("<") then
    (range._1, range._2 min (if compare contains '=' then value else value - 1))
  else if compare.startsWith(">") then
    (range._1 max (if compare contains '=' then value else value + 1), range._2)
  else throw new IllegalArgumentException()

def findAllPossibleCombinations(rules: RuleCollection): BigInt =
  // group rules by their character add 0 and 4000 and finally sort them
  var initialRange = (1, 4000)
  type RangeEntry = Either[Rule, String]
  // this is a vector with each entry having holding four range -> one for each in "XMAS"
  val initialRanges = Vector(Vector(initialRange, initialRange, initialRange, initialRange))

  // given a character it updates the vector entry at the right position with the constrained range
  def constrainRangeEntry(char: Char, compare: String, value: Int, ranges: Seq[IndexedSeq[Range]]) =
    val index = "xmas".indexOf(char)
    ranges.map(rngs =>
      val (start, end) = rngs(index)
      if start > end then rngs else rngs.updated(index, constrainRange(compare, value, (start, end)))
    )

  def applyRanges(ranges: Seq[RangeEntry]): Seq[IndexedSeq[Range]] =
    ranges.head match
      case Right(ruleName) => ruleName match
        case "R" => Seq()
        case "A" => initialRanges
        case _ =>
          val (conditions, defaultRule) = rules(ruleName)
          applyRanges(conditions.map(Left(_)) appended Right(defaultRule))
      case Left((v, comp, value, next)) =>
        val conditionsHolds = constrainRangeEntry(v, comp.toString, value, applyRanges(Seq(Right(next))))
        val conditionsFails = constrainRangeEntry(v, if comp == '>' then "<=" else ">=", value, applyRanges(ranges.tail))
        conditionsHolds ++ conditionsFails

  applyRanges(Seq(Right("in")))
    .map(_.map((start, end) => BigInt(end - start + 1)).product) // we fold now each of the four ranges of
    .sum

@main
def main(): Unit =
  val (rules, parts) =
    Using(Source.fromFile("input.txt"))(src => parseInput(src.mkString)).get
  val processor = processPart(rules)
  val partOne = parts.filter(p => processor(p) == 'A').map(_.sum).sum
  val partTwo = findAllPossibleCombinations(rules)
  println(s"Part one: $partOne, Part two: $partTwo")
