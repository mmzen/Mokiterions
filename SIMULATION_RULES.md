# How the Mokiterions Simulation Works

A plain-language guide. No programming knowledge needed. If you read this once, you will understand
what happens when you run the simulation and why.

> **Where the real rules live.** This file explains the rules. It does not *set* them. The binding
> contract is `docs/engineering/simulation/specifications/SPEC-MOK-001.md`. If this guide and that
> document ever disagree, the specification is right and this file has a bug. Every number here was
> taken from it or measured by running the program.

---

## 1. The one-minute version

Twelve creatures called **Mokiterions** live on a square map. They get hungry. There is food on the
map, but not much. Each creature can look around, walk one step, or eat what it is standing on — and,
if somebody is standing right next to it, hit them, threaten them, back away from them, or hand over
half its dinner. Nobody tells them what to do as a group — each one decides for itself, one at a
time, over and over.

The interesting question is: **who is still alive after 1,000 turns?**

The answer is **8 to 10 of the 12** under either of the two deciders that only care about food: 8, 9,
10, 9 and 9 across the five seeds the project measures under the default one, and 10, 8, 10, 8 and 8
under the trait-aware decider of section 11. Some always die. That is on purpose. A world where
everyone survives easily would be boring, and a world where everyone dies would tell us nothing.

The twelve are also no longer interchangeable. Each is born with one fixed quirk — how much food it
is willing to waste — which changes what it will eat and what it will walk to (section 3, section
11). Each also carries a mood, `fear`, that rises when it is not alone, and one of the four deciders
now reads it: below a threshold it engages, above it it backs off, high enough and it surrenders.

**With that decider, 7 to 9 survive, and half the deaths are killings.** Nine of the eighteen deaths
across its five runs are caused by combat — but the violence is an opening rather than a way of life:
fifty-six of the sixty-three strikes land inside the first fourteen turns, and for the remaining nine
hundred and eighty-six the twelve mostly keep out of each other's way. Section 11 spells it out, and
says which of its own older figures were wrong.

**The numbers in this document were re-measured on 2026-08-21**, when a missing term in the eating rule
was corrected (section 12). A few could not be — they came from runs far longer than the ones the
project declares — and each of those says so where it appears.

---

## 2. The world

The map is a grid, 128 squares wide and 128 squares tall. Every position is a pair of numbers,
written `x:y`, each from 0 to 127. Drawn on a page, `0:0` sits top-left and `127:127` bottom-right —
that is just a drawing convention, but this guide sticks to it.

The map is split into two halves called **territories**:

```
        x = 0 ....................................... x = 127
 y = 0    ┌───────────────────────────────────────────┐
          │                                           │
          │              Territory A                  │   y from 0 to 63
          │                                           │
 y = 63   ├───────────────────────────────────────────┤
 y = 64   │                                           │
          │              Territory B                  │   y from 64 to 127
          │                                           │
 y = 127  └───────────────────────────────────────────┘
```

Each territory is 128 × 64 = **8,192 squares**.

The line between them is **not a wall**. Anyone can walk across it. The territory is just a label
that says which half you are currently standing in, and the simulation makes a note when someone
crosses over. Food, however, belongs to a territory, and food only regrows within its own half.

Nothing blocks movement. There are no walls, no obstacles, no water. Two creatures can stand on the
same square, and a creature can stand on top of a piece of food.

---

## 3. The Mokiterions

There are exactly **twelve**, and each one has a name as well as an identifier. Six start in
territory A and six start in territory B, at random positions, never two on the same square at the
start.

| Territory | Who lives there |
|---|---|
| **A** | `M01` Zug, `M02` Krul, `M03` Quib, `M04` Sput, `M05` Trok, `M06` Womp |
| **B** | `M07` Hozz, `M08` Nurb, `M09` Vonk, `M10` Gorm, `M11` Xob, `M12` Drix |

The name is fixed for good. Trok is `M05` on seed 42, on seed 43, and on every run of every seed;
nothing that happens in a run changes it, and no name is ever reused or handed on when somebody dies.
The identifier stays as well, because it is what every line of output is filed under and what the
observer's panes are joined on — so a name is something a Mokiterion *has in addition to* its number,
not instead of it. The twelve initials are twelve different letters, which is what lets the map draw a
Mokiterion as a single letter and still tell you which one it is.

Each one carries four numbers, each from 0 to 100:

| Number | What it means | Starts at |
|---|---|---:|
| **health** | how alive you are. Reaches 0 and you are dead, permanently | 100 |
| **satiety** | how *full* you are. This is hunger, counted upwards | 100 |
| **energy** | how rested you are | 100 |
| **fear** | how uneasy you are. Rises when others are in sight, falls when you are alone | 0 |

**Satiety is the word to remember.** It is not hunger — it is the opposite. 100 means completely
full, 0 means completely empty. It only goes up by eating.

Those four change during the run. There is also one number that never changes:

| Trait | What it means | Range |
|---|---|---|
| **waste tolerance** | how much of a piece of food you are willing to throw away in order to eat it now | 0 to 40 |

**This is the only thing that makes one Mokiterion behave differently from another.** It is decided
once, at the moment the world is created, from the seed and the creature's own number — so `M07` on
seed 42 has the same tolerance every single time you run seed 42, and a different one on seed 43. It
is announced once, on the creature's first line of output, and never mentioned again. Nothing in the
run can change it: not eating, not starving, not dying.

The values are genuinely spread out. On seed 42, the twelve are:

```
M01 11   M02 40   M03  4   M04 24   M05 21   M06 13
M07  7   M08 40   M09 24   M10 15   M11 10   M12 23
```

Nine to eleven different values per seed, in practice, with both `0` and `40` turning up across the
five seeds the project measures. What the number actually *does* is section 12 — and only two of the
four deciders read it at all: `individual`, which is the trait-aware one, and `social`, which delegates
its food decisions to it. Under `baseline` and `reference` — the default — the number is derived,
reported, and ignored. Since 2026-08-21 it is also read a great deal less than the rest of this document
used to imply: section 12 sets out where the trait now changes nothing at all.

