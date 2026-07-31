# The story of how I wrote a Svelte compiler in Rust

Everything was slow for us: builds, the dev server, svelte-check, the LSP. So I went and wrote my own Svelte compiler in Rust. I hacked at it by hand for five and a half months and gave up. A year later I came back with an AI agent and finished it in 106 evenings and $700.

---

Hi everyone! I want to tell you about my path writing a Svelte compiler in Rust, about my motives and the results.

The compiler is here: https://github.com/MrWaip/svelte-rs

---

# Backstory

My name is Constantine, I have been a frontend developer at an enterprise company for 5 years. We have been using SvelteKit since the beta.

Our Svelte codebase is huge. The biggest project has 25 thousand components including internal packages. And I think it was in '24 that we hit the wall: projects took forever to build, the LSP lagged in the IDE, svelte-check took ages. On 8 GB Macs (not many people had those) the projects would not start at all. The dev server was slow to boot and slow to work. Laptops got hot and loud.

At that time TypeScript on Go had not even been announced: Project Corsa was only announced in March 2025, which was already near the end of my first attempt. I think void0 had already said they were building Rolldown. And we had Vite 5, Svelte 4 in the monolith, Svelte 5 in the microfrontends. Help from the outside was still far away: Vite 8 went stable on 12 March 2026, Rolldown 1.0 on 7 May 2026, TypeScript 7 RC on 18 June 2026, and GA on 8 July 2026 (the LSP is still in progress for TS 7, and there is no programmatic API).

Around then I saw ThePrimeagen recommend "Writing an Interpreter in Go" in one of his videos. I read it, wrote the Monkey interpreter in Go, and got completely hooked.

Next I decided to try writing a JS/TS parser in Go, just for fun. And reading the ECMAScript spec I was horrified, and TS had no spec at all, they just have a `typescript.js` of 9 megabytes and 196 thousand lines. So I dropped that idea, not worth the effort.

A week later I decided to narrow the scope and picked Svelte, because it is a compiler after all.

On 4 November 2024 I started. At first I thought: there is not much code, I will just port it line by line. I really did not like that the way the Svelte parser is written looked nothing like what I had seen in the book. Lots of regex work, extra traversals, extra scanning and so on. I decided to write the parser from scratch myself, prototyping it and reverse engineering the Svelte AST by looking at it.

# First try

I spent five and a half months on this: from 4 November 2024 to 24 April 2025, 303 commits, 108 days with commits. At that time AI agents did not exist yet, or I did not know about them. Everything was done by hand, while learning Rust and raging at the borrow checker and its other fun concepts.

I would take some simple Svelte unit: render plain text, render a static HTML element, render an interpolation. You can see how it went from the commits. On 6 November I started scanning interpolation. On 8 December I committed `finally interpolation`. A month for `Hello {name}!`.

And I wrote the scanner first, which turned the string `<div> text </div>` into tokens like StartTag {name: "div"}, Text {value: " text "}, EndTag {name: "div"}. Then the parser, which turned those tokens into an AST. Then the codegen part, which turned that into JS. On simple cases this was very easy to do.

The first hard part was how to parse JS. Svelte used acorn for that. OXC already existed, which is the core of Vite 8 / Rolldown, and that is why I picked Rust, because Go had no maintained JS/TS parsers.

A script tag is fairly easy to parse, but the expression `{ 1 + 1 }` was not obvious to me. You cannot just scan from brace to brace, because there can be nested braces inside a JS Expression. There can be string literals `"${name}"`. The original compiler just hands acorn the chunk starting at `{` and lets it find the end of the expression itself.

I could not come up with anything better than counting brace balance and not parsing JS during lexing, especially since OXC did not behave like acorn and would crash. That way lexing stayed backtracking free at O(n).

Around the same time I decided to build the parser with recovery. Parsing in Svelte is built on exceptions: the first error stops it, and in the IDE you see one problem at a time. In mine the error goes into a list and parsing continues. It was groundwork for embedding into an LSP.

After that came a lot of confusion about how the Svelte compiler works, I will tell you about two things.

Inside there are various optimizations, for example the original strips extra `\n`, `\t`, `\s` characters from the start of strings. And in `<h1>Hello {name}!</h1>` three tokens (text, expression, text) get glued into one template literal:

```js
var root = $.from_html(`<h1></h1>`);

export default function App($$anchor) {
	var h1 = root();
	h1.textContent = `Hello ${name ?? ''}!`;
	$.append($$anchor, h1);
}
```

