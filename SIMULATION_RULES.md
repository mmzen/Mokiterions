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
map, but not much. Each creature can look around, walk one step, or eat what it is standing on.
Nobody tells them what to do as a group — each one decides for itself, one at a time, over and over.

The interesting question is: **who is still alive after 1,000 turns?**

The answer is usually 8 to 11 of the 12, and 9 to 12 with the newest of the three deciders described
in section 11. Some always die. That is on purpose. A world where everyone survives easily would be
boring, and a world where everyone dies would tell us nothing.

The twelve are also no longer interchangeable. Each is born with one fixed quirk — how much food it
is willing to waste — which changes what it will eat and what it will walk to (section 3, section
11). Each also carries a mood, `fear`, that rises when it is not alone. The quirk changes behaviour.
**The mood, so far, changes nothing at all** (section 8).

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
five seeds the project measures. What the number actually *does* is section 12 — and only the newest
decider reads it at all. Under the other two it is derived, reported, and ignored.

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
   - **It looks around.** The simulation hands it a description of what it can see (see section 7).
   - **It picks one action** — just one — from wait, sleep, eat, or move.
   - **The simulation checks the action is legal**, and applies it if so.
   - **Then it gets hungrier and more tired, and its fear moves** (see section 8).
4. **After everyone has moved, food may regrow** — but only on ticks that divide by 10 (tick 10, 20,
   30 …). See section 9.
5. **The simulation checks whether to stop**: either everyone is dead, or the turn limit was reached.

Two details that matter more than they look:

- Order is fixed and always the same. `M01` always acts before `M02`. If `M01` eats the last piece of
  food on a square, `M02` arrives to find nothing there. Being early is an advantage.
- One action per turn. A creature cannot walk *and* eat in the same turn. Walking onto food means
  eating it next turn at the earliest.

---

## 6. The four things a Mokiterion can do

That is the complete list. There is nothing else.

| Action | What it does | What it costs |
|---|---|---|
| **move** | step one square north, east, south, or west | nothing extra |
| **eat** | consume a piece of food **on your own square**, removing it from the world | nothing extra |
| **sleep** | recover 20 energy | nothing extra |
| **wait** | do nothing at all | nothing extra |

Rules and limits:

- **Movement is one square at a time, and never diagonal.** To reach a square 3 east and 2 north, you
  need 5 turns.
- **You can only eat what is under your feet.** Not next to you — under you.
- Eating raises satiety and energy by the amounts in the food table, and the food is gone for good.
- Nothing can push any number above 100. Extra is simply lost.
- Trying something illegal — walking off the edge of the map, eating food that is not there — is not
  a crash. The turn is spent, nothing happens, and the simulation records that the attempt was
  rejected.

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

It adds up rather more quickly than the numbers suggest. Measured over 111,604 creature-turns across
every seed and every decider: **fear is sitting at exactly 100 on 39% of them**, and above 0 on
roughly half to two thirds. There is also one step size the table above does not predict — **+5**,
which occurred 219 times. That is a creature at 95 gaining 10 and stopping at 100.

**And now the honest part: nothing reads fear.** No rule consults it, no decider consults it, no
creature behaves differently because of it. It is computed, kept in range, reported on every
`survival_changed` line and on the observer's roster, and that is the end of it. Which means the two
numbers `+10` and `−5` cannot be shown to be right or wrong by anything that happens in the world —
there is no outcome for them to be wrong *about* yet. Fear exists now so that the thing that will use
it, in a later phase, does not have to invent it. Until then, treat that 39% as a fact about the
current constants and not as a finding.

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
same world with more food in it. Measured example: on seed `0`, a density of `0.50%` leaves ten alive
while `0.75%` leaves eight. Only the default `0.75%` carries a promise about survivors; every other
density is unexplored territory.

---

## 11. Who decides what a Mokiterion does

Here is the part that matters most for understanding the project.

**The creatures do not control the world.** When it is a creature's turn, it is handed a read-only
description of its surroundings and hands back a *request*: "I would like to move east." It cannot
change anything itself. The simulation checks the request and decides what actually happens.

This separation is the whole point of the design. It means the thing making decisions can be swapped
out — for something smarter, later — without it ever being able to cheat. The newest decider reads
each creature's waste tolerance, and that changes nothing about the arrangement: it still only asks,
and the simulation still decides.

Three deciders exist today. Pick one with `--policy`.

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
  of 2,808 possible situations was checked one by one, and it proposed the identical action in all of
  them. So the new decider contains the old one as its own most careful member.
- **A creature with tolerance 40**, the highest possible, will eat a feast at satiety 70 where the
  reference decider stops at 50. See the table in section 12.