Still absent: no memory, no preferences, no relationships. A name is not a personality — Zug and Krul
run on the same rules, and two Mokiterions with the same tolerance are still interchangeable apart
from where they are standing and what they are called.

---

## 4. Food

Food sits on the map and does not move. Each piece has an ID like `F0058` and one of three sizes:

| Class | Satiety it restores | Energy it restores |
|---|---:|---:|
| **Low** — a snack | 15 | 5 |
| **Medium** — a meal | 30 | 10 |
| **High** — a feast | 50 | 20 |

At the start, the three classes are handed out in strict rotation — low, medium, high, low, medium,
high — so the world begins with an even three-way split.

---

## 5. Time: what happens in one turn

Time moves in whole steps called **ticks**. Think of a tick as one turn. On every tick, this happens
in this exact order:

1. **The clock advances** by one.
2. **Each living Mokiterion gets one action opportunity**, in order: `M01` first, then `M02`, and so
   on to `M12`. Dead ones are skipped entirely.
3. For each one, in turn:
   - **It looks around.** The simulation hands it a description of what it can see (see section 7),
     together with who hit it since its last turn.
   - **It picks one action** — just one — from the eleven in section 6.
   - **The simulation checks the action is legal**, and applies it if so.
   - **Its memory of who hit it is wiped**, whether it acted on it or not.
   - **Then it gets hungrier and more tired, and its fear moves** (see section 8).
4. **After everyone has moved, food may regrow** — but only on ticks that divide by 10 (tick 10, 20,
   30 …). See section 9.
5. **The simulation checks whether to stop**: either everyone is dead, or the turn limit was reached.

Three details that matter more than they look:

- Order is fixed and always the same. `M01` always acts before `M02`. If `M01` eats the last piece of
  food on a square, `M02` arrives to find nothing there. Being early is an advantage.
- One action per turn. A creature cannot walk *and* eat in the same turn. Walking onto food means
  eating it next turn at the earliest.
- **Everything a creature does to somebody else lands immediately, inside its own turn.** There is no
  simultaneous resolution and nothing is queued up for later. So if `M03` strikes `M07` hard enough to
  kill it, `M07` never takes its turn that tick — it was alive at the start of the tick and dead
  before its own opportunity arrived. Being early is an advantage here too, and a bigger one.

---

## 6. The eleven things a Mokiterion can do

That is the complete list. There is nothing else.

Four of them concern nobody but yourself:

| Action | What it does | What it costs |
|---|---|---|
| **move** | step one square north, east, south, or west | nothing extra |
| **eat** | consume a piece of food **on your own square**, removing it from the world | nothing extra |
| **sleep** | recover 20 energy | nothing extra |
| **wait** | do nothing at all | nothing extra |

The other seven each name **one other creature**:

| Action | What it does | What it costs |
|---|---|---|
| **attack** | strike the named creature for 10 to 30 damage | 5 energy |
| **fight** | the same strike, aimed at somebody who struck you | 5 energy |
| **threaten** | raise the named creature's fear by 30 | nothing |
| **surrender** | hand half your own satiety to somebody who struck you | half your satiety |
| **approach** | step one square toward the named creature | nothing extra |
| **avoid** | step one square away from the named creature | nothing extra |
| **retreat** | the same step away, from somebody who struck you | nothing extra |

Each of the seven has its own condition, and this is where most of the interesting behaviour comes
from:

- **attack, fight and threaten need *contact*** — the target within one square of you, diagonals
  included. That is a circle of 8 squares, against the 1,088 squares you can *see* (section 7). The
  gap between those two numbers matters more than anything else in this section; section 11 has the
  measurement.
- **approach and avoid need only that you can see the target**, out to the full 16 squares.
- **fight, surrender and retreat need the target to have struck you**, and specifically to have
  struck you since your last turn — see the memory rule below. `surrender` and `retreat` do *not*
  need contact: you can pay off or back away from somebody who hit you and then stepped away.
- Nobody can target themselves, and nobody can target the dead — including for a `fight` answering
  an attack the attacker made while it was still alive.

Rules and limits:

- **Movement is one square at a time, and never diagonal.** To reach a square 3 east and 2 north, you
  need 5 turns. `approach`, `avoid` and `retreat` are ordinary moves under the hood and obey this
  too: they pick a direction and take one step, and they are refused for the same reasons any move is
  refused.
- **You can only eat what is under your feet.** Not next to you — under you.
- Eating raises satiety and energy by the amounts in the food table, and the food is gone for good.
- **How hard you hit depends only on you:** `10 + (your energy + your health) / 10`, which is 10 at
  your weakest and 30 at full strength. The creature you hit makes no difference to it. There is no
  armour, no dodging, no luck, and no roll of any kind — combat consumes no randomness whatsoever.
- **Damage kills by the ordinary rule.** Health reaching 0 is death whether it got there through a
  strike or through starvation (section 8); there is no separate killing rule and no second way to
  die.
- **Being struck is remembered exactly one turn.** Whoever hit you is on your list until your next
  turn comes round, and taking that turn clears the list — all of it, however many people are on it.
  So you may answer *one* of the people who hit you, and answering one forgets the rest. That is the
  only memory anything in this simulation has (section 16).
- Nothing can push any number above 100. Extra is simply lost. A surrender of 50 satiety to somebody
  already at 80 gives them 20 and destroys the other 30 — the surrendering creature pays in full
  either way.
- Trying something illegal — walking off the edge of the map, eating food that is not there,
  attacking somebody two squares away — is not a crash. The turn is spent, nothing happens, and the
  simulation records that the attempt was rejected and which condition it failed.

---

## 7. What a Mokiterion can see

Each creature sees a square region reaching **16 squares in every direction**, including diagonally —
a 33 × 33 block centred on itself. Anything further away does not exist as far as it is concerned.

