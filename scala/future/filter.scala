import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(1)

val f2 = f1.filter(x => { throw new Exception("X") })
f2.onComplete { x => println(x) } // Failure(java.lang.Exception: X)
