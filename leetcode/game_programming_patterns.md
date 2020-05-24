# Introduction

In fifth grade, my friends and I were given access to a little unused classroom housing a couple
of very beat-up TRS-80s. Hoping to inspire us, a teacher found a printout of some simple
BASIC programs for us to tinker with.

The audio cassette drives on the computers were broken, so any time we wanted to run some
code, we’d have to carefully type it in from scratch. This led us to prefer programs that were
only a few lines long:

    10 PRINT "BOBBY IS RADICAL!!!"
    20 GOTO 10

Maybe if the computer prints it enough times, it will magically become true.

Even so, the process was fraught with peril. We didn’t know how to program, so a tiny syntax
error was impenetrable to us. If the program didn’t work, which was often, we started over
from the beginning.

At the back of the stack of pages was a real monster: a program that took up several dense
pages of code. It took a while before we worked up the courage to even try it, but it was
irresistible — the title above the listing was “Tunnels and Trolls”. We had no idea what it did,
but it sounded like a game, and what could be cooler than a computer game that you
programmed yourself?

We never did get it running, and after a year, we moved out of that classroom. (Much later
when I actually knew a bit of BASIC, I realized that it was just a character generator for the
table-top game and not a game in itself.) But the die was cast — from there on out, I wanted to
be a game programmer.

When I was in my teens, my family got a Macintosh with QuickBASIC and later THINK C. I
spent almost all of my summer vacations hacking together games. Learning on my own was
slow and painful. I’d get something up and running easily — maybe a map screen or a little
puzzle — but as the program grew, it got harder and harder.

Many of my summers were also spent catching snakes and turtles in the swamps of southern
Louisiana. If it wasn’t so blisteringly hot outside, there’s a good chance this would be a
herpetology book instead of a programming one.

At first, the challenge was just getting something working. Then, it became figuring out how to
write programs bigger than what would fit in my head. Instead of just reading about “How to
Program in C++”, I started trying to find books about how to organize programs.

Fast-forward several years, and a friend hands me a book: Design Patterns: Elements of
Reusable Object-Oriented Software. Finally! The book I’d been looking for since I was a 
teenager. I read it cover to cover in one sitting. I still struggled with my own programs, but it
was such a relief to see that other people struggled too and came up with solutions. I felt like I
finally had a couple of tools to use instead of just my bare hands.

This was the first time we’d met, and five minutes after being introduced, I sat down on his
couch and spent the next few hours completely ignoring him and reading. I’d like to think my
social skills have improved at least a little since then.

In 2001, I landed my dream job: software engineer at Electronic Arts. I couldn’t wait to get a
look at some real games and see how the pros put them together. What was the architecture like
for an enormous game like Madden Football? How did the different systems interact? How did
they get a single codebase to run on multiple platforms?

Cracking open the source code was a humbling and surprising experience. There was brilliant
code in graphics, AI, animation, and visual effects. We had people who knew how to squeeze
every last cycle out of a CPU and put it to good use. Stuff I didn’t even know was possible,
these people did before lunch.

But the architecture this brilliant code hung from was often an afterthought. They were so
focused on features that organization went overlooked. Coupling was rife between modules.
New features were often bolted onto the codebase wherever they could be made to fit. To my
disillusioned eyes, it looked like many programmers, if they ever cracked open Design
Patterns at all, never got past Singleton.

Of course, it wasn’t really that bad. I’d imagined game programmers sitting in some ivory
tower covered in whiteboards, calmly discussing architectural minutiae for weeks on end. The
reality was that the code I was looking at was written by people working to meet intense
deadlines. They did the best they could, and, as I gradually realized, their best was often very
good. The more time I spent working on game code, the more bits of brilliance I found hiding
under the surface.

Unfortunately, “hiding” was often a good description. There were gems buried in the code, but
many people walked right over them. I watched coworkers struggle to reinvent good solutions
when examples of exactly what they needed were nestled in the same codebase they were
standing on.

That problem is what this book aims to solve. I dug up and polished the best patterns I’ve
found in games, and presented them here so that we can spend our time inventing new things
instead of re-inventing them.

- Time and sequencing are often a core part of a game's architecture. Things must happen in the right order and at the right time.

- Development cycles are highly compressed, and a number of programmers need to be able to rapidly build and iterate on a rich set of different behavior without stepping on each other's toes of leaving footprints all over the codebase.

- After all of this behavior is defined, it starts interacting. Monsters bite the hero, potions are mixed together, and bombs blast enemies and friends alike. Those interactions must happen without the codebase turning into an intertwined hairball.