Within that region it is told:

- every piece of food: its ID, its class, roughly which way it lies (`north`, `north_east`, `east`, …)
  and how far away it is;
- every other *living* Mokiterion, with direction and distance.

What it is **not** told: anything outside the region, anything about dead creatures, and anything at
all about itself in its own list of neighbours.

It is handed one thing that is not about the region at all: **who struck it since its last turn, and
for how much.** That list is the only non-spatial thing in the description, and the only thing in it
that refers to the past. It can name somebody who has since walked out of sight, or died.

Two things do not obstruct sight: the territory line, and other creatures. Nobody hides behind
anybody.

Looking around is free. It costs no turn, changes nothing, and gives no advantage to whoever looks
first.

**Important:** seeing food is not the same as reaching it. A creature can watch a feast sitting 16
squares away and starve on the way there. This is the single biggest reason Mokiterions die — not a
shortage of food in the world, but the walking distance to it.

---

## 8. Getting hungry, dying, and being afraid

At the end of every single turn, for every living creature:

- **satiety drops by 1**
- **energy drops by 1**
- **and then**, if either of those has hit 0, **health drops by 5**
- **and then fear moves**, up or down — see below

Neither number can go below 0. Health reaching 0 means death, and death is final: no more turns, no
more looking around, no more anything.

So a creature that starts full and never eats:

- runs out of satiety after **100 turns**,
- then loses 5 health per turn, 20 times,
- and **dies on tick 119**.

That is not an estimate. Run `cargo run --bin Mokiterions -- --seed 42 --ticks 200 --density 0.02` — a world with a
single piece of food in each half — and **eleven of the twelve die together on tick 119**. The
twelfth happened to be standing near the one piece of food, ate it, and lasted until tick 134.

Notice that **sleep saves you from tiredness but nothing saves you from hunger except food**. Energy
has a free refill; satiety does not. That is why hunger is what actually kills everyone, and why the
whole simulation is really about food.

### Fear

Fear is not part of dying. It is bolted onto the same end-of-turn step, and the rule is two lines
long:

| What the creature saw this turn | What happens to fear |
|---|---|
| **at least one** other living Mokiterion in its look-around | **+10** |
| nobody at all | **−5** |

It stops at 0 at the bottom and at 100 at the top; overshoot is simply lost, exactly like satiety.

Four things about that rule are worth spelling out, because they are all deliberate:

- **It reuses the look-around the creature already did** to make its decision that turn (section 7).
  There is no second look, no extra cost, no randomness, and no separate "fear distance". If it is in
  sight, it counts.
- **Quantity, distance and direction are all ignored.** One stranger 16 squares away and eleven
  strangers standing on your square produce the same +10. The rule cannot tell the two apart.
- **Being seen is not the same as seeing.** Because the twelve act in order, `M01` looks around while
  `M02` is still in its old spot. So two neighbours may not both notice each other on the same turn.
- **The step is +10 up and −5 down**, so fear climbs twice as fast as it fades. In a world where you
  usually have company, that adds up quickly.

It adds up rather more quickly than the numbers suggest. Measured over 105,445 creature-turns — every
declared seed under the reference and trait-aware deciders, at the default density — **fear is sitting at
exactly 100 on 38% of them**, and above 0 on 44% to 64% of them depending on the run. There is also one
step size the table above does not predict — **+5**, which occurred 260 times. That is a creature at 95
gaining 10 and stopping at 100. Across all four deciders the population is 160,921 creature-turns and
fear sits at 100 on 34% of them, the baseline runs pulling the share down.

Those figures are from the current build. The same measurement over the same two deciders before the
waste allowance was corrected on 2026-08-21 gave 111,604 creature-turns, 39% and 219 — fewer turns now
because fewer creatures reach turn 1,000, for the reason section 12 records.

**Something reads fear now, and there is a third way for it to move.** The social decider of section
11 consults it at three points, and a `threaten` aimed at you adds **+30** on the spot, outside the
end-of-turn step entirely. So fear can now move twice in one turn: once because somebody threatened
you, and again at the end of the turn by the table above.

That also means the two numbers `+10` and `−5` finally have an outcome to be right or wrong about,
and the first measurement said they are wrong — not obviously, but consequentially. Fear rises for
every turn you can *see* company, at 16 squares, while the actions that fear gates need company
within *one* square. Ten per turn crosses the engagement gate on the tenth turn of noticing somebody,
and walking 16 squares takes fifteen, so a creature can still be too afraid to engage by the time it
could. That gate was 30 — three turns — until it was raised to 95 on 2026-08-20, which is the project's
answer to the mismatch so far and not a repair of it: `+10` up against `−5` down still saturates. Section
11 has the numbers. Until the constants themselves are settled, treat the 38% as a fact about the current
ones and not as a finding.

---

## 9. How food comes back

Every 10 turns, each territory gets one chance to regrow food. The rules are strict:

| Situation in that territory | What happens |
|---|---|
| It has some food, and room for more | **2 new pieces appear** at random empty squares |
| It has some food, and only room for 1 | 1 appears, filling it to the top |
| It is completely full | nothing appears |
| **It has zero food left** | **nothing appears, now or ever again** |

That last row is the trap, and it is deliberate. **Food regrows from food.** Strip a territory bare
and you have permanently destroyed it — no amount of waiting brings it back. Half the world becomes a
desert, for the rest of the run.

New pieces get a random class (low, medium, or high, each equally likely), so over a long run the
tidy three-way split from the start drifts.

---

## 10. How much food: the density setting

You choose how much food the world holds, with one number: **density**, the percentage of a
territory's squares that hold food.

Since a territory has 8,192 squares:

| You ask for | Food per territory | Notes |
|---|---:|---|
| `0.15%` | 12 | very sparse |
| **`0.75%`** | **61** | **the default** |
| `1.00%` | 81 | |
| `1.50%` | 122 | comfortable |
| `3.00%` | 245 | easy |
| `0.01%` | 0 | **rejected** — see below |

