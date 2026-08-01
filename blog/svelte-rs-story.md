# The story of how I wrote a Svelte compiler in Rust

Everything was slow: builds, dev server, svelte-check, the LSP. So I went off to write my own Svelte compiler in Rust. Spent five and a half months hacking at it by hand and gave up. Came back a year later with an AI agent and finished it in 106 evenings and $700.

---

Hi everyone! I want to tell you about my journey writing a Svelte compiler in Rust, my motivations and what came out of it.

The compiler lives here: https://github.com/MrWaip/svelte-rs

---

# Backstory

My name is Constantine, I've worked as a frontend dev at an enterprise company for 5 years. We've been using SvelteKit since the beta.

We have a huge Svelte codebase. The biggest project has 25k components counting internal packages. And somewhere around '24, I think, we hit the wall: projects took forever to build, LSP lagged in the IDE, svelte-check crawled. On 8-gig Macs (not many people had those) the projects simply wouldn't start. Dev server took ages to boot and ran slow. Laptops got hot and loud.

At that point TypeScript on Go hadn't even been announced: Project Corsa was announced only in March 2025, i.e. already near the end of my first attempt. I think void0 had said they were building Rolldown. And what we had was Vite 5, Svelte 4 in the monolith, Svelte 5 in the microfrontends. A solution from the outside world was still far off: Vite 8 went stable in March 2026, Rolldown 1.0 shipped in May, TypeScript 7 RC in June, GA in July (LSP is still a work in progress for TS 7, and there's no programmatic API support).

Around that time, in one of ThePrimeagen's videos, I saw a recommendation to read "Writing an Interpreter in Go". I read it, wrote the Monkey interpreter in Go, and got completely hooked.

Next I decided to try writing a JS/TS parser in Go, just for kicks. And, reading the ECMAScript spec, I was horrified, and TS doesn't even have a spec, all they have is a 9 MB, 196k-line `typescript.js`. So I dropped it, not worth the candle.

A week later I decided to narrow the scope and picked Svelte, since, well, it's a compiler.

I started on Nov 4, 2024. At first I thought: there isn't that much code, I'll just port it line by line. I really didn't like how the Svelte parser was written, it looked nothing like what I'd seen in the book. Lots of regex work, extra traversals, scans and so on. So I decided to write the parser from scratch myself, prototyping it as I went, reverse-engineering by peeking at Svelte's AST.

# First try

I spent five and a half months on this: from Nov 2024 to Apr 2025, 303 commits, 108 days with commits. Back then AI agents either didn't exist yet or I didn't know they did. Everything was done by hand, learning Rust along the way, tilting at borrow checker and its other fun concepts.

I'd take some simple unit of Svelte: render text, render a static HTML element, render an interpolation. You can see how it went from the commits. On Nov 6 I started scanning interpolations. On Dec 8 I committed `finally interpolation`. A month for `Hello {name}!`.

First I wrote the scanner, which broke the string `<div> text </div>` into tokens like StartTag {name: "div"}, Text {value: " text "}, EndTag {name: "div"}. Then parser, which turned those tokens into an AST. Then codegen part, which turned that into JS. On simple cases this was very easy to do.

The first hard part was how to parse JS. Svelte used acorn for that. OXC already existed, it's the core of Vite 8 / Rolldown, and it's why I picked Rust, because Go had no maintained JS/TS parsers.

A script tag is fairly easy to parse, but an expression like `{ 1 + 1 }` was not obvious to me. You can't just scan from brace to brace, because a JS expression can have nested braces inside. It can have string literals like `"${name}"`. The original compiler just hands acorn the chunk starting at `{` and lets it find the end of the expression itself.

I couldn't think of anything better than counting brace balance and not parsing JS in the lexer at all. OXC also couldn't behave like acorn and would just blow up. So lexing stayed backtracking-free, O(n).

That's also when I decided to write a recovering parser. Svelte's parsing is built on exceptions: the first error stops it, and in the IDE you see one problem at a time. In mine the error goes into a list and parsing continues. The idea was to eventually embed it in an LSP.

After that came a lot of not understanding how the Svelte compiler is built. I'll cover two of those.

There are various optimizations inside. For example, the original strips extra `\n`, `\t`, `\s` characters from the start of lines. And in `<h1>Hello {name}!</h1>` three tokens (text, expression, text) get glued into a single template literal:

```js
var root = $.from_html(`<h1></h1>`);

export default function App($$anchor) {
    var h1 = root();
    h1.textContent = `Hello ${name ?? ''}!`;
    $.append($$anchor, h1);
}
```

