# MASTER MIND
v0.1

## What is this game ?
*Master mind* is a very simple guessing game.
A secret set is hidden composed of 4 colors, with possibly multiple same, choosen from a pool of 8 or 9.
The goal is to uncover in less than 8 guess that hidden set.
Guesses consist of placing a set; then the opponents gives hints.
These hints are how many colors you have placed correctly (⚪) and how many colors are in the hidden set but missplaced (🔴).
Take this exemple:
> The hidden set is: `🟥🟨🟩🟦`;
>
> If one guesses is: `🟦🟨🟥🟫`;
>
> The hints are: 1⚪ 2🔴, because the yellow is **exact**, the blue and red **exists**, and the marroon is straight up **non-existant**, **null**.
The games and when the guesser is out of guesses or that it found the hidden set, given 4 **exact** hint

## The application.
For this computer and rudimentary version, few things will be simplified because of UI limitation and laziness:
- Hints will have different symbols;
- Set are not composed of colors, but of number. The pool of choices if just a range of natural numbers;
- E.g: [1, 2, 3, 4] is set.
These modifications allows a modular gameplay and create rules:
- The pool of choices can be of any size;
- The set lenght too;
- And the maximum amount of tries is customizable.

## The Engine.
Only a remake in a video-game would be lame! This is why the main focus point of the project is the solver-engine.
This bot has of objective to **play the best guesses to garranty the fastest victory** (for v0.1, it ain't, guess average is 5.5~7).
To make the best choices, it uses the *Information theory* to take the guess that will, in average, **divide the most the entire set of possiblities**.
There's a value that describe that, and its entropy, defined as:
> `E(set) = SUM(p(h) * -log(2; p(h)))`

Where p(h) is the probability of a hint, given a set, and such that the entropy is the sum of all hint's probabilites multiplied by minus log base 2 of the probability.
In other word, one meaning of the entropy is the **amount of times that it doubles precision**.
To see more, see the main inspiration of this project: [3Blue1Brown's information theory video](https://www.youtube.com/watch?v=v68zYyaEmEA).

## Using it.
> [!TIP]
> While choosing, **[ values in brackets ]** describe the availables choices and **( value in parentheses )** is the default value if you press Enter. 
First thing first, a game mode / player has to be choosen between `Human`, `Robot`, `Assisted`, `RobotBenchmark`;

- `Human` is the classic mode, where the player enter the guesses manually;

- `Robot` lets the engine auto-play;

- `Assisted` mode means that human player still manually take the guesses but the engine will show them its recommendations;

- `RobotBenchmark` mode purpose is to test the performence of the engine (for v0.1, W.I.P.).

By default, debug mode is activated (for v0.1, not revertable) and will be printed default values
If you want to change them, say `yes` for `advanced configuration`.

Some limitations (for v0.1):
- There is no cache. So each time the program runs, it has to recalculate the thousands of possibilites;
- It can not be used to cheat real-life game because there no option to enter a guess and the resulting hint. 

## Miscellaneous
Why Rust lang ?
> I'm currently learning it and it compiles, unlike Python; 
> Not for it's speed, security and efficency.

Inspirations ?
> As said earlier, [3Blue1Brown Wordle and Information theory video](https://www.youtube.com/watch?v=v68zYyaEmEA).

## License
Do whatever you want, just mention me (@Detroix23).