This one number does **three** jobs at once:

1. how much food the world **starts** with,
2. the **maximum** a territory can ever hold,
3. the level that regrowth **aims for**.

Tying all three together is intentional. An earlier version set only the maximum, so territories
began nearly empty and slowly filled up over hundreds of turns — and everyone starved during the
climb. Now a territory starts at its intended level and regrowth restores it back to that same level.

A density that works out to zero food is rejected with an error, because a territory with no food can
never regrow any (section 9), so the run's ending would be decided before it began.

**A warning that trips people up:** more food does not reliably mean more survivors. Changing the
density changes where *everything* gets placed, so two densities are two different worlds, not the
same world with more food in it. Measured example, on seed `0` under the reference decider: `0.70%`
leaves nine alive while the default `0.75%` leaves eight, and `1.00%` leaves seven while `1.25%` leaves
twelve. Only the default `0.75%` carries a promise about survivors; every other density is unexplored
territory.

---

## 11. Who decides what a Mokiterion does

Here is the part that matters most for understanding the project.

**The creatures do not control the world.** When it is a creature's turn, it is handed a read-only
description of its surroundings and hands back a *request*: "I would like to move east." It cannot
change anything itself. The simulation checks the request and decides what actually happens.

This separation is the whole point of the design. It means the thing making decisions can be swapped
out — for something smarter, later — without it ever being able to cheat. The newest decider asks to
hit other creatures, and that changes nothing about the arrangement: it still only asks, and the
simulation still decides. A creature cannot damage anybody by wanting to. It names a target and a
verb, and every condition in section 6 is checked against the simulation's own state before anything
happens.

Four deciders exist today. Pick one with `--policy`.

### The baseline decider — `--policy baseline`

Lists every legal action and picks one **at random**. That is all. It is a control case: it shows what
the world does to someone with no judgement at all.

It starves. On seed 42 at the default density, all twelve are dead by tick **142** — with 122 pieces
of food still lying on the ground, both territories completely full. Food was never the problem.
Walking to it was.

### The reference decider — `--policy reference` (the default)

A simple food-seeker. Every turn it runs down this list and does the **first** thing that applies:

1. **Eat what I am standing on** — but only if the food fits (see section 12). Prefer the biggest
   piece that fits.
2. **Sleep**, if energy has dropped below 20.
3. **Walk one step toward the nearest food I can see that would fit.** Ties break toward the bigger
   piece. It moves sideways first if the food is at all to the east or west, otherwise up or down.
4. **Take one random step** — because nothing worth walking to is in sight.

It never chooses "wait". A creature with nothing to do goes looking instead of standing still.

This decider is **not intelligence and not an AI**. It is a deliberately simple, predictable yardstick
that proves the world is survivable and gives future, smarter deciders something to be compared
against. Without it, "the clever decider kept 9 alive" would be a meaningless claim — you would not
know whether 9 was good.

### The trait-aware decider — `--policy individual`

**This is the same four-step list as the reference decider, with exactly one difference:** how much
food it is prepared to throw away. It reads the creature's waste tolerance from section 3 and nothing
else. Steps 1 and 3 — eat what I am standing on, walk toward the nearest food worth reaching — use the
looser test described in section 12. Steps 2 and 4 are untouched. It never chooses "wait" either.

Because the tolerance is a different number for each creature, **two Mokiterions in exactly the same
spot, at exactly the same fullness, looking at exactly the same piece of food, can make different
choices.** That is the entire point of it, and it is the first time anything in this simulation has
been true of one creature and not another.

Two ends of the range are worth knowing:

- **A creature with tolerance 0 behaves like the reference decider — not roughly, exactly.** Every one
  of 4,536 possible situations was checked one by one, and it proposed the identical action in all of
  them, having consumed the identical amount of randomness. So the new decider contains the old one as
  its own most careful member. That was 2,808 situations until 2026-08-21: the set is built around the
  satieties where a piece stops fitting, those moved, and the old ones were kept alongside the new.
- **A creature with tolerance 40**, the highest possible, will eat a *snack* at satiety 91 where the
  reference decider stops at 87 — and on a feast it does exactly what the reference decider does,
  because the piece's own allowance is already wider than anything tolerance 40 can add. That is a
  change of 2026-08-21 and it used to be the other way round; see the table in section 12.

Measured over 1,000 turns at the default density, on the five seeds the project uses:

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---:|---:|---:|---:|---:|
| survivors, trait-aware | **10** | 8 | 10 | 8 | 8 |
| survivors, reference | 8 | 9 | 10 | 9 | 9 |

It is not simply better, and since 2026-08-21 it is mostly not better at all: it wins on seed 0, ties on
seed 42 and loses on the other three. Both deciders clear their floor of eight on every seed, but neither
clears it by much: the reference decider is exactly on it on seed 0, and the trait-aware one on three of
the five seeds. The seed-777 run that used to keep all twelve alive — which
the project treated as an adverse result rather than a good one, because a world nobody dies in is not
testing anything — now loses four.

**A real difference, from a real run.** On seed 42, turn 381, `M08` — whose tolerance is 40, the maximum
— ate a low snack while its satiety was 88. The snack restores 15, so 3 of it were destroyed, and 3 is
one more than the 2 a Low piece is allowed to spill on its own account. The reference decider refuses
that snack, and so would a creature with tolerance 19. You can see the line yourself:

```bash
cargo run --bin Mokiterions -- --policy individual --seed 42 --ticks 400 | grep food_consumed
```

```
tick=381 subject=M08 event=food_consumed result=food:F0257,class:low,satiety:88->100,energy:45->50
```

Turn 14 used to be this example — `M08` eating a snack at satiety 87, on the same run. That meal still
happens, but it is no longer a *difference*: satiety 87 is now exactly where the reference decider
accepts a snack as well, so no tolerance was needed to explain it.

Two honest limits on all of this:

