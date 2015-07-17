import scala.concurrent.Future
import scala.concurrent.ExecutionContext.Implicits.global

val f1 = Future.successful(1)
val f2 = f1 collect {
  case x if x < 0 => - x
}

f2 onComplete {
  case x => println(x) // Failure(java.util.NoSuchElementException: Future.collect partial function is not defined at: 1)
}
