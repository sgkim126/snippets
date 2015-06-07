import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(1)
val f2 = Future.successful(2)
val f3 = Future.successful(3)
val f4 = Future.successful(4)

val f5 = Future.sequence(Seq(f1, f2, f3, f4))

f5.map { case a => println(s"$a") } // List(1, 2, 3, 4)

val f6 = Future.successful(1)
val f7 = Future.successful(List(2))
val f8 = Future.successful(Seq(3))
val f9 = Future.successful(Seq(4, 5, 6))

val f10 = Future.sequence(Seq(f6, f7, f8, f9))

f10.map { case a => println(s"$a") } // List(1, List(2), List(3), List(4, 5, 6))