- **Differences of that kind are now very rare.** Counted as "meals eaten that the reference decider
  would have refused", it is **0 to 3 per run by 0 to 2 different creatures**, and on seed 777 there is
  not one such meal in the whole thousand turns. Before the allowance was corrected on 2026-08-21 the
  same count was 54 to 97 per run by 9 or 10 creatures. Seed 777's two runs still diverge all the same,
  because the tolerance also decides what is worth *walking to*, and a path that differs changes who is
  standing where long after it has stopped changing who eats what.
- **The stricter count has not been re-measured.** "Moments where two creatures facing the genuinely
  same situation would have chosen differently" was 3 to 10 per run, and **never two such creatures on
  the same turn**, so you could not watch it happen side by side in one frame. That was measured on the
  pre-correction world and not repeated; the figure in the bullet above counts something else and is not
  a substitute for it.
- **A higher tolerance is not a better tolerance.** It stops helping well before the range runs out,
  which is why the range stops at 40 and not at 100 — see section 12. Since 2026-08-21 it also stops
  mattering sooner: on a feast it never matters at all, on a Medium piece only above 33, and on a snack
  only above 19.

### The social decider — `--policy social`

The first decider that lets creatures do anything to each other. It is the trait-aware decider with
three checks threaded into it, and it runs down six steps in order, doing the **first** thing that
applies:

1. **Somebody hit me.** Answer the first name on the list (section 6), and which way depends entirely
   on how afraid I already am: **surrender** at fear 60 or more, **retreat** at 30 to 59, **fight**
   below 30.
2. **Eat what I am standing on, or sleep if exhausted.** Survival comes before society, always. A
   starving creature does not stop to posture.
3. **Walk one step toward the nearest food worth reaching.** Food perceived outranks company
   perceived. This step used to sit *below* the next two and was lifted above them on 2026-08-20 —
   which is the single change that made this decider survivable, for the reason three paragraphs down.
4. **Somebody is in contact.** Engage the nearest: **attack** below fear 95, **threaten** at 95 or
   more.
5. **Somebody is in sight but not in contact.** **Approach** below fear 95, **avoid** at 95 or more.
6. **Nothing above applied.** Take a random step.

Steps 2, 3 and 6 are the trait-aware decider verbatim, tolerance and all, so a creature that is alone
behaves exactly as it did before. It never chooses "wait" either.

Fear is doing all the work in steps 1, 4 and 5, and the direction is worth noticing: **fear makes a
creature more submissive, never more violent.** Below 95 it engages, at 95 it starts backing off, and
on step 1 above 60 it pays. There is no rage.

**And now the measured part.** Over 1,000 turns on the five seeds the project uses, at the default
density, here is every targeted action the decider actually took — before the waste allowance of
section 12 was corrected on 2026-08-21, and after:

| Action | Before | Now |
|---|---:|---:|
| avoid | 3,842 | 4,268 |
| approach | 1,788 | 1,724 |
| threaten | 128 | 114 |
| attack | 59 | 54 |
| **surrender** | 36 | **31** |
| retreat | 14 | 14 |
| fight | 9 | 9 |

**Both columns replace a much gentler set of figures that this section carried until 2026-08-21** —
6,329 avoids, three attacks, no surrenders at all, and nobody ever dying of violence. That set was
measured before commit `7d744bb` reordered the five steps above — hoisting "walk toward food" ahead of
the social branches and raising the engagement threshold from 30 to 95 — and it was not re-measured when
that landed, so what this section described was a decider the project had already stopped shipping.
It is named here rather than quietly overwritten, because a stale number in a reader-facing document
is the same kind of defect as a stale number in a requirement.

Sixty-three strikes across five complete runs, and **fifty-six of them land inside the first fourteen
turns** — between creatures that happened to be *placed* near each other at the start. The other seven
are scattered as far out as turn 882, so the violence is front-loaded rather than confined to the
opening. All five seeds are violent, and **nine of the eighteen deaths across the five runs are caused
by combat**: one, two, two, three and one, seed by seed.

The mismatch section 8 describes is what front-loads it, and the gate is plainly visible in the numbers.
Fear rises by 10 for every turn you can *see* company at 16 squares and falls by only 5 when you are
alone, so it climbs to 100 and stays there; engaging needs company at 1 square **and** a fear below 95.
Ten turns of noticing anybody is enough to shut the gate for the rest of the run. Every one of the 63
strikes was proposed at a fear of 90 or lower — thirteen of them at exactly 90, the last rung before the
gate — and so was every one of the 1,724 approaches. Every one of the 114 threats was proposed at
exactly 100, and every one of the 4,268 avoidances at 95 or 100. Nothing straddles the line. So the
decider's forward half lives inside a creature's first ten turns of company and its submissive half owns
the remaining nine hundred and ninety: `avoid` outnumbers the strikes about seventy to one.

**That gate was 30 until 2026-08-20, which left three turns rather than ten**, and raising it is what
made the strikes land, the deaths happen, and `surrender` reachable in an ordinary run — thirty-one
times across the five — rather than only in a test. It was the product owner's chosen answer to the
mismatch, taken in preference to rescoping the fear rise itself, which is the alternative section 8
still describes as open.

Two things are recorded rather than papered over:

- The project's own target for this decider is **at least 5 of 12 alive at turn 1,000 and at least
  one death caused by combat, on every seed.** Measured on the current build: **9, 7, 9, 8 and 9
  survivors**, with **1, 2, 2, 3 and 1** combat deaths. Every seed meets both halves and the worst
  seed clears the floor by two.
- No test is failing, skipped or ignored to make that true: the workspace runs 268 tests green.

Part of that is progress and part of it is a cost. The survivor counts above are unchanged on two
seeds and one to three lower on the other three than the same build produced before the allowance was
corrected, because a world grazed harder feeds fewer creatures at turn 1,000 — section 12 has the
figures. The fear-versus-contact mismatch is still the open question, and moving those thresholds or
the order of the five steps is a decision taken and recorded before it is coded, not a number quietly
nudged until the tests pass. `social` is still **not** the default — `--policy` with nothing given
selects `reference`.

