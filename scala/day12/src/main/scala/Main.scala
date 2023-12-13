import zio._
import zio.nio.file.{Files, Path}
import zio.stream.ZStream

object Main extends ZIOAppDefault {
  private val num_re = """\d+""".r

  private final case class ParsedLine(firstPart: List[Char], nums: List[Int])

  private def loadFile(filename: Path) =
    Files
      .lines(filename)
      .map { line =>
        val parts = line.split(' ')

        val nums = num_re.findAllIn(parts(1)).map(_.toInt).toList

        ParsedLine(parts.head.toList, nums)
      }

  private def countValidSolutions(line: ParsedLine) = {
    var memo = Map.empty[(List[Char], List[Int]), Long]

    def ret(s: List[Char], n: List[Int], a: Long) = {
      memo += ((s, n) -> a)
      a
    }

    def loop(remStr: List[Char], remNums: List[Int]): Long =
      memo.get((remStr, remNums)) match {
        case Some(a) => a
        case None =>
          if ((remStr.isEmpty || !remStr.contains('#')) && remNums.isEmpty) 1
          else if (remStr.isEmpty || remNums.isEmpty) 0
          else
            remStr.head match {
              case '.' => loop(remStr.tail, remNums)

              case '?' =>
                val a = loop('.' :: remStr.tail, remNums)
                val b = loop('#' :: remStr.tail, remNums)
                ret(remStr, remNums, a + b)

              case '#' =>
                val thisNum = remNums.head
                val canFit = remStr.lengthCompare(thisNum) >= 0 && !remStr
                  .take(thisNum)
                  .contains('.') && !remStr.lift(thisNum).contains('#')

                if (canFit) {
                  val tmpNext = remStr.drop(thisNum)
                  val next =
                    if (tmpNext.headOption.contains('?')) '.' :: tmpNext.tail
                    else tmpNext
                  loop(next, remNums.tail)
                } else { ret(remStr, remNums, 0) }
            }
      }

    loop(line.firstPart, line.nums)
  }

  private def doit(filename: Path) =
    loadFile(filename).map(countValidSolutions).runSum.timed

  private def doit2(filename: Path) = loadFile(filename)
    .mapZIO { pl =>
      val nums = List.fill(5)(pl.nums).flatten
      ZStream
        .repeat(pl.firstPart)
        .take(5)
        .intersperse(List('?'))
        .runCollect
        .map(_.toList.flatten)
        .map(firstPart => ParsedLine(firstPart, nums))
    }
    .map(countValidSolutions)
    .runSum
    .timed

  private val base = Path("e:/dev/aoc2023") / "data" / "day12"

  private def runOne[R, E](correct: Long, z: ZIO[R, E, (Duration, Long)]) =
    z.flatMap { case (time, answer) =>
      Console.printLine(
        s"Answer in ${time.toMillis}ms (should be $correct): $answer"
      )
    }

  private val t1 = runOne(21, doit(base / "test.txt"))
  private val t2 = runOne(525152, doit2(base / "test.txt"))
  private val d = {
    val file = base / "data.txt"
    runOne(7857, doit(file)) *> runOne(28606137449920L, doit2(file))
  }

  override def run: ZIO[Environment with ZIOAppArgs with Scope, Any, Any] =
    (t1 *> t2 *> d).repeatN(5)
}