- And, finally, performance is critical in games. Game developers are in a constant race to see who can squeeze the most out of their platform. Tricks for shaving off cycles can mean the difference between an A-rated game and millions of sales or dropped frames and angry reviewers.

**Each of these patterns is described using a consistent structure so that you can use this book as a reference and quickly find what you need:**

- The Intent section provides a snapshot description of the pattern in terms of the problem
  it intends to solve. This is first so that you can hunt through the book quickly to find a
  pattern that will help you with your current struggle.

- The Motivation section describes an example problem that we will be applying the
  pattern to. Unlike concrete algorithms, a pattern is usually formless unless applied to
  some specific problem. Teaching a pattern without an example is like teaching baking
  without mentioning dough. This section provides the dough that the later sections will
  bake.

- The Pattern section distills the essence of the pattern out of the previous example. If you
  want a dry textbook description of the pattern, this is it. It’s also a good refresher if
  you’re familiar with a pattern already and want to make sure you don’t forget an ingredient.
  
- So far, the pattern has only been explained in terms of a single example. But how do you
  know if the pattern will be good for your problem? The When to Use It section provides
  some guidelines on when the pattern is useful and when it’s best avoided. The Keep in
  Mind section points out consequences and risks when using the pattern.
  
- If, like me, you need concrete examples to really get something, then Sample Code is
  your section. It walks step by step through a full implementation of the pattern so you can
  see exactly how it works.
  
- Patterns differ from single algorithms because they are open-ended. Each time you use a
  pattern, you’ll likely implement it differently. The next section, Design Decisions,
  explores that space and shows you different options to consider when applying a pattern.

- To wrap it up, there’s a short See Also section that shows how this pattern relates to
  others and points you to real-world open source code that uses it.

# What is Software Architecture?

If you read this book cover to cover, you won’t come away knowing the linear algebra behind
3D graphics or the calculus behind game physics. It won’t show you how to alpha-beta prune
your AI’s search tree or simulate a room’s reverberation in your audio playback.

Wow, this paragraph would make a terrible ad for the book.

Instead, this book is about the code between all of that. It’s less about writing code than it is
about organizing it. Every program has some organization, even if it’s just “jam the whole
thing into main() and see what happens”, so I think it’s more interesting to talk about what
makes for good organization. How do we tell a good architecture from a bad one?

I’ve been mulling over this question for about five years. Of course, like you, I have an
intuition about good design. We’ve all suffered through codebases so bad, the best you could
hope to do for them is take them out back and put them out of their misery.

Let’s admit it, most of us are responsible for a few of those.

A lucky few have had the opposite experience, a chance to work with beautifully designed
code. The kind of codebase that feels like a perfectly appointed luxury hotel festooned with
concierges waiting eagerly on your every whim. What’s the difference between the two?

# What is good software architecture?

> For me, good design means that when I make a change, it's as if the entire program was crafted in anticipation of it. I can solve a task with just a few choice function calls that slot in perfectly, leaving not the slightest ripple on the placid surface of the code.

That sounds pretty, but it's not exactly actionable. "Just write your code so that changes don't disturb its placid surface." Right.

Let me break that down a bit. The first key piece is that architecture is about change.
Someone has to be modifying the codebase. If no one is touching the code — whether because
it’s perfect and complete or so wretched no one will sully their text editor with it — its design
is irrelevant. The measure of a design is how easily it accommodates changes. With no
changes, it’s a runner who never leaves the starting line.

# How do you make a change?

Before you can change the code to add a new feature, to fix a bug, or for whatever reason
caused you to fire up your editor, you have to understand what the existing code is doing. You
don’t have to know the whole program, of course, but you need to load all of the relevant
pieces of it into your primate brain.

It’s weird to think that this is literally an OCR process.

We tend to gloss over this step, but it’s often the most time-consuming part of programming. If
you think paging some data from disk into RAM is slow, try paging it into a simian cerebrum over a pair of optical nerves.

Once you’ve got all the right context into your wetware, you think for a bit and figure out your
solution. There can be a lot of back and forth here, but often this is relativel straightforward.

Once you understand the problem and the parts of the code it touches, the actual coding is
sometimes trivial.

You beat your meaty fingers on the keyboard for a while until the right colored lights blink on
screen and you’re done, right? Not just yet! Before you write tests and send it off for code
review, you often have some cleanup to do.