---

## 12. The rule that surprises everyone: don't waste food

A Mokiterion will refuse a feast it is standing on.

Nothing can go above 100. So eating a High piece (worth 50 satiety) at satiety 80 would waste 30 of
it. The reference decider will not do that. It eats when the food fits **completely**, and otherwise
only when the spill is no more than **the piece's own allowance**: `worth × worth ÷ 100`, whole
numbers rounded down. A bigger piece is allowed to spill a little more, because a bigger piece is
harder to fit anywhere. The trait-aware decider can widen that allowance but never narrow it, and by
how much is the second half of this section.

| Class | Worth | Own allowance | Eaten only when satiety is at most |
|---|---:|---:|---:|
| Low | 15 | 2 | **87** |
| Medium | 30 | 9 | **79** |
| High | 50 | 25 | **75** |

**This allowance was missing until 2026-08-21.** Both the reference test and the tolerant one below
were written without it, so a High piece was refused above satiety 50 rather than above 75. What that
cost is the "feasts pile up" bullet two paragraphs down, and the correction is `REQ-MOK-060`, with the
whole measurement in `docs/engineering/simulation/evidence/WO-MOK-017/`.

The same test decides whether it is worth *walking* to something. A creature will not cross the map
toward a feast it would refuse on arrival.

Applying the test to both eating *and* walking is essential, and getting it wrong caused a real bug.
An earlier version tested only eating. A creature would decline the food under its feet, step off it,
immediately notice that same food as "the nearest food nearby", and step back on. Then off. Then on.
It jittered in place while starving. Testing both sides fixed it: the food you just refused stops
being a destination for exactly as long as you would refuse it.

Two honest consequences of this rule, both known and accepted:

- **A well-fed Mokiterion wanders aimlessly.** Above satiety 87 nothing at all fits, so rule 3 never
  applies and it falls through to random steps until it gets hungry enough. You can watch this happen
  in section 14. The most tolerant creature stretches that to 91 and no further.
- **Feasts still pile up, but far less.** High pieces are wanted by hungrier creatures than snacks are,
  so they still get eaten last and still accumulate. By turn 1,000 at the default density the High
  share of a territory's standing food runs **33% to 54%** across the three deciders `REQ-MOK-060` binds
  — reference, trait-aware and social — and the five declared seeds, against a balanced starting third. Before the allowance was corrected it ran 36% to 82%, and
  the case `REQ-MOK-060` names — High standing at 45 of 61 in one territory, 73.8% — is gone: no class
  now holds more than three fifths of any territory on any declared seed, which is that requirement's
  obligation. Territories still look fuller than the food anybody will walk to.
- **The long horizon is not re-measured.** The extinction figures this section used to carry — everyone
  dead by tick 9,154 at the default density on seed 123, and the 10,000-turn comparison below — were
  measured on the **uncorrected** world. `REQ-MOK-060` binds turn 1,000 only, deliberately, and
  `WO-MOK-017` did not re-run the long horizon. Treat those numbers as history, not as a description of
  the simulation you have. Re-measuring them is recorded in `docs/ROADMAP.md`.

### Bending the rule: waste tolerance

Every creature has a number saying how much waste it will put up with (section 3). The trait-aware
decider uses it like this: a piece of food worth `R` satiety, eaten at satiety `S`, spills
`S + R − 100` if that is positive, and the creature accepts the piece when

> the spill is no more than **`T` percent of the piece** — where `T` is its waste tolerance — **or no
> more than the piece's own allowance from the table above, whichever of the two is larger.**

Whole numbers only, rounded down, and a piece that fits completely is always fine. The own allowance is
that same arithmetic at a tolerance equal to the piece's worth, so the second half of that sentence is
not a separate rule: it is a floor under the tolerance, and it is why no creature can end up stricter
than the reference decider. So the thresholds from the table above stretch as the tolerance rises —
where it rises past the floor at all:

| Class | Worth | Refused above satiety, at tolerance **0** | at tolerance **20** | at tolerance **40** |
|---|---:|---:|---:|---:|
| Low | 15 | **87** | 88 | **91** |
| Medium | 30 | **79** | 79 | **82** |
| High | 50 | **75** | 75 | **75** |

The left column is the reference decider exactly. The right column is the loosest any creature can
ever be. And notice what the shape of the rule now does — which is the opposite of what it did before
2026-08-21. Both the tolerance and the floor are a *percentage of the piece*, but the floor's
percentage is the piece's own worth and the tolerance stops at 40. On a High piece the floor of 25
beats even `40% of 50`, so **the trait changes nothing whatsoever on feasts**; on a Medium piece it
decides only above tolerance 33, and on a Low piece only above tolerance 19. Two creatures differing in
tolerance alone now often make the identical choice, and where they differ it is over snacks rather
than over feasts. That is a real narrowing of what the trait does, it was measured rather than assumed
(`docs/engineering/simulation/evidence/WO-MOK-017/post/divergence.txt`), and it is recorded as a
consequence of correcting the allowance and not as something anyone set out to do.

The same test still governs eating and walking together, for exactly the reason the two-cell jitter
bug taught, so no tolerance setting can reintroduce it.

**Why the range stops at 40 and not at 100.** It was originally written as 0 to 100, and that was
measured and thrown out. A creature at satiety 80 eating a feast gains 20 satiety and destroys 30 —
and it lands on 100, which a snack would also have reached. So tolerance past a certain point buys its
holder nothing while stripping a world whose food only regrows from food. The measurements agreed —
taken on the pre-2026-08-21 world, and not re-taken since: over the wider range the trait-aware decider
dropped below the required eight survivors on three of the five seeds, and on four of five seeds the
creatures that died had a *higher* average tolerance than the ones that lived. Cutting the range at 40
removed a stretch of the scale that was strictly worse, not a second way of living. The full working is
in `docs/engineering/simulation/evidence/WO-MOK-010/escalation.md`.