No three text nodes, no three assignments. Things like that don't fall out of the grammar, you have to go find them in the original one by one.

And how do you design an AST in Rust? I decided to follow the JS version of the AST and store strings. But not `String`, just `str`, efficient, no allocations, same string, I thought. The next mistake was storing meta information in the AST, like the original does, filled in by analysis phase. Both decisions meant a lifetime grew through the whole AST. And mutating such a tree without Cell, RefCell, .borrow was impossible. Together it turned into code that's slow and boring to write and maintain.

A separate headache: where to put all these optimizations and facts about the code. I didn't want to shove them into the AST, it was already bloated with meta fields. That's when I read about how HIR works in rustc, and it seemed logical: there it is, a second tree, let's put everything there.

On Feb 25 I created two crates, `hir` and `ast_to_hir`. A second tree that the AST lowers into, with `NodeId`, `OwnerId`, `AttributeId` identifiers instead of references. In a week I had lowering, compilation, and a skeleton for analysis and transform; by early March, text and elements were flowing through it.

And it was pointless. The separate tree turned out to be unnecessary: the thing I built it for became analysis and semantics in the second attempt, with no second tree at all. I didn't get that at the time and just ended up with two ASTs instead of one, a lowering step between them, and twice as much boilerplate for every little thing.

That was the final nail in the coffin of my motivation.

Five and a half months, 303 commits, and I'd gotten nowhere. I was falling further behind the original, they were shipping new features, OXC kept updating, and I was drowning in lifetimes and boilerplate. Writing Rust was exhausting and I spent more time threading types around than doing actual useful work. I understood I wouldn't finish it even in a year. And I'd burn out on top of that, and stay behind the original forever. I quit on Apr 24, 2025, on the commit `each block`, in the middle of `{#each}`. And forgot about the whole thing for ten and a half months.

# Second try: AI generation

On Mar 9, 2026 I came back. In the meantime AI had exploded, I'd discovered Cursor, Claude Code, Codex for myself and built all sorts of stuff with them, from a Steam Deck plugin to a VS Code extension.

Anyway, I discovered AI, and it was a really interesting experience. It helped me do something I'd never have wanted to do myself in my free time, that would have taken me way longer, and I still wouldn't have solved the problem.

The thought crept into my head: what else could one person pull off with AI? And I remembered the Rust compiler, and off we went.

The work happened after an 8-hour workday, in the evenings or on weekends: 42% of commits between 19:00 and 01:00, 43% on weekends.

I picked Claude for $20. Very quickly I started burning through the whole subscription and hitting limits. So I moved to the $100 subscription, which lasted until mid-spring, when Anthropic's cache problems started, and until Anthropic moved onto Grok's infra.

In one month that's 100,864 messages, 100M output tokens and 24B cache-read tokens. At API list prices that's around $17,000. Over five months it would have added up to something like 80k. I paid $700.

## Getting current

The first thing I did: researched how to get rid of string lifetimes sprouting everywhere. Exactly the thing I drowned in on the first attempt. The solution: store a span in the AST, start + end, instead of `str`. That makes working with AST way simpler. In a few places I cheated and put a String there. An extra allocation, but not critical. It made writing code much easier. Five months of pain, and one evening to make it go away.

Then I updated Svelte to the version that was current at the time. I dropped the compiler's sources and the docs from svelte.dev into the project root so the model could easily look at the original implementation. Then I aligned OXC version with the current one. I also got rid of the HIR version of the AST and of the intermediate parser and codegen versions that had appeared because of HIR. So we had a project you could actually work with.

All the old code I nuked on day two, Mar 10, in a commit called `Drop legacy version`: 82 files, +10 / −12,296.

A few questions nagged at me the whole way:

- Where do I even start? How do I set up the process? What do I do with features that depend on each other?
- How do I test this?
- How do I review this much code?
- How do I organize work across many sessions?
- How do I make AI navigation more efficient?
- How do I write skills, and why?
- What do I do when the AI does the wrong thing?

## First kind of work

It looked roughly like this: "scan the whole original and produce a checkbox list of the features that exist in ./original/compiler". And I got a list: 128 lines, 76 checkboxes, zero checked.

Then I'd write a big prompt, basically: "take a checkbox from todo.md, look at the original, and implement it according to our architecture."

At this stage I gave myself over to the Vibe... I reviewed badly, and I didn't have much of an understanding back then of how I wanted to see the compiler; the only part I could look at closely was parser. Progress went by leaps and bounds, lots of code, lots of features implemented fast. In three weeks of March, 631 commits, peaking at 85 in a day, fifteen days straight with no days off.