Measured over 1,000 turns at the default density, on the five seeds the project uses:

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---:|---:|---:|---:|---:|
| survivors, trait-aware | 11 | 9 | 9 | 10 | **12** |
| survivors, reference | 8 | 11 | 8 | 9 | 11 |

It is not simply better. It wins on three of the five seeds, loses on one, and on seed 777 keeps all
twelve alive — which the project treats as an adverse result rather than a good one, because a world
nobody dies in is not testing anything.

**A real difference, from a real run.** On seed 42, turn 14, `M08` — whose tolerance is 40, the maximum
— ate a low snack while its satiety was 87. The snack restores 15, so 2 of it were destroyed. The
reference decider refuses any snack above satiety 85, so it would have walked away. You can see the
line yourself:

```bash
cargo run --bin Mokiterions -- --policy individual --seed 42 --ticks 20 | grep food_consumed
```

Two honest limits on all of this:

- **Differences of that kind are rare.** Across a 1,000-turn run there are only about 3 to 10 moments
  where two creatures facing the genuinely same situation would have chosen differently — and in the
  runs measured, **never two such creatures on the same turn**, so you cannot watch it happen side by
  side in one frame. Counted the looser way, as "meals eaten that the reference decider would have
  refused", it is 54 to 97 per run by 9 or 10 different creatures. Both numbers are real; they measure
  different things.
- **A higher tolerance is not a better tolerance.** It stops helping well before the range runs out,
  which is why the range stops at 40 and not at 100 — see section 12.

---

## 12. The rule that surprises everyone: don't waste food

A Mokiterion will refuse a feast it is standing on.

Nothing can go above 100. So eating a High piece (worth 50 satiety) at satiety 80 would waste 30 of
it. The reference decider will not do that. It only eats when the food fits **completely**. The
trait-aware decider bends this, and by how much is the second half of this section.

| Class | Worth | Eaten only when satiety is at most |
|---|---:|---:|
| Low | 15 | **85** |
| Medium | 30 | **70** |
| High | 50 | **50** |

The same test decides whether it is worth *walking* to something. A creature will not cross the map
toward a feast it would refuse on arrival.

Applying the test to both eating *and* walking is essential, and getting it wrong caused a real bug.
An earlier version tested only eating. A creature would decline the food under its feet, step off it,
immediately notice that same food as "the nearest food nearby", and step back on. Then off. Then on.
It jittered in place while starving. Testing both sides fixed it: the food you just refused stops
being a destination for exactly as long as you would refuse it.

Two honest consequences of this rule, both known and accepted:

- **A well-fed Mokiterion wanders aimlessly.** Above satiety 85, nothing at all fits, so rule 3 never
  applies and it falls through to random steps until it gets hungry enough. You can watch this happen
  in section 14.
- **Feasts pile up.** High pieces are only wanted by quite hungry creatures, so they get eaten last
  and accumulate. By turn 1,000 at the default density, roughly three quarters of the remaining food
  is High. Territories look full while the food anybody will actually walk to has run out. Over very
  long runs this is fatal: at the default density on seed 123, everyone is dead by tick 9,154. Nothing
  promised otherwise — the survivor target is about turn 1,000 — and fixing it is scheduled work,
  recorded in `docs/ROADMAP.md`.

### Bending the rule: waste tolerance

Every creature has a number saying how much waste it will put up with (section 3). The trait-aware
decider uses it like this: a piece of food worth `R` satiety, eaten at satiety `S`, spills
`S + R − 100` if that is positive, and the creature accepts the piece when

> the spill is no more than **`T` percent of the piece**, where `T` is its waste tolerance.

Whole numbers only, rounded down, and a piece that fits completely is always fine. So the thresholds
from the table above stretch as the tolerance rises:

| Class | Worth | Refused above satiety, at tolerance **0** | at tolerance **20** | at tolerance **40** |
|---|---:|---:|---:|---:|
| Low | 15 | **85** | 88 | **91** |
| Medium | 30 | **70** | 76 | **82** |
| High | 50 | **50** | 60 | **70** |

The left column is the reference decider exactly. The right column is the loosest any creature can
ever be. And notice what the shape of the rule does: it is a *percentage of the piece*, so a tolerant
creature bends most on feasts, which are the pieces the strict rule leaves lying around.

The same test still governs eating and walking together, for exactly the reason the two-cell jitter
bug taught, so no tolerance setting can reintroduce it.

**Why the range stops at 40 and not at 100.** It was originally written as 0 to 100, and that was
measured and thrown out. A creature at satiety 80 eating a feast gains 20 satiety and destroys 30 —
and it lands on 100, which a snack would also have reached. So tolerance past a certain point buys its
holder nothing while stripping a world whose food only regrows from food. The measurements agreed:
over the wider range the trait-aware decider dropped below the required eight survivors on three of
the five seeds, and on four of five seeds the creatures that died had a *higher* average tolerance
than the ones that lived. Cutting the range at 40 removed a stretch of the scale that was strictly
worse, not a second way of living. The full working is in
`docs/engineering/simulation/evidence/WO-MOK-010/escalation.md`.