**What this did not fix.** Waste tolerance was never going to end the feast pile-up and did not. As
measured then, at turn 1,000 the share of standing food that is High ran 35% to 77% per territory under
the trait-aware decider against 45% to 75% under the reference one — no clean improvement either way.
`WO-MOK-017` re-measured both on its own pre-change captures and got 36% to 77% and 46% to 75%, so that
finding stands as recorded. What ended the pile-up was the missing allowance at the top of this section,
under `REQ-MOK-060`'s ceiling of three fifths; the corrected figures are in the third bullet above.

The 10,000-turn comparison this section used to close with was measured on the uncorrected world and has
not been re-run: the trait-aware runs lasted longer, reaching the limit on four of the five seeds with 1
to 5 survivors while the reference runs went extinct on four of the five, and the fifth trait-aware run
went extinct at tick 9,938 on seed 777 — which was then the seed that kept all twelve alive at turn
1,000, and no longer is. Nothing in the project claims a long-horizon result in either direction, and
since the correction nothing in the project has measured one.

---

## 13. Same input, same run, every time

Give the simulation the same seed, the same settings, and the same decider, and you get **exactly**
the same run — the same deaths on the same turns, character for character of output. Nothing is truly
random; the `--seed` number drives a formula that produces random-*looking* results reproducibly.

This is a hard requirement, not a nicety. It is what makes a claim like "10 survived on seed 42"
something you can check for yourself rather than take on trust.

That includes the traits. `M08` on seed 42 has waste tolerance 40 today, tomorrow, and on anybody
else's machine.

Three consequences worth knowing:

- Runs are only comparable to runs with the **same decider** and the **same density**. All four
  deciders draw on the random stream at different rates, so switching decider reshuffles everything
  downstream.
- Turning on `--trace-actions` does not change the run. It only prints more about it.
- **Adding traits and fear did not disturb any run that existed before them.** Each creature's trait
  is worked out by a little formula of its own, off to one side, which never touches the shared random
  stream — so a `baseline` or `reference` run at any seed prints exactly the same output today as it
  did before individuality existed, apart from the two new numbers appearing on their lines. That was
  checked over 42 recorded runs, character for character, with the new numbers stripped back out.
- **Giving everybody a name did not disturb anything either.** A name is looked up from a fixed list by
  the creature's number, so it costs no random draw at all — not even one off to the side, the way the
  trait does. Every run at every seed makes exactly the same decisions it made before the names existed,
  and the only difference in the output is the name now standing at the front of each creature's first
  line. That was checked the same way, over 90 recorded runs with the names stripped back out.

---

## 14. A real run, step by step

This is genuine output, not an illustration. Trok (`M05`), seed 42, default density, with
`--trace-actions`. Read `satiety:` as the value at the moment it decided.

| Tick | What it did | Satiety | Why |
|---:|---|---:|---|
| 1–13 | wandered: south, south, north, south, north, north, south, north, east, west, east, north, south | 100 → 88 | Too full. Even a Low snack (15) would spill more than the 2 a Low piece is allowed to — 15 of it on turn 1, and still 3 of it on turn 13 — so *nothing* is worth walking to. Falls through to random steps. |
| 14 | **move east**, from `85:15` toward `89:13` | 87 | Satiety just hit 87. Now 87 + 15 = 102, spilling exactly the 2 a Low piece is allowed — so a Low snack fits. Rule 3 switches on and it heads for `F0058`, six squares away, which it had been able to see the whole time. Sideways first, because the food lies to the north *east*. |
| 15 | move east → `87:15` | 86 | still walking |
| 16 | move east → `88:15` | 85 | still walking |
| 17 | move east → `89:15` | 84 | now directly below the food |
| 18 | move north → `89:14` | 83 | horizontal work done, so switch to the vertical axis |
| 19 | **move north** → `89:13` | 82 | arrives on the food |
| 20 | **eat `F0058`** (low) | 81 → 96 | Standing on it at last. 81 + 15 = 96, and 96 is 4 short of full, so nothing at all was wasted. |
| 21 | move east | 95 | Full again. Back to wandering. |

Everything in section 12 is visible here. It could see the food from the very first turn and
deliberately ignored it for thirteen turns, because eating it then would have spilled more than a Low
piece is allowed to. The instant its satiety crossed the threshold it went straight there — four steps
east until it was directly below the food, then two north — and ate. Trok's own waste tolerance is 21,
which the default decider ignores; the trait-aware one would have admitted that snack one point of
satiety earlier, at 88. Not that you could watch it in this run — switch decider and the whole thousand
turns are a different run, and Trok is somewhere else entirely (section 13).

---

## 15. Running it yourself

```bash
cargo run --bin Mokiterions -- --help                      # what all the options do
cargo run --bin Mokiterions                                # defaults: seed 0, 100 turns, food-seeker, 0.75%
cargo run --bin Mokiterions -- --seed 42 --ticks 1000      # the standard measurement
cargo run --bin Mokiterions -- --policy baseline           # watch the random decider starve
cargo run --bin Mokiterions -- --policy individual         # the trait-aware decider
cargo run --bin Mokiterions -- --density 1.5               # a kinder world
cargo run --bin Mokiterions -- --ticks 40 --trace-actions  # show every single decision
```

`--bin Mokiterions` says which program to run. There are two: this one, which prints the run as
text, and `mokiterions-tui`, which shows the same run in a live terminal display — with a bar each for
health, satiety, energy and now fear beside every living creature. If you want that one instead,
`cargo run -p mokiterions-tui -- --help` will tell you how.

Exit codes: `0` fine, `2` you typed something invalid, `1` something broke while running.

### Reading the output

Every line has the same shape: the turn, who it is about, what happened, and the details.

Five real lines, all from `--seed 42 --ticks 40`:

