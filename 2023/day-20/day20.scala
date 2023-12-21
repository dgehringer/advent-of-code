

import scala.io.Source
import scala.util.Using
import scala.annotation.tailrec
import scala.collection.mutable
import scala.collection.immutable

type Pulse = Boolean
type Signal = (String, String, Pulse)
type Network = Map[String, List[String]]
type Conjunctions = Map[String, mutable.Map[String, Pulse]]
type FlipFlops = mutable.Map[String, Pulse]
type PulseNetwork = (Network, FlipFlops, Conjunctions)

extension (pulseNetwork: PulseNetwork)
  def network: Network = pulseNetwork._1
  def flipFlops: FlipFlops = pulseNetwork._2
  def conjunctions: Conjunctions = pulseNetwork._3

def parseInput(lines: List[String]): (Network, FlipFlops, Conjunctions) =
  val network = mutable.Map.empty[String, List[String]]
  val conjunctions = mutable.Map.empty[String, mutable.Map[String, Pulse]]
  val flipFlops: FlipFlops = mutable.Map()

  lines.foreach(line =>
    val nameAndDests = line.split(" -> ")
    val dests = nameAndDests.tail.head.split(", ").toList
    nameAndDests.head match
      case name if name.startsWith("%") =>
        network(name.tail) = dests
        flipFlops(name.tail) = false
      case name if name.startsWith("&") =>
        network(name.tail) = dests
        conjunctions(name.tail) = mutable.Map.empty[String, Pulse]
      case name =>
        network(name) = dests
  )
  network.foreach((src, dst) =>
    dst.filter(conjunctions.contains).foreach(conj =>
      conjunctions(conj)(src) = false
    )
  )
  (Map.from(network), flipFlops, Map.from(conjunctions))

def nextPulse(pulseNetwork: PulseNetwork)(src: String, dest: String, pulse: Pulse): Option[Pulse] =
  if pulseNetwork.conjunctions contains dest then
      pulseNetwork.conjunctions(dest)(src) = pulse
      Some(!pulseNetwork.conjunctions(dest).values.forall(_ == true))
  else if pulseNetwork.flipFlops contains dest then
    if pulse then None else
      val state = pulseNetwork.flipFlops(dest)
      pulseNetwork.flipFlops(dest) = !state
      Some(!state)
  else if pulseNetwork.network contains dest then Some(pulse)
  else None

def pushButton(pulseNetwork: PulseNetwork): (Long, Long) =
  val pulses = mutable.Queue(("button", "broadcaster", false))
  val genNextPulse = nextPulse(pulseNetwork)
  var hic, loc = 0L
  while pulses.nonEmpty do
    val sig @ (src, dst, pulse) = pulses.dequeue()
    if pulse then hic += 1 else loc += 1
    genNextPulse(src, dst, pulse) match
      case Some(p) => pulses enqueueAll pulseNetwork.network(dst).map((dst, _, p))
      case _ =>
  (hic, loc)

def solvePartOne(pulseNetwork: PulseNetwork, n: Int): Long =
  val (hic, loc) = (0 until n)
    .map(_ => pushButton(pulseNetwork))
    .fold((0L, 0L))((r, c) => (r._1 + c._1, r._2 + c._2))
  hic * loc

def lcm(factors: Seq[BigInt]): BigInt =
  @tailrec
  def gcd(a: BigInt, b: BigInt): BigInt = if (b == 0) a.abs else gcd(b, a % b)
  def lcm_(a: BigInt, b: BigInt) = (a * b).abs / gcd(a, b)
  factors.fold(1:BigInt)(lcm_)

def solvePartTwo(pulseNetwork: PulseNetwork): BigInt =
  // the rx element must have only one conjunction module as input
  // each of those mus again hava single inverter as input

  val rxInput = pulseNetwork.network.find(_._2 == List("rx")).get._1
  val inverterInputs = mutable.Set.from(pulseNetwork.network.filter(_._2 contains rxInput).keys)
  var cycle = 1
  val cycleNums = mutable.Set.empty[BigInt]
  println(inverterInputs)
  while inverterInputs.nonEmpty do
    val pulses = mutable.Queue(("button", "broadcaster", false))
    while pulses.nonEmpty do
      val (src, dst, pulse) = pulses.dequeue()
      if !pulse && inverterInputs.contains(dst) then
        cycleNums.add(cycle)
        inverterInputs.remove(dst)
      nextPulse(pulseNetwork)(src, dst, pulse) match
        case Some(p) => pulses enqueueAll pulseNetwork.network(dst).map((dst, _, p))
        case _ =>
    cycle += 1
  lcm(cycleNums.toSeq)

@main
def main(): Unit =
  val lines =
    Using(Source.fromFile("input.txt"))(src => src.getLines().toList).get
  val partOne = solvePartOne(parseInput(lines), 1000)
  val partTwo = solvePartTwo(parseInput(lines))
  println(s"Part one: $partOne, Part two: $partTwo")