**What this did not fix.** The feast pile-up above is untouched, and no promise was made about it.
At turn 1,000 the share of standing food that is High runs 35% to 77% per territory under the
trait-aware decider against 45% to 75% under the reference one — no clean improvement either way.
Over 10,000 turns the trait-aware runs do last longer, reaching the limit on four of the five seeds
with 1 to 5 survivors while the reference runs go extinct on four of the five — but the fifth
trait-aware run goes extinct too, at tick 9,938, and that is seed 777, the same seed that keeps all
twelve alive at turn 1,000. Nothing in the project claims a long-horizon result in either direction.

---

## 13. Same input, same run, every time

Give the simulation the same seed, the same settings, and the same decider, and you get **exactly**
the same run — the same deaths on the same turns, character for character of output. Nothing is truly
random; the `--seed` number drives a formula that produces random-*looking* results reproducibly.

This is a hard requirement, not a nicety. It is what makes a claim like "8 survived on seed 42"
something you can check for yourself rather than take on trust.

That includes the traits. `M08` on seed 42 has waste tolerance 40 today, tomorrow, and on anybody
else's machine.

Three consequences worth knowing:

- Runs are only comparable to runs with the **same decider** and the **same density**. All three
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
| 1–15 | wandered: south, south, north, south, north, north … | 100 → 86 | Too full. Even a Low snack (15) would overflow 100, so *nothing* is worth walking to. Falls through to random steps. |
| 16 | **move east**, from `86:14` toward `89:13` | 85 | Satiety just hit 85. Now 85 + 15 = 100 exactly — a Low snack fits perfectly. Rule 3 switches on and it heads straight for `F0058`, three squares away, which it had been able to see the whole time. Sideways first, because the food lies to the north *east*. |
| 17 | move east → `88:14` | 84 | still walking |
| 18 | move east → `89:14` | 83 | now directly below the food |
| 19 | **move north** → `89:13` | 82 | horizontal work done, so switch to the vertical axis. Arrives on the food. |
| 20 | **eat `F0058`** (low) | 81 → 96 | Standing on it at last. 81 + 15 = 96. |
| 21 | move north | 95 | Full again. Back to wandering. |

Everything in section 12 is visible here. It could see the food from the very first turn and
deliberately ignored it for fifteen turns, because eating it then would have wasted part of it. The
instant its satiety crossed the threshold, it walked four squares in a straight line and ate.

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
cargo run --bin Mokiterions -- --events-path run.jsonl     # also write the run to a file, for a program
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
tick=40 subject=A     event=food_regenerated  result=food:F0128,class:medium,position:110:32
tick=40 subject=M12   event=survival_changed  result=health:100->100,satiety:91->90,energy:71->70,fear:100->100
```

In order: a medium piece placed at the start; `M08` arriving in the world and giving its name as Nurb,
on the one and only line that will ever state either its name or its waste tolerance — every later line
about it says `M08` and nothing else; territory A growing nothing on turn 10 because it is already
at its 61-piece cap; the same territory growing a meal on turn 40, after somebody had eaten; and `M12`
getting hungrier and more tired at the end of turn 40.

That last line is where fear shows up. `fear:100->100` means `M12` could see somebody *and* was already
at the top of the range, so nothing moved. A rise from 30 would print `fear:30->40` and a fall
`fear:30->25`. Every living creature gets one of these lines every turn, so fear is always visible even
though nothing acts on it.

The last line is always the scoreboard:

```
summary reason=tick_limit ticks=40 survivors=12 deaths=0 territory_a=5 territory_b=7
        food_a_low=19 food_a_medium=21 food_a_high=20 food_b_low=16 food_b_medium=21 food_b_high=21