No three text nodes, no three assignments. Things like this do not follow from the grammar, you have to find them in the original one by one.

And how do you design an AST in Rust? I decided to follow the JS version of the AST and store strings. But not `String`, `&str`, it is efficient after all, no allocations, the same string, I thought. The next mistake was storing meta information in the AST, like the original does, filled in by the analyze phase. Both of these decisions meant a lifetime grew through the entire AST, and mutating a tree like that without Cell, RefCell, .borrow was impossible. All together it turned into code that is slow and boring to write and maintain.

A separate headache: where to put all those optimizations and facts about the code. I did not want to stuff them into the AST, it was already bloated with meta fields. Then I read how HIR works in rustc, and it seemed logical: there it is, a second tree, that is where everything goes.

On 25 February I created two crates, `hir` and `ast_to_hir`. A second tree that the AST gets lowered into, with `NodeId`, `OwnerId`, `AttributeId` identifiers instead of references. In a week I did the lowering, the compilation, a skeleton for analyze and transform, and by early March text and elements went through it.

And there was no point to it. A separate tree turned out to be unnecessary: the thing I built it for became analysis and semantics in the second attempt, with no second tree at all. Back then I did not understand that and just got two ASTs instead of one, a lowering between them, and twice the boilerplate for every little thing.

That was the last nail in the coffin of my motivation.

Five and a half months, 303 commits, and I got nowhere. I was falling further behind the original, they kept shipping new features, OXC kept updating, and I was drowning in lifetimes and boilerplate. Writing Rust was very tiring and spent more time threading types around than doing real useful work. I realized I would not finish this even in a year, and on top of that I would burn out and stay behind the original forever. I quit on 24 April 2025 on the commit `each block`, right in the middle of `{#each}`. And I forgot about this story for ten and a half months.

# Second try AI generation

On 9 March 2026 I came back. In that time AI had grown a lot, I discovered Cursor, Claude Code, Codex and made all kinds of things with them, from a Steam Deck plugin to a VS Code extension.

Anyway, I discovered AI and it was a very interesting experience. It helped me do things I would never have wanted to spend my free time on, that would have taken me much longer, and that I probably would not have solved anyway.

The thought crept in of what else one person could do with AI, and I remembered the compiler in Rust, and off it went.

The work happened after an 8 hour workday, in the evenings or on weekends: 42% of commits between 19:00 and 01:00, 43% on weekends.

I picked Claude for 20 bucks. Very quickly I started burning the whole subscription and hitting limits. So I moved to the 100 buck subscription, which lasted until mid spring, when Anthropic started having cache problems and moved onto Grok infrastructure.

For one month that is 100,864 messages, 100 million output tokens and 24 billion tokens of cache reads. At API list price that is around $17,000. Over five months it would have added up to about eighty thousand. I paid $700.

## Getting the project current

The first thing I did: I researched how to get rid of string lifetimes growing everywhere. Exactly what drowned me the first time around. The solution is this: we store a span in the AST, start + end, instead of `&str`. That makes working with the AST much simpler. In a few places I cheated and put a String. An extra allocation, but not critical. It made writing code much easier. Five months of pain, and one evening to remove it.

Then I updated svelte to the version current at the time. I put the compiler sources and the docs from svelte.dev in the project root, so the model could easily look at how the original does it. Next I aligned the OXC version. I also got rid of the HIR version of the AST and the intermediate parser and codegen versions that had appeared because of HIR. That gave us a project you can actually work with.

I deleted all the old code on day two, 10 March, in the commit `Drop legacy version`: 82 files, +10 / −12,296.

Throughout the whole thing a few questions kept bugging me:

- Where do I even start? How do I build the process? What do I do with features that are tangled together?
- How do I test this?
- How do I review that much code?
- How do I organize work across many sessions?
- How do I make navigation more efficient for the AI?
- How do I write skills, and why?
- What do I do when the AI does not do what I need?

## First mode of work

It looked roughly like this: "scan the whole original and make a checkbox list of features that exist in ./original/compiler". And I got a list: 128 lines, 76 checkboxes, zero checked.

Then I wrote a big prompt, basically: "Take a checkbox from todo.md, look at the original and implement it following our architecture".

