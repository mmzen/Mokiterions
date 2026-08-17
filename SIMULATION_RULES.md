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

The answer is usually 8 to 11 of the 12. Some always die. That is on purpose. A world where everyone
survives easily would be boring, and a world where everyone dies would tell us nothing.

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

There are exactly **twelve**, named `M01` to `M12`. Six start in territory A (`M01`–`M06`) and six
start in territory B (`M07`–`M12`), at random positions, never two on the same square at the start.

Each one carries three numbers, each from 0 to 100:

| Number | What it means | Starts at |
|---|---|---:|
| **health** | how alive you are. Reaches 0 and you are dead, permanently | 100 |
| **satiety** | how *full* you are. This is hunger, counted upwards | 100 |
| **energy** | how rested you are | 100 |

**Satiety is the word to remember.** It is not hunger — it is the opposite. 100 means completely
full, 0 means completely empty. It only goes up by eating.

That is the whole creature. No name, no personality, no memory, no fear, no preferences. All twelve
are identical apart from their number and their position. (Giving them individual personalities is
planned work, not something the simulation does today.)

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
   - **Then it gets hungrier and more tired** (see section 8).
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

## 8. Getting hungry, and dying

At the end of every single turn, for every living creature:

- **satiety drops by 1**
- **energy drops by 1**
- **and then**, if either of those has hit 0, **health drops by 5**

Neither number can go below 0. Health reaching 0 means death, and death is final: no more turns, no
more looking around, no more anything.

So a creature that starts full and never eats:

- runs out of satiety after **100 turns**,
- then loses 5 health per turn, 20 times,
- and **dies on tick 119**.

That is not an estimate. Run `cargo run -- --seed 42 --ticks 200 --density 0.02` — a world with a
single piece of food in each half — and **eleven of the twelve die together on tick 119**. The
twelfth happened to be standing near the one piece of food, ate it, and lasted until tick 134.

Notice that **sleep saves you from tiredness but nothing saves you from hunger except food**. Energy
has a free refill; satiety does not. That is why hunger is what actually kills everyone, and why the
whole simulation is really about food.

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
out — for something smarter, later — without it ever being able to cheat.

Two deciders exist today. Pick one with `--policy`.

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

---

## 12. The rule that surprises everyone: don't waste food

A Mokiterion will refuse a feast it is standing on.

Nothing can go above 100. So eating a High piece (worth 50 satiety) at satiety 80 would waste 30 of
it. The reference decider will not do that. It only eats when the food fits **completely**:

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
  long runs this is fatal: at the default density, everyone is dead by tick 9,154. Nothing promised
  otherwise — the survivor target is about turn 1,000 — and fixing it is scheduled work, recorded in
  `docs/mokiterions/ROADMAP.md`.

---

## 13. Same input, same run, every time

Give the simulation the same seed, the same settings, and the same decider, and you get **exactly**
the same run — the same deaths on the same turns, character for character of output. Nothing is truly
random; the `--seed` number drives a formula that produces random-*looking* results reproducibly.

This is a hard requirement, not a nicety. It is what makes a claim like "8 survived on seed 42"
something you can check for yourself rather than take on trust.

Two consequences worth knowing:

- Runs are only comparable to runs with the **same decider** and the **same density**. The two
  deciders draw on the random stream at different rates, so switching decider reshuffles everything
  downstream.
- Turning on `--trace-actions` does not change the run. It only prints more about it.

---

## 14. A real run, step by step

This is genuine output, not an illustration. `M05`, seed 42, default density, with `--trace-actions`.
Read `satiety:` as the value at the moment it decided.

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
cargo run -- --help                      # what all the options do
cargo run                                # defaults: seed 0, 100 turns, food-seeker, 0.75%
cargo run -- --seed 42 --ticks 1000      # the standard measurement
cargo run -- --policy baseline           # watch the random decider starve
cargo run -- --density 1.5               # a kinder world
cargo run -- --ticks 40 --trace-actions  # show every single decision
```

Exit codes: `0` fine, `2` you typed something invalid, `1` something broke while running.

### Reading the output

Every line has the same shape: the turn, who it is about, what happened, and the details.

```
tick=0  subject=F0002 event=food_initialized  result=class:medium,position:82:20,territory:A
tick=40 subject=M12   event=survival_changed  result=health:100->100,satiety:61->60,energy:61->60
tick=40 subject=A     event=food_regenerated  result=food:F0027,class:low,position:57:27
tick=40 subject=B     event=food_regeneration_skipped result=reason:capacity,count:12
```

In order: a medium piece placed at the start; `M12` getting hungrier at the end of turn 40; territory
A growing a new snack; territory B growing nothing because it is already full.

The last line is always the scoreboard:

```
summary reason=tick_limit ticks=40 survivors=12 deaths=0 territory_a=6 territory_b=6
        food_a_low=4 food_a_medium=4 food_a_high=4 food_b_low=3 food_b_medium=5 food_b_high=4
```

Stopped because it hit the turn limit, after 40 turns, everybody alive, six in each half, and the
food left over broken down by half and by size. The other possible `reason` is `extinction`.

---

## 16. What the simulation does *not* do yet

Worth stating plainly, because the project's eventual goals make people expect more than is there:

- **No fear, no personality, no individuality.** All twelve are interchangeable.
- **No fighting, no threatening, no fleeing, no cooperating.** Creatures cannot interact at all. They
  can see each other and that is the end of it.
- **No memory.** Each decision is made from what is visible right now. Nothing is remembered between
  turns.
- **No AI or language model.** The reference decider is a few lines of fixed rules.
- **No graphics, no saved files, no network.** Text output only.

All of these are planned. The order they arrive in, and why, is in `docs/mokiterions/ROADMAP.md`.

---

## 17. Where to go next

| To find out | Read |
|---|---|
| The exact, binding rules | `docs/engineering/simulation/specifications/SPEC-MOK-001.md` |
| What is planned, and in what order | `docs/mokiterions/ROADMAP.md` |
| What was measured, and how | `docs/engineering/simulation/evidence/WO-MOK-002/` |
| How changes get approved here | `ENGINEERING_HARNESS.md` |
