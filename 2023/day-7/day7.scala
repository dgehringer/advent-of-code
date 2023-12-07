
import scala.io.Source

type Card = Char
type Hand = String
type Frequencies = Vector[Int]

trait Classifier[T]:
  def classify(o: T): Int

def frequencyMap(h: Hand): Map[Char, Int] =
  h.foldLeft(Map[Char, Int]())((m, c) => m updated(c, m.getOrElse(c, 0) + 1))

def classFromFrequencies(amount: Frequencies): Int =
  if amount.length == 5 then 1 // highest card
  else if amount.length == 4 then 2 // pair
  else if amount.length == 3 then if amount contains 3 then 4 else 3 // a triple with two cards or two pairs
  // as we have sorted them by ascending frequency, we know if the second value's freq is a two, there are two pairs
  else if amount.length == 2 then if amount contains 4 then 6 else 5 // full-house or 4 of a kind
  else 7

given defaultClassifier: Classifier[Hand] with
  override def classify(h: Hand): Int = classFromFrequencies(frequencyMap(h).values.toVector.sorted)

given jokerClassifier: Classifier[Hand] with
  override def classify(o: Hand): Int =
    if o == "JJJJJ" then 7
    else
      val freqMap = frequencyMap(o)
      val jokerCount = freqMap.getOrElse('J', 0)
      val frequencies = (freqMap removed 'J').values.toVector.sorted
      classFromFrequencies(frequencies.init appended (frequencies.last + jokerCount))

given handOrdering(using cardOrd: Ordering[Card], classifier: Classifier[Hand]): Ordering[Hand] with
  override def compare(x: Hand, y: Hand): Int =
    val classificationCompare = classifier.classify(x) compare classifier.classify(y)
    if classificationCompare == 0 then
      val (a, b) = (x zip y).dropWhile(_ == _).head
      cardOrd.compare(a, b)
    else classificationCompare


def orderingFromSeq(ordStr: String): Ordering[Card] = new Ordering[Card]:
  override def compare(x: Card, y: Card): Int = ordStr.indexOf(x) compare ordStr.indexOf(y)

def cardsTotal(cards: List[(Hand, Int)])(using Ordering[Hand]): Int =
  cards
    .sortBy(_._1)
    .zipWithIndex
    .map((cardAndBid, rank) => cardAndBid._2 * (rank + 1))
    .sum

@main
def main(): Unit =
  val cards = Source.fromFile("input.txt").getLines
    .map(line =>
      val handAndBid = line.split(raw"\s+")
      (handAndBid(0), handAndBid(1).toInt)
    ).toList
  val partOne = cardsTotal(cards)(using handOrdering(using orderingFromSeq("23456789TJQKA"), defaultClassifier))
  val partTwo = cardsTotal(cards)(using handOrdering(using orderingFromSeq("J23456789TQKA"), jokerClassifier))
  println(s"Part one: $partOne, Part two: $partTwo")
