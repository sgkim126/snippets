import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(1)
val f2 = Future.firstCompletedOf(Seq(f1))
f2.onComplete { x => println(x) } // Success(1)


val f3 = Future.failed(new Exception("a"))
val f4 = Future.firstCompletedOf(Seq(f3))
f4.onComplete { x => println(x) } // Failure(java.lang.Exception: a)

val f5 = Future.firstCompletedOf(Seq())
f5.onComplete { x => println(x) } // Not completed
