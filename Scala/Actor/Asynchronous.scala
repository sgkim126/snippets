import akka.actor.{ ActorRef, ActorSystem, Props }
import akka.pattern.ask
import scala.concurrent.duration._

case class WhoToGreet(who: String)

object ASynchronous extends App {
  val system = ActorSystem("actorexample")

  val greeter = system.actorOf(Props[Greeter], "greeter")

  greeter.tell(WhoToGreet("akka"), ActorRef.noSender)

  val future = ask(greeter, Greet("typesafe"))(5.seconds)
  future.onSuccess {
    case Greeting(message: String) =>   println(s"Greeting: $message1")
  }
}

