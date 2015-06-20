import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(1)
val f2 = Future.failed(new Exception("a"))

val f3 = Future.find(Seq(f1, f2))(x => { x == 1 })
f3.onComplete { x => println(x) } // Success(Some(1))

val f4 = Future.find(Seq(f2))(x => { x == 1 })
f4.onComplete { x => println(x) } // Success(None)

val f5 = Future.find(Seq(f1, f2))(x => { x == 2 })
f5.onComplete { x => println(x) } // Success(None)

val f6 = Future.find(Seq(f1, f2))(x => { throw new Exception("b") })
f6.onComplete { x => println(x) } // Success(None)