```

Stopped because it hit the turn limit, after 40 turns, everybody alive, five creatures in one half and
seven in the other, and the food left over broken down by half and by size. The other possible `reason`
is `extinction`. Note that this early the three sizes are still roughly even — it is over hundreds of
turns that the High pieces pile up, for the reason in section 12.

### The same run, written for a program

Everything above is written for a person to read. `--events-path` writes the same run a second time,
to a file, in a shape a program can read:

```bash
cargo run --bin Mokiterions -- --seed 42 --ticks 40 --events-path run.jsonl
```

One record per line, each line a self-contained JSON object, and the `record` field says which of the
four kinds it is. All four, from that exact run, with the long ones cut short:

```
{"record":"header","schema":1,"engine":"0.1.0","config":{"seed":42,"ticks":40,"policy":"reference","density":"0.75","trace_actions":false}}
{"record":"event","tick":10,"subject":"A","event":"food_regeneration_skipped","result":{"reason":"capacity","count":61}}
{"record":"metrics","tick":40,"living":12,"deaths":0,"population":{"A":5,"B":7},"health":{"sum":1200,"min":100}, … }
{"record":"run","reason":"tick_limit","ticks":40,"survivors":12,"deaths":0,"crossings":1,"consumed":{"low":12,"medium":1,"high":0}, … }
```

- **`header`**, once, at the top: the settings the run actually used — not the ones you typed, the ones
  it resolved, with every default filled in. `schema` is the version of the record shape itself, so a
  program can tell whether it understands the file before it reads it.
- **`event`**, one for every `tick=` line the text stream printed, in the same order, carrying the same
  values. The second line above is the `food_regeneration_skipped` line from the five earlier in this
  section. Nothing is added and nothing is dropped, so either stream can be rebuilt from the other.
- **`metrics`**, one at the end of every completed turn: how many are alive, how many have died, where
  the living ones are, the totals and the extremes of the four attributes, and how much food is standing
  in each half and whether it has run out. This is the one thing the text stream never had — getting it
  out of the text meant adding event lines up yourself, and getting it right meant knowing every rule.
- **`run`**, once, at the bottom: the scoreboard line's twelve figures, plus the totals for the whole
  run — crossings, pieces eaten by size, regrowths, regrowths skipped and why — plus all twelve
  creatures by name with the turn each died on, or `null` for the ones that did not.

Two things the file deliberately does not contain. **No judgement.** Nothing in it says a run went well
or badly, or calls a number high, low, healthy or concerning. It states what happened and stops.
Deciding what that means is the reader's job, and that is a rule rather than an oversight — the
moment a file starts labelling its own numbers, everyone downstream inherits somebody else's opinion.
**No clocks.** No timestamps, no durations, nothing about how long anything took. So the same seed and
settings, from the same build, produce a byte-for-byte identical file on any machine on any day, which
makes comparing two files a real test: anything that differs is a difference in the simulation.

Four practical things:

- The text output is unchanged, byte for byte, whether you ask for a file or not. The file is added;
  nothing is traded away for it.
- The file replaces whatever was already at that path.
- If something goes wrong while the run is going — the disk fills up, the terminal you were printing to
  goes away — the run stops, the exit code is `1`, and the half-written file is deleted. A file that
  exists is a whole run. The one exception: if a file was already sitting at that path before this run
  started, it is left where it is rather than deleted, and the reason is printed. Deleting somebody
  else's file is a worse outcome than leaving a partial one behind.
- A path that cannot be opened at all is reported and then nothing runs — not one turn. A run that
  cannot be recorded is not run.

---

## 16. What the simulation does *not* do yet

Worth stating plainly, because the project's eventual goals make people expect more than is there:

- **One trait, not a personality.** There is exactly one thing that differs between creatures — waste
  tolerance — and it only affects what they will eat. No caution, no aggression, no sociability, no
  preferences.
- **Fear is measured but unused.** It rises, falls, stays in range and gets reported, and no rule and
  no decider reads it. It changes nobody's behaviour. This is deliberate: it is there for the fighting
  and fleeing that come next, and it is honest to call it inert until then.
- **No fighting, no threatening, no fleeing, no cooperating.** Creatures cannot interact at all. They
  can see each other, they get uneasy about it, and that is the end of it.
- **No memory.** Each decision is made from what is visible right now. Nothing is remembered between
  turns — including how afraid you were.
- **No AI or language model.** All three deciders are a few lines of fixed rules.
- **No graphics and no network.** A run can now write itself to a file, as the end of section 15
  describes, but that file is a record of one run and nothing reads it back: there is no saved state,
  nothing to resume, and no way to start a run from anything other than a seed and a handful of options.
- **Nothing that reads the file.** The record stream exists so that something can be built on it. That
  something is not here yet — no reader, no analysis, no comparison across runs, no batch of seeds.

All of these are planned. The order they arrive in, and why, is in `docs/ROADMAP.md`.

---

## 17. Where to go next

| To find out | Read |
|---|---|
| The exact, binding rules | `docs/engineering/simulation/specifications/SPEC-MOK-001.md` |
| The exact shape of every record in the file | `docs/engineering/simulation/specifications/SPEC-MOK-006.md` |
| What is planned, and in what order | `docs/ROADMAP.md` |
| What was measured, and how | `docs/engineering/simulation/evidence/WO-MOK-002/`, `.../WO-MOK-010/` and `.../WO-MOK-012/` |
| How changes get approved here | `ENGINEERING_HARNESS.md` |
