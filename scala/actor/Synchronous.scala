import akka.actor.{ ActorRef, ActorSystem, Props, Inbox }
import scala.concurrent.duration._

case class WhoToGreet(who: String)

object Synchronous extends App {
  val system = ActorSystem("actorexample")

  val greeter = system.actorOf(Props[Greeter], "greeter")

  val inbox = Inbox.create(system)

  greeter.tell(WhoToGreet("akka"), ActorRef.noSender)

  inbox.send(greeter, Greet)

  val Greeting(message1) = inbox.receive(5.seconds)
  println(s"Greeting: $message1")
}