Somewhere around there the first mega-skills `/port` + `/audit` appeared. Five competing trackers showed up: `plan.md`, `PROGRESS.md`, `ROADMAP.md`, `TODO.md`, `PLAN.md`. How to track progress effectively was anyone's guess.

It was also nice to see everything in the code covered in comments... (we'll come back to those).

That's also when the testing approach appeared. There's a `just generate` command that takes `cases/*/case.svelte` files and generates a case.svelte.js file next to each one, from the original compiler. Our compiler is pointed at the same cases, and you get a comparison. We'd take case.svelte.js and diff it against our output. Tests got added any which way, by the AI, with no system, depending on its mood.

While the tests piled up, a few facts came out: the original strips TS, but not all of it. For example, enums get diagnostics, i.e. it's assumed a preprocessor will handle that. The original and OXC write comments differently, so comments were removed from the test comparison. The original's CSS parser is half-baked: instead of cutting out unused AST nodes, they get wrapped in `/* (unused) */` comments via magic string, and it only deletes them when minification is on. So I had to normalize CSS with third-party tools before comparing. Also, with customElement or in dev mode, CSS with comments gets inlined into the JS, which also had to be normalized.

That's also when "toilet coding" started (as a joke). That's when you can launch agents from claude.ai in the cloud in a chat and do the same thing you do from your computer, but from your phone, in the toilet, in bed, on the road, whatever. From the phone I'd fire off `/audit` -> `/port` -> merge.

Then a few code-review skills got added: `/review`, then four separate commands per crate, which collapsed back into one three days later, then `/fix-review`, then three agents: `codebase-analyzer`, `quality-reviewer`, `reference-tracer`. They were slow, often false-positive, ate tokens, and gave little in return. They'd assemble giant md files, checklists of problems, rank them by importance, and it was all crap. Fix it once, and the AI repeats it in another session. Later I deleted them all at once along with fifteen more commands.

## Limits

In mid-spring Anthropic ran into trouble and didn't have enough resources and hardware. They introduced peak hours / cut limits, there was a lot of rage. I noticed it too, which pushed me to Codex for $200. But Codex turned out to be trash, imho. It wrote a lot of code, fast, pretty. Just not the right code. It was always raw, half-done, hacky, ad hoc. Its plans were just as shallow. I couldn't do anything about it, filed a refund and went to $200 Claude, sat there for 2 months, then moved to the $100 subscription. They doubled the limits on May 6, by the way, I was already back by then.

## Second kind of work

It started when progress slowed down: tons of code being written, but slowly and without moving forward, tests green and yet plenty of mismatches.

I was building up an eye for it and an understanding of how I saw this thing. I discovered the AI didn't give a damn about separation of layers scanner -> parser -> analyze -> transform -> codegen. It would happily call parseJs in the scanner, in the transform, and in the analysis. Why? Because the original has no middle: it rediscovers domain facts right there in codegen, and the agent dutifully copied that. It was porting more and more instead of following our architecture. The architecture was: "scanner = scans. parser = parses and does zero analysis. analyze walks the AST and collects facts about the code. transform only walks the AST and mutates some of its nodes, rewrites runes, unwraps variable references, and so on. codegen generates the final code, without computing meaning, and stays dumb." That was violated from commit zero, the part I wasn't reviewing.

I also started noticing the AI was fudging results to match. Not the code it generated, but the generated output, bending it to our answer, with the words "the original is buggy here." I lost it when I ran `just generate` by hand and then the tests, and there were a lot of failures. I had to add a note to every skill and to claude.md about what you can touch by hand and what you can't. It appeared Apr 23, verbatim: "Never edit by hand case-_.json, case-_.js files. Only generated by `just generate`".

That's when the big refactor started, with me eyeballing everything. Duplicate codegen was found, parsing inside analysis, analysis inside codegen, a smeared-out transform. Weird enums had spread across the codebase, layer responsibilities were violated. I spent the next month fixing all that with no new features; those problems still exist as GitHub issues, but there are almost none left.

It cost a lot. On Apr 23 a single commit deleted 10,894 lines across the crates, killed the `template/` directory that had been touched 767 times, and cut `CLAUDE.md` from 165 lines to 47. By Apr 30, of the 86 March `.rs` files, 48 survived: I erased two thirds of March's code.

I also noticed that comments in the code are garbage. They're on everything, on every little thing. I tried to forbid the model from writing them in claude.md, argued with it, put it in the skills. Didn't help. **First takeaway: existing code will be repeated by the model, all your constraints for the model are suggestions, and it will wipe its ass with them. So from commit zero, set up strict guardrails, so no tech debt is left around for the model to copy and waste your time on.** I stripped the comments out with a regex, all of them. And wrote a hook that prevents writing them, and that worked. Over the project's entire history, 10,374 lines of comments were added to `crates/**/*.rs` and exactly the same number deleted.

In parallel I was trying to find a way to track progress, what to do now, what's not done, and so on. Because there are a ton of features, a ton of compiler options, and everything overlaps. At that point I was tracking everything in ./specs/feature_name.md.

I tracked them badly, the checklist got expanded by the AI via one of my skills and turned into a dumpster. Instead of a feature checklist with descriptions of what each feature does, the files turned into a registry of tests and comments on them. Burned tokens, gave nothing to me or the model. It's clearest in one file: on Mar 28 it had 141 lines and zero checkmarks; on May 14, 138 lines and 99 checkmarks. Same size.

## Third kind of work. We grinding again boy

So, after the cleanup on May 27, it's early June, post-refactor, and what we have is a compiler, client code only, still a rawish analysis phase, codegen/transform partly cleaned of analysis but not fully. There were 1113 tests at that point.

I decide to write a sweep utility that takes a directory path and compares every Svelte component with how we compile it. First on the enterprise codebase. Out of 7k components, about 40 compiled successfully. That was a modern Svelte 5 codebase with sprinkles of legacy syntax.

The grind began, and it lasted months. (In hindsight I'd note that I should have run sweep against the original's test cases; later that's exactly what I did.) I'd take a component from sweep with a mismatch and throw it at a new skill, `/dig`. Its job: create a minimal repro for the finding. 1 test! (we'll come back to this). And report what didn't match. Under the hood it used another skill, `/quick-check`, which told the agent how to quickly run the comparison against our compiler. Then came a very smart combine-harvester skill that tried to figure out what the mismatch was about, sketch an implementation, and check itself with a subagent. And that's how I moved from 7k failures toward 0. It took two months, from May 9 to early July. Client prod bundle only, no dev, no SSR.

That's when it hit me how much had been missed in the first kind of work, how many edge cases and so on. I couldn't automate this process, and even though it was mostly repetitive, a pile of architectural questions kept popping up that had to be solved.

That's also when the idea of `ReactivitySemantics` came to me. All the reactive primitives were implemented as a pile of raw indices (hash maps) and contained a myriad of boolean flags. What I wanted was a scheme where codegen/transform makes one query by entity id and gets a ready answer with all the domain information, enough to make every decision. Instead of codegen doing this:

```rust
if attr_name == "defaultChecked" && has_static_true_boolean_attribute(el, "checked") {
    return RegularAttrUpdate::Call { setter_fn: "$.set_default_checked", attr_name: None };
}

fn has_static_true_boolean_attribute(el: &Element, name: &str) -> bool {
    el.attributes
        .iter()
        .any(|attr| matches!(attr, Attribute::BooleanAttribute(ba) if ba.name == name))
}
```

And out of that thought later grew the /verdict-directed principle: analysis produces a verdict, and transform and codegen pick a form based on it and never reconstruct a domain fact themselves, not from a node name, not by re-walking the AST. AttributeSemantics, ReferenceSemantics, BindingSemantics, ElementSemantics, FragmentSemantics, RuntimeSemantics appeared, all of them domain facts. So that the code tends toward this shape:

```
query semantic by id -> match exhaustive Semantic -> emit/rewrite
```

That refactor also took a long time, but now I started to fully understand and notice when something was going wrong. It got easier to keep an eye on the AI. When it does something ad hoc in codegen that belongs in semantics, I see it now. And the `/verdict-directed` review agent got noticeably better and actually useful. Clear dependencies between analysis passes also appeared, instead of spaghetti.

./specs was wiped, and ./docs/ came in its place, holding real specs/PRDs. They described how a given piece of the system works. ReactivitySemantics, for example. What it's responsible for, what the main public methods are, where it lives. Invariants of what you can and can't do. These files are reviewed strictly per `/writing-docs` for fluff and junk, only the info that's hard or expensive to derive from the code, because it runs through many systems. A `/required` skill was written for it: it greps tags out of ./docs and reads the files that match. As a whole this gave a boost in writing quality and speed. Fewer ad-hoc fixes, and more often in the right places. A model is a model, and sometimes it wrote absolute garbage, you just have to make peace with that.

Sweep was closing mismatches, slowly. And then I started running it against the original compiler's tests, and those were still >50% failing. I decided to rebuild the workflow one more time.

The `/dig` skill was developed. Its job was to take a file/directory, find the mismatch or the most common mismatch in the input. Follow that thread back into the original. Look at every if/switch/lambda in there and write e2e tests for them. And produce a summary of what the problem was. Then in the chat I'd tell it to do this or that, or use `/grill-me` to arrive at an implementation, especially where a lot of code had to be shaken up. If the task was big, I used the `/to-spec` -> `/to-tickets` skills; after that I had GitHub issues, each of which I implemented in a separate session. All together this gave a strong boost to parity and to how pleasant it was to work on. `/grill-me` and a couple of others I took ready-made from Matt Pocock's skill set, I didn't invent everything myself.

## Fourth kind of work

July 1. Reader, we're close to the resolution. The client prod codegen and transform were done, all the original's tests passed. On the enterprise projects all 144k Svelte components matched.

Time to implement SSR, but where do you start? Luckily, the original's SSR uses the same analysis as CSR. The decision was to turn every existing e2e test case into an SSR one as well. And add dev mode too. Total test count came out to number of files * 4 (client dev, client prod, ssr dev, ssr prod). SSR was off by default.

With the AI's help the work was split into clusters, by the principle of what yields the most green tests. Implementing SSR took roughly 2-3 weeks. All thanks to the work from the third kind of work. On Jul 1 there were zero server-side reference outputs in the repo; on the 6th there were 2219 out of 2219. Sweep ran all 144k components and the original's tests across all 4 modes, and everything passed.

Somewhere around here I noticed sweep wasn't accounting for the experimental async feature, so I had to grind and refactor for another week, implementing AwaitSemantics.

I'd been profiling the compiler constantly and optimizing things here and there, but now the focus shifted to exactly that. `nativePreprocess` options appeared, letting you preprocess TypeScript and SCSS on our side so less has to happen on JS side. A fork of the Vite plugin was built with the compiler wired into it.

@mrwaip/svelte-rs turned out to be 9-15x faster than the original. And 3.7-4.9x faster than the native rsvelte. **Blazingly fast, as it should be. That's a win!**

Time to test performance on a real codebase!

# The finale

It's end of July now, Vite 8 has shipped, TypeScript 7 has shipped. rsvelte is breathing down my neck. What did we get out of this enormous effort?

The result is mixed.

First, on a codebase with vite < 8 and svelte-check without TypeScript 7, it gives a significant boost. The biggest project, 23k components on Vite 7: it was 3:37, now it's 2:09. Minus 40% off every build. On the Mac with 8 gigs, `npm run build` completes, where with the original compiler it got OOM killed. In dev mode Vite starts faster, prebundles faster, HMR is faster. All of that is great.

Second, on projects with Vite 8 and svelte-check with `--tsgo` the difference isn't that big. 8-10 seconds, about 9% of the build. Rolldown is parallel and keeps the whole structure in Rust code, so it eats less memory by itself, and the slow Svelte compiler isn't sequential anymore. And with Rust compiler it's nothing at all, 0.2 seconds for every component in the project.

But the seconds aren't the point at all, and I only understood that at the end.

The point is memory. JS compiler allocates AST in the same V8 heap where module graph already lives. On Rollup that's not "N seconds slower", that's "does it build or not".

And the compiler is mine, so I can drag into it whatever I want. SCSS and TypeScript preprocessing moved into Rust, sass-embedded got thrown out, grass compiles a style block in 2.6 ms versus 30. With someone else's JS compiler you can't do that at all.

Where there's no bundler, speed matters again. IDE and svelte-check, 2785 components in a loop, no Rolldown to hide the difference behind itself.

And the boring one. It's a drop-in replacement. Byte-for-byte match across 144k components means you install it with one line and remove it with one line. Didn't like it, roll back, nothing breaks.

In the first attempt, 108 days with commits and an abandoned `{#each}`. In the second, 106 days and a working compiler.

For the project the result is mixed. For me it's not. I learned AI agents as a tool and leveled up in Rust like never before. Compiler world stopped being a dark forest for me. And one guy in his spare time really did rewrite the Svelte compiler.

Yes, when I started I didn't know about rsvelte. And I didn't know that with Vite 8 the win would be less noticeable. But it's still a one-line speedup for your project.

Code is here: https://github.com/MrWaip/svelte-rs. It's canary, there are bugs, we're already using it on our projects.

If anyone wants details on the skills and hooks, or how the compiler is built inside, or how the parity harness works, say so and I'll figure something out.

And honestly, I'm taking a break from this project, I'm pretty tired. For a while. Happy hacking, everyone!