Did I say “tests”? Oh, yes, I did. It’s hard to write unit tests for some game code, but a large
fraction of the codebase is perfectly testable.

I won’t get on a soapbox here, but I’ll ask you to consider doing more automated testing if you
aren’t already. Don’t you have better things to do than manually validate stuff over and over
again?

You jammed a bit more code into your game, but you don’t want the next person to come along
to trip over the wrinkles you left throughout the source. Unless the change is minor, there’s
usually a bit of reorganization to do to make your new code integrate seamlessly with the rest
of the program. If you do it right, the next person to come along won’t be able to tell when any
line of code was written

# At What Cost?
>This sounds great, right? Decouple everything and you’ll be able to code like the wind. Each
 change will mean touching only one or two select methods, and you can dance across the
 surface of the codebase leaving nary a shadow.

This feeling is exactly why people get excited about abstraction, modularity, design patterns,
and software architecture. A well-architected program really is a joyful experience to work in,
and everyone loves being more productive. Good architecture makes a huge difference in
productivity. It’s hard to overstate how profound an effect it can have.

But, like all things in life, it doesn’t come free. Good architecture takes real effort and
discipline. Every time you make a change or implement a feature, you have to work hard to
integrate it gracefully into the rest of the program. You have to take great care to both 
organize the code well and keep it organized throughout the thousands of little changes that  
make up a development cycle.

The second half of this — maintaining your design — deserves special attention. I’ve seen many
programs start out beautifully and then die a death of a thousand cuts as programmers add “just
one tiny little hack” over and over again.

Like gardening, it’s not enough to put in new plants. You must also weed and prune.
You have to think about which parts of the program should be decoupled and introduce
abstractions at those points. Likewise, you have to determine where extensibility should be
engineered in so future changes are easier to make.

People get really excited about this. They envision future developers (or just their future self)
stepping into the codebase and finding it open-ended, powerful, and just beckoning to be extended. They imagine The One Game Engine To Rule Them All.

But this is where it starts to get tricky. Whenever you add a layer of abstraction or a place
where extensibility is supported, you’re speculating that you will need that flexibility later.
You’re adding code and complexity to your game that takes time to develop, debug, and
maintain.

That effort pays off if you guess right and end up touching that code later. But predicting the
future is hard, and when that modularity doesn’t end up being helpful, it quickly becomes
actively harmful. After all, it is more code you have to deal with.

# Performance and Speed

There’s another critique of software architecture and abstraction that you hear sometimes, especially in game development: that it hurts your game’s performance. Many patterns that make your code more flexible rely on virtual dispatch, interfaces, pointers, messages, and other mechanisms that all have at least some runtime cost.

There’s a spectrum of flexibility here. When you write code to call a concrete method in some class, you’re fixing that class at author time — you’ve hard-coded which class you call into. When you go through a virtual method or interface, the class that gets called isn’t known until runtime. That’s much more flexible but implies some runtime overhead.

Template metaprogramming is somewhere between the two. There, you make the decision of which class to call at compile time when the template is instantiated.

But performance is all about assumptions. The practice of optimization thrives on concrete  limitations. Can we safely assume we’ll never have more than 256 enemies? Great, we can pack  an ID into a single byte. Will we only call a method on one concrete type here? Good, we can  statically dispatch or inline it. Are all of the entities going to be the same class? Great,  we can make a nice contiguous array of them.

This doesn’t mean flexibility is bad, though! It lets us change our game quickly, and development speed is absolutely vital for getting to a fun experience. No one, not even Will Wright, can come up with a balanced game design on paper. It demands iteration and experimentation.

The faster you can try out ideas and see how they feel, the more you can try and the more likely you are to find something great. Even after you’ve found the right mechanics, you need plenty of time for tuning. A tiny imbalance can wreck the fun of a game.

There’s no easy answer here. Making your program more flexible so you can prototype faster will have some performance cost. Likewise, optimizing your code will make it less flexible. My experience, though, is that it’s easier to make a fun game fast than it is to make a fast game fun. One compromise is to keep the code flexible until the design settles down and then tear out some of the abstraction later to improve your performance.

# Design Patterns Revisited

- Command
> Command is one of my favorite patterns. Most large programs I write, games or otherwise, end
up using it somewhere. When I’ve used it in the right place, it’s neatly untangled some really
gnarly code. For such a swell pattern, the Gang of Four has a predictably abstruse description:

    Encapsulate a request as an object, thereby letting users parameterize clients with
    different requests, queue or log requests, and support undoable operations.