At this stage I gave in to the Vibe... I reviewed badly, and I did not have much of an idea of how I wanted the compiler to look, I could only look closely at the parser part. Progress went in leaps, lots of code, lots of features implemented fast. Over three weeks of March, 631 commits, 85 in one day at peak, fifteen days in a row without a break.

Around then the first giga skills /port and /audit appeared. Five competing trackers appeared: `plan.md`, `PROGRESS.md`, `ROADMAP.md`, `TODO.md`, `PLAN.md`. How to track progress efficiently was not clear.

I was also happy to see that all the code was covered in comments... (we will come back to those).

That is also when the testing approach appeared. There is a command just generate, which takes `cases/*/case.svelte` files and generates a case.svelte.js next to them, from the original compiler.

And our compiler is pointed at the same cases, and you get a comparison. We took case.svelte.js and compared it with our own output. Tests got added any old way, by the AI, with no system, depending on its mood.

While the tests were piling up, a few facts came out: the original strips TS, but not all of it. For example enums throw diagnostics, meaning a preprocessor is expected to handle that. The original and OXC print comments differently, so they were removed from the comparison in tests. The original's CSS parser is not a full one: instead of removing unused AST nodes it wraps them in `/* (unused) */` comments through magic string, and it only removes them when minification is on. So I had to normalize CSS with third party tools before comparing. Also with customElement or dev mode the CSS with comments gets inlined into JS, which I had to normalize too.

That is also when "toilet coding" started (as a joke). It is when you can launch agents in a chat on claude.ai in the cloud and do the same thing you do from your computer, but from your phone, in the toilet, in bed, on the road and so on. From the phone I called the skills /audit -> /port -> merge.

Then a few code review skills with agents were added: `/review`, then four separate commands per crate, which collapsed back into one three days later, then `/fix-review`, then three agents: `codebase-analyzer`, `quality-reviewer`, `reference-tracer`. They worked slowly, gave false positives often, ate tokens and gave little in return. They collected huge md files, checklists of problems, rated them by importance, and it was all bullshit. Fix it once and the AI repeats it in another session. Later I deleted all of them at once along with fifteen more commands.

## Limits

In mid spring Anthropic ran into problems and did not have enough resources and hardware. They introduced peak hours and cut limits, there was a lot of ranting. I noticed it too, which pushed me to go to Codex for 200 bucks. But Codex turned out to be crap, imho. It wrote a lot of code, fast, pretty. But not the right code. It was always raw, incomplete, hacky, ad hoc. Its plans were just as shallow. I could not do anything about it, filed a refund and moved to 200 dollar Claude and sat on it for 2 months, then moved to the 100 buck subscription. The limits, by the way, were doubled on 6 May, and by then I had already come back.

## Second mode of work

It started when progress slowed down, a lot of code was being written, but slowly and without progress, tests green and plenty of divergences.

I started developing an eye for it and an understanding of how I see this. I found out the AI did not give a shit about the layer split scanner -> parser -> analyze -> transform -> codegen. It would call parseJs in the scanner, in the transform, and in the analyze. Why? Because the original has no middle: it re-derives domain facts right in the codegen, and the agent copied that faithfully. It was porting more and more instead of following our architecture. The architecture was this: "the scanner scans. the parser parses and does no analysis. analyze walks the AST and collects facts about the code. transform only walks the AST and mutates some of its nodes. it rewrites runes, unwraps references to variables and so on. codegen produces the final code, without computing any meaning, and stays dumb". This was broken from day zero, because I did not review.

I also started noticing that the AI was fitting the result of our parser to the answer. Not the code it generates, but the generated output to our answer, saying "the original is buggy". I lost it when I ran just generate by hand and then the tests, and a lot of them failed. I had to write a note in every skill and in claude.md about what you can touch by hand and what you cannot. It appeared on 23 April, word for word: "Never edit by hand case-_.json, case-_.js files. Only generated by `just generate`".

Then a big refactor started, I looked through everything by eye. Duplicated codegen was found, parsing in analyze, analysis in codegen, a blurry transform. Weird enums had spread across the codebase, layer responsibilities were violated. I spent the next month fixing all of this with no new features, these problems still exist as issues on gh, but there are almost none left.

It cost a lot. On 23 April one commit deleted 10,894 lines in the crates, killed the `template/` directory that had been touched 767 times, and cut `CLAUDE.md` from 165 lines to 47. By 30 April, of the 86 March `.rs` files, 48 survived: I erased two thirds of the March code.

