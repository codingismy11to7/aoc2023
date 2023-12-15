import zio._
import zio.nio.file.{Files, Path}
import zio.stream.ZStream

import scala.annotation.tailrec

object Main extends ZIOAppDefault {
  private type Stream[A] = ZStream[Any, Nothing, A]

  private final case class BlockInfo(numRows: Int, numCols: Int)
  private final case class Block(
      data: Chunk[Chunk[Char]],
      info: BlockInfo
  )
  private final case class NumBlock(
      cols: Chunk[Long],
      rows: Chunk[Long]
  )
  private sealed trait MirrorPoint {
    def valueIf(isColumn: Boolean): Option[Int]
  }
  private object MirrorPoint {
    final case class Column(num: Int) extends MirrorPoint {
      override def valueIf(isColumn: Boolean): Option[Int] =
        if (isColumn) Some(num) else None
    }
    final case class Row(num: Int) extends MirrorPoint {
      override def valueIf(isColumn: Boolean): Option[Int] =
        if (isColumn) None else Some(num)
    }
  }

  private def convertToNumber(chars: Chunk[Char]) =
    chars
      .foldLeft((0L, 1)) { case ((acc, mult), c) =>
        val thisBit = if (c == '.') 0 else 1
        (acc + thisBit * mult) -> (2 * mult)
      }
      ._1

  private def blockColumn(block: Block)(whichCol: Int) = {
    block.data.map(line => line(whichCol))
  }

  private def blockToNumblock(block: Block) = {
    val rows = block.data.map(convertToNumber)
    val numCols = block.data.head.size
    val cols =
      Chunk.from(0 until numCols).map(blockColumn(block)).map(convertToNumber)

    NumBlock(cols, rows)
  }

  private def isMirrorpoint(nums: Chunk[Long])(mirrorAfter: Int): Boolean = {
    @tailrec
    def loop(delta: Int): Boolean = {
      val left =
        if ((delta - 1) > mirrorAfter) None
        else nums.lift(mirrorAfter - (delta - 1))
      val right = nums.lift(mirrorAfter + delta)

      left -> right match {
        case (Some(l), Some(r)) if l != r => false
        case (Some(_), Some(_))           => loop(1 + delta)
        case _                            => true
      }
    }

    loop(1)
  }

  private def blockToSmudgeCorrected(block: Block): Stream[Block] = {
    ZStream.fromIterable(0 until block.info.numRows).flatMap { row =>
      ZStream.fromIterable(0 until block.info.numCols).map { col =>
        val c = block.data(row)(col)
        block.copy(data =
          block.data.updated(
            row,
            block.data(row).updated(col, if (c == '.') '#' else '.')
          )
        )
      }
    }
  }

  private def findMirrorpoint(nums: Chunk[Long], butNot: Option[Int]) =
    (0 until (nums.size - 1))
      .filterNot(butNot.contains)
      .find(isMirrorpoint(nums))

  private def numblockToMirrorpointOpt(
      butNot: Option[MirrorPoint]
  )(block: NumBlock): Option[MirrorPoint] = {
    val row =
      findMirrorpoint(block.rows, butNot.flatMap(_.valueIf(false))).map(i =>
        MirrorPoint.Row(i)
      )
    val col =
      findMirrorpoint(block.cols, butNot.flatMap(_.valueIf(true))).map(i =>
        MirrorPoint.Column(i)
      )

    row orElse col
  }

  private def numblockToMirrorpoint(butNot: Option[MirrorPoint])(
      block: NumBlock
  ) =
    numblockToMirrorpointOpt(butNot)(block).get

  private def mirrorPointToNum(mp: MirrorPoint): Long = mp match {
    case MirrorPoint.Column(c) => c + 1
    case MirrorPoint.Row(r)    => 100 * (r + 1)
  }

  private def toBlock(strs: Chunk[String]) = Block(
    strs.map(Chunk.fromIterable(_)),
    BlockInfo(strs.size, strs.head.length)
  )

  private def loadFile(filename: Path) =
    Files
      .lines(filename)
      .map(_.trim)
      .split(_.isEmpty)
      .map(toBlock)

  private def doit(filename: Path) =
    loadFile(filename)
      .map(blockToNumblock)
      .map(numblockToMirrorpoint(None))
      .map(mirrorPointToNum)
      .runSum

  private def doit2(filename: Path) = loadFile(filename)
    .mapZIO { block =>
      val oldMirrorPoint =
        numblockToMirrorpointOpt(None)(blockToNumblock(block))

      blockToSmudgeCorrected(block)
        .map(blockToNumblock)
        .map(numblockToMirrorpointOpt(oldMirrorPoint))
        .collect { case Some(x) => x }
        .runHead
        .map(_.get)
    }
    .map(mirrorPointToNum)
    .runSum

  private val base = Path("e:/dev/aoc2023") / "data" / "day13"

  private def runOne[R, E](correct: Long, z: ZIO[R, E, Long]) =
    z.timed.flatMap { case (time, answer) =>
      Console.printLine(
        s"Answer in ${time.toMillis}ms (should be $correct): $answer"
      )
    }

  private val t1 = runOne(405, doit(base / "test.txt"))
  private val t2 = runOne(400, doit2(base / "test.txt"))
  private val d = {
    val file = base / "data.txt"
    runOne(29213, doit(file)) *> runOne(37453, doit2(file))
  }

  override def run: ZIO[Environment with ZIOAppArgs with Scope, Any, Any] =
    (t1 *> t2 *> d *> Console.printLine("")).repeatN(5)
}