```
tick=0  subject=F0002 event=food_initialized  result=class:medium,position:82:20,territory:A
tick=0  subject=M08   event=agent_initialized result=name:Nurb,position:62:104,territory:B,health:100,satiety:100,energy:100,fear:0,waste_tolerance:40
tick=10 subject=A     event=food_regeneration_skipped result=reason:capacity,count:61
tick=40 subject=A     event=food_regenerated  result=food:F0129,class:medium,position:56:30
tick=40 subject=M12   event=survival_changed  result=health:100->100,satiety:91->90,energy:71->70,fear:100->100
```

In order: a medium piece placed at the start; `M08` arriving in the world and giving its name as Nurb,
on the one and only line that will ever state either its name or its waste tolerance — every later line
about it says `M08` and nothing else; territory A growing nothing on turn 10 because it is already
at its 61-piece cap; the same territory growing a meal on turn 40, after somebody had eaten; and `M12`
getting hungrier and more tired at the end of turn 40.

That last line is where fear shows up. `fear:100->100` means `M12` could see somebody *and* was already
at the top of the range, so nothing moved. A rise from 30 would print `fear:30->40` and a fall
`fear:30->25`. Every living creature gets one of these lines every turn, so fear is always visible.

Three more lines exist, and only `--policy social` produces them. All three are real. The first two are
from `--policy social --seed 123` and `--policy social --seed 123 --density 1.50`:

```
tick=1   subject=M11 event=attack_resolved result=target:M10,damage:30,target_health:100->70,striker_energy:100->95,target_died:no
tick=337 subject=M02 event=threat_resolved result=target:M04,increase:5,target_fear:95->100
```

`M11` hits `M10` for 30 — the hardest anybody can hit — and both sides of it are on the one line,
including whether it was fatal. The second is a threat that only partly landed: the threat is worth 30,
but `M04` was already at 95, so the number after `increase:` is the 5 that actually arrived. That line
is rarer than it looks — across all sixty social captures `WO-MOK-017` took, at both commits, and the
2,850 threats they resolved between them, it is **the only threat that moved its target's fear at
all.** Every other one lands on somebody already sitting at 100 and prints
`increase:0,target_fear:100->100`, which is the fear-versus-contact mismatch of section 11 showing up
one event at a time.

The third is from `--policy social --seed 123` at the default density:

```
tick=7 subject=M06 event=surrender_resolved result=recipient:M04,transferred:7,discarded:40,subject_satiety:94->47,recipient_satiety:93->100
```

`M06` gives up half of 94, so 47 leaves it; `M04` was already at 93, so 7 arrives and 40 is destroyed.
`transferred` and `discarded` differ precisely when something was wasted, and the payer is out 47
either way. This section used to say that no run had ever produced a surrender and print a
hand-constructed example instead, on the strength of the figures section 11 now corrects. They happen
thirty-one times across the five declared seeds.

There is no line of its own for `approach`, `avoid` or `retreat`. They are moves, and they report as
moves. To see who *proposed* what, including the refusals and the reason for each, add
`--trace-actions`:

```
tick=2 subject=M10 event=action_trace result=proposal:fight,target:M11,status:accepted,detail:damage:26,position:103:102,territory:B,health:70,satiety:99,energy:94,fear:10,suffered:M11:30
```

`M10` fights back at `M11` and the whole reason is on the line: `suffered:M11:30` is its one-turn
memory (section 6), read as `who:how-much`, and it is the only place that list is ever visible. Empty
means nobody hit you. Note the fear of 10 — under 30, which is exactly why this came out as `fight`
and not `retreat`.

The last line is always the scoreboard:

```
summary reason=tick_limit ticks=40 survivors=12 deaths=0 territory_a=5 territory_b=7
        food_a_low=17 food_a_medium=22 food_a_high=20 food_b_low=18 food_b_medium=19 food_b_high=19
```

Stopped because it hit the turn limit, after 40 turns, everybody alive, five creatures in one half and
seven in the other, and the food left over broken down by half and by size. The other possible `reason`
is `extinction`. Note that this early the three sizes are still roughly even — it is over hundreds of
turns that the High pieces pile up, for the reason in section 12.

---

## 16. What the simulation does *not* do yet

Worth stating plainly, because the project's eventual goals make people expect more than is there:

- **One trait, not a personality.** There is exactly one thing that differs between creatures — waste
  tolerance — and it only affects what they will eat. No caution, no aggression, no sociability, no
  preferences. Two creatures at the same fear make the same social choice; the aggression is a
  property of the situation, not of anybody.
- **Fighting happens, and then stops.** Creatures can strike, threaten, back away and give in, and with
  `--policy social` they do — 63 strikes across five 1,000-turn runs, of which 56 land inside the first
  fourteen turns, and nine of the eighteen deaths are caused by combat. So it kills, but it is an
  opening event rather than a way of life: after the first two dozen turns almost everybody is too
  frightened to engage anybody. Section 11 has the diagnosis and the figures. The other three deciders
  cannot interact at all and are unchanged.
- **No cooperating.** Nothing anybody can do helps anybody else. `surrender` transfers food, and it
  is a payment under duress, not a gift.
- **Almost no memory.** One thing is remembered, for exactly one turn: who hit you (section 6).
  Everything else in a decision comes from what is visible right now — including how afraid you were,
  which nobody stores because it is a live attribute rather than a recollection.
- **No AI or language model.** All four deciders are a few lines of fixed rules.
- **No graphics, no saved files, no network.** Text output only.

All of these are planned. The order they arrive in, and why, is in `docs/ROADMAP.md`.

---

## 17. Where to go next

| To find out | Read |
|---|---|
| The exact, binding rules | `docs/engineering/simulation/specifications/SPEC-MOK-001.md` |
| What is planned, and in what order | `docs/ROADMAP.md` |
| What was measured, and how | `docs/engineering/simulation/evidence/WO-MOK-002/`, `.../WO-MOK-010/`, `.../WO-MOK-016/` and `.../WO-MOK-017/` |
| How changes get approved here | `ENGINEERING_HARNESS.md` |