I also noticed that comments in the code are garbage. They are on everything, on every sneeze. I tried to forbid the model from writing them in claude.md, argued with it, in the skills. It did not help. **First takeaway: existing code will be repeated by the model, all your constraints for the model are suggestions, and it will wipe its ass with them. So from day zero take care of strict boundaries, so no tech debt is left, none of the stuff the model will repeat and waste your time on.** I cleaned out the comments with a regex, all of them. And I wrote a hook that blocks writing them, and that worked. Over the whole history of the project, 10,374 lines of comments were added to `crates/**/*.rs` and exactly the same number were deleted.

In parallel I was trying to find a way to track progress, what to do now, what is not done and so on. Because there are a lot of features, a lot of compiler options, everything overlaps. At that point I tracked everything in ./specs/feature_name.md.

I watched them badly, the checklist got expanded by the AI from one of my skills and turned into a dump. Instead of a checklist of features and a description of what the feature does, the files turned into a registry of tests and comments on them. It burned tokens and gave nothing to me or the model. It shows best in one file: on 28 March it had 141 lines and zero checkboxes, on 14 May 138 lines and 99 checkboxes. Same size.

## Third mode of work. We grinding again boy

So, after the cleanup on 27 May, it is early June, after the refactor we have a compiler, client code only, the analyze phase still raw, codegen and transform cleaned of analysis but still not fully. There were 1113 tests at that point.

I decide to write a sweep utility that takes a path to a directory and compares all svelte components with how we compile them. First on the enterprise codebase. Out of 7k components about 40 compiled successfully. That was a modern codebase on svelte 5 with bits of legacy syntax.

