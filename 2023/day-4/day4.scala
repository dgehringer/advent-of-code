
import scala.io.Source
import scala.annotation.tailrec

type Counter = Map[Int, Int]

extension (c: Counter)
  def addMany(cardid: Int, amount: Int): Counter = c.updated(cardid, c.getOrElse(cardid, 0) + amount)
  
  def add(cardid: Int): Counter = addMany(cardid, 1)

case class Card(id: Int, numbers: Set[Int], pool: Set[Int]):
  private def winningNumbers: Int = pool.intersect(numbers).size

  def score: Int = 1 << (winningNumbers - 1)

  def winningIDs: Iterable[Int] = (id + 1 to id + winningNumbers)

def parseCard(line: String): Card =
  val allNumbers = raw"\d+|\|".r.findAllIn(line).toList
  val numbersAndPool = allNumbers.tail
  val pipeIndex = numbersAndPool.indexOf("|")
  val numbers = numbersAndPool.slice(0, pipeIndex).map(_.toInt).toSet
  val pool = numbersAndPool.slice(pipeIndex + 1, numbersAndPool.length).map(_.toInt).toSet
  Card(allNumbers.head.toInt, numbers, pool)

def playGame(cards: Map[Int, Card]): Counter =
  val maxCardId = cards.keys.max

  @tailrec
  def play(cid: Int, counter: Counter): Counter =
    if cid > maxCardId then counter
    else
      val cardAmount = counter.getOrElse(cid, 0) + 1
      val newCards = cards(cid).winningIDs.foldLeft(counter add cid)(_.addMany(_, cardAmount))
      play(cid + 1, newCards)

  play(cards.keys.min, Map())

@main
def main(): Unit =
  val cards = Source
    .fromFile("input.txt")
    .getLines
    .map(parseCard)
    .map(c => (c.id, c))
    .toMap
  println(s"Part one: ${cards.values.map(_.score).sum}, Part two: ${playGame(cards).values.sum}")