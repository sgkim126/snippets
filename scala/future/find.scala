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

val f7 = Future {
  Thread.sleep(10000)
  1
}
val f8 = Future.successful(2)

val f9a = Future.find(Seq(f7, f8))(x => { true })
f9a.onComplete { x => println(x) } // Success(Some(2))

Thread.sleep(20000)

val f9b = Future.find(Seq(f7, f8))(x => { true })
f9b.onComplete { x => println(x) } // Success(Some(1))