The grind started, months long. (In hindsight I would note that I should have run sweep on the original's test cases, which I did later.) I took a component from sweep with a divergence and threw it at a new skill `/dig`. The point of it: create a minimal repro for the finding. One test! (we will come back to that.) And report what did not match. Under the hood it used another skill /quick-check, which told the agent how to quickly run a comparison against our compiler. Then came a very smart combine skill that tried to figure out what the divergence was about, sketch an implementation, check itself with a subagent. And that is how I moved from 7k failures toward 0. It took two months, from 9 May to early July. Client prod bundle only, no dev, no ssr.

That is when it hit me how much had been missed in the first mode of work, how many edge cases and so on. I could not automate this process, even though it was mostly repetitive, because a pile of architectural questions kept coming up that had to be answered.

That is also when the `ReactivitySemantics` idea came to me. The reactive primitives were implemented over a pile of raw indices (hash tables) and carried a myriad of boolean flags. What I wanted was a scheme where codegen and transform do one query by entity id and get a ready answer with all the domain information, enough to make every decision. Instead of codegen doing this:

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

And out of that thought grew the /verdict-directed principle: analysis produces a verdict, and transform and codegen pick the form from it and never recover a domain fact themselves, not by node name, not by walking the AST again. AttributeSemantics, ReferenceSemantics, BindingSemantics, ElementSemantics, FragmentSemantics, RuntimeSemantics appeared, all of them domain facts. So that the code would tend toward this shape:

```
query semantic by id -> match exhaustive Semantic -> emit/rewrite
```

That refactor also took a long time, but now I could properly understand and notice when something goes wrong. It got easier to watch the AI: when it does something ad hoc in codegen that could be moved into semantics. And the /verdict-directed review agent also got noticeably better and started being useful. A clear dependency between analysis passes appeared too, instead of spaghetti.

./specs was cleared out, and ./docs/ took its place, holding real specs and PRDs. They described how one part of the system or another works. ReactivitySemantics, for example. What it is responsible for, what the main public methods are, where it lives. Invariants about what you can and cannot do. These files are reviewed strictly by /writing-docs for filler and junk, only the information that is hard or expensive to derive from the code, because it runs through many systems. /required was written for it. A skill that greps tags from ./docs and reads the files that match. As a whole this change gave a boost in quality and speed. Fewer edits were ad hoc, and more often they landed in the right places. A model is a model, and sometimes it wrote absolute garbage, and you just have to live with that.

Sweep kept closing divergences, slowly. And then I started running it on the original compiler's tests, and there were still more than 50% failing. I decided to rebuild the working workflow one more time.

The /dig skill was built. Its job was to take a file or a directory, find a divergence or the most common divergence in the input. Follow that thread into the original. Look at all the ifs, switches and lambdas that are there and write e2e tests for them. And produce a summary of what the problem was. Then in the chat I would give a command to do this or that, or use /grill-me to arrive at an implementation, especially where a lot of code had to be moved around. If the task was large I used the /to-spec -> /to-tickets skills, after which I had issues on gh, each of which I implemented in a separate session. All together this gave a big boost to parity and to how comfortable it was to work on it. `/grill-me` and a couple more I took ready made from Matt Pocock's skill set, I did not invent everything myself.

## Fourth mode of work

First of July. Reader, we are getting close to the resolution. Client prod codegen and transform were done, all of the original's tests passed. On the enterprise projects all 144 thousand svelte components matched.

Time to implement SSR, but where to start? Luckily the original SSR uses the same analysis as CSR. The decision was to turn all the existing e2e test cases into ssr ones as well. And to add dev mode too. The total number of tests was the number of files * 4 (client dev, client prod, ssr dev, ssr prod). ssr was off by default.

With the AI the work was split into clusters, by which one gives the most green tests. And implementing SSR took about 2 to 3 weeks. All thanks to the work from stage 3. On the first of July there were zero server references in the repository, on the sixth there were 2219 out of 2219. Sweep ran all 144 thousand components and the original's tests in all 4 modes, and everything passed.

Somewhere around then I noticed that sweep did not account for the experimental async feature, and I had to grind and refactor for another week, implementing AwaitSemantics.

I profiled the compiler constantly and optimized things, but now that became the focus. The nativePreprocess options appeared, which let you preprocess TypeScript and SCSS on our side, so less is done on the JS side. A fork of the vite plugin was built with the compiler wired into it.

@mrwaip/svelte-rs turned out to be 9 to 15 times faster than the original. And 3.7 to 4.9 times faster than the native rsvelte. **Blazingly fast, as required. This is a success!**

Time to test performance on a real codebase!

# The end

It is 31 July now, vite 8 has shipped, typescript 7 has shipped. rsvelte is breathing down my neck. What did we get out of all this work?

The result is mixed.

First, on a codebase where vite is below 8 and svelte-check runs without TypeScript 7, this gives a real boost. The biggest project, 23 thousand components on vite 7: it was 3:37, now it is 2:09. Minus 40% on every build. On a Mac with 8 gigs, npm run build finishes, while with the original compiler it gets OOM killed. In dev mode vite starts faster, prebundles faster, HMR is faster. All of that is great.

Second, on projects with vite 8 and svelte-check with --tsgo the difference is not as dramatic, 8 to 10 seconds, about 9% of the build, because Rolldown uses parallelism and keeps the whole structure in Rust code, so it eats less memory automatically and the slow svelte compiler is no longer sequential, it gets parallelized. And with the Rust compiler the time is tiny, 0.2 seconds for all the components in the project.

But the seconds are not the point at all here, and I only understood that at the end.

The main thing is memory. The JS compiler allocates the AST in the same V8 heap where the module graph already sits. On Rollup that is not "slower by N seconds", that is "builds or does not build".

Second: the compiler is mine, and so you can drag anything into it. SCSS and TypeScript preprocessing moved into Rust, sass-embedded is gone: grass compiles a style block in 2.6 ms against 30. With somebody else's JS compiler a move like that is impossible in principle.

Third: where there is no bundler, speed starts to matter again. That is the IDE and svelte-check: 2785 components in a loop, without rolldown to hide the difference.

And fourth, the most boring one: it is a drop-in replacement. Byte for byte matching with the original on 144 thousand components means you can install it with one line and remove it with one line. Did not like it, roll back, nothing broken.

In the first attempt, 108 days with commits and an abandoned `{#each}`. In the second, 106 days and a working compiler.

For the project the result is mixed. For me it is not. I learned AI agents as a tool and levelled up in rust like never before. The compiler world stopped being a dark forest for me. And one person, in his spare time, really did rewrite the Svelte compiler.

Yes, at the start I did not know about rsvelte. And I did not know that with vite 8 arriving the gain would be less noticeable. But it is still a speedup for your project in one line.

The code is here: https://github.com/MrWaip/svelte-rs. It is canary, it has bugs, we are already using it on our projects.

If this kind of experience is interesting to you, I can go into specifics later: the skills and hooks, how the compiler itself is built, how the parity harness is made. Tell me what would be more useful.

And honestly, I am taking a break from this project, I am pretty tired. For a while. Happy Hacking everyone!
