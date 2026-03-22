<script>
    import { onMount } from "svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { flip } from "svelte/animate";
    import ChildComponent from "./Child.svelte";

    let {
        title = "Default Title",
        count = 0,
        items = [],
        config = $bindable({}),
        multiplier = 2,
        visible = $bindable(false),
        ...rest
    } = $props();

    const propsId = $props.id();

    let state = $state("");
    let counter = $state(0);
    let rawData = $state.raw({ x: 1, y: 2 });
    let checked = $state(false);
    let group = $state([]);
    let volume = $state(0.5);
    let inputEl;
    let componentRef;

    /** @type {Function | undefined} */
    let show;

    counter = 10;

    let doubled = $derived(count * multiplier);
    let computed = $derived.by(() => {
        return items.length * multiplier + counter;
    });
    let snapshot = $state.snapshot(rawData);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });

    $effect.pre(() => {
        console.log("Pre effect:", counter);
    });

    let tracking = $effect.tracking();

    $inspect(counter, doubled);

    export const APP_VERSION = "1.0.0";

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }

    function action(node, arg) {
        return { destroy() {} };
    }

    function handleClick(e) {
        counter++;
    }

    function getHandler() {
        return handleClick;
    }

    function handleError(error) {
        console.error(error);
    }

    let promise = Promise.resolve(42);
</script>

<svelte:head>
    <title>{title} - Benchmark</title>
    <meta name="description" content="Benchmark component">
    <link rel="canonical" href="/benchmark">
</svelte:head>

<svelte:window onscroll={handleClick} />
<svelte:document onvisibilitychange={handleClick} />
<svelte:body onmouseenter={handleClick} use:action={state} />

{#snippet badge(text, variant)}
    <span class="badge" class:primary={variant === "primary"} class:secondary={variant === "secondary"}>
        {text}
    </span>
{/snippet}

{#snippet card(heading, body)}
    <div class="card">
        <h3>{heading}</h3>
        <p>{body}</p>
        {@render badge("new", "primary")}
    </div>
{/snippet}

<div>
    Chunk 0: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 0</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-0"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 0.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 0.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 0.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 0: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-0-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 0...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-0" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-0">
        Dynamic element chunk 0: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-0", "secondary")}
    {@render card(title, "Content for chunk 0")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 0: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 0: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 1: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 1</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-1"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 1.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 1.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 1.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 1: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-1-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 1...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-1" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-1">
        Dynamic element chunk 1: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-1", "secondary")}
    {@render card(title, "Content for chunk 1")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 1: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 1: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 2: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 2</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-2"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 2.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 2.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 2.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 2: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-2-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 2...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-2" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-2">
        Dynamic element chunk 2: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-2", "secondary")}
    {@render card(title, "Content for chunk 2")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 2: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 2: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 3: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 3</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-3"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 3.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 3.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 3.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 3: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-3-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 3...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-3" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-3">
        Dynamic element chunk 3: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-3", "secondary")}
    {@render card(title, "Content for chunk 3")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 3: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 3: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 4: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 4</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-4"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 4.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 4.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 4.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 4: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-4-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 4...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-4" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-4">
        Dynamic element chunk 4: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-4", "secondary")}
    {@render card(title, "Content for chunk 4")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 4: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 4: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 5: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 5</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-5"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 5.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 5.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 5.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 5: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-5-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 5...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-5" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-5">
        Dynamic element chunk 5: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-5", "secondary")}
    {@render card(title, "Content for chunk 5")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 5: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 5: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 6: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 6</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-6"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 6.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 6.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 6.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 6: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-6-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 6...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-6" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-6">
        Dynamic element chunk 6: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-6", "secondary")}
    {@render card(title, "Content for chunk 6")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 6: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 6: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 7: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 7</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-7"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 7.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 7.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 7.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 7: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-7-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 7...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-7" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-7">
        Dynamic element chunk 7: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-7", "secondary")}
    {@render card(title, "Content for chunk 7")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 7: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 7: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 8: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 8</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-8"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 8.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 8.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 8.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 8: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-8-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 8...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-8" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-8">
        Dynamic element chunk 8: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-8", "secondary")}
    {@render card(title, "Content for chunk 8")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 8: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 8: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 9: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 9</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-9"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 9.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 9.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 9.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 9: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-9-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 9...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-9" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-9">
        Dynamic element chunk 9: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-9", "secondary")}
    {@render card(title, "Content for chunk 9")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 9: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 9: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 10: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 10</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-10"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 10.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 10.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 10.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 10: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-10-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 10...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-10" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-10">
        Dynamic element chunk 10: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-10", "secondary")}
    {@render card(title, "Content for chunk 10")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 10: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 10: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 11: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 11</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-11"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 11.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 11.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 11.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 11: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-11-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 11...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-11" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-11">
        Dynamic element chunk 11: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-11", "secondary")}
    {@render card(title, "Content for chunk 11")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 11: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 11: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 12: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 12</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-12"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 12.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 12.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 12.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 12: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-12-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 12...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-12" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-12">
        Dynamic element chunk 12: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-12", "secondary")}
    {@render card(title, "Content for chunk 12")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 12: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 12: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 13: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 13</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-13"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 13.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 13.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 13.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 13: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-13-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 13...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-13" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-13">
        Dynamic element chunk 13: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-13", "secondary")}
    {@render card(title, "Content for chunk 13")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 13: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 13: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 14: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 14</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-14"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 14.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 14.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 14.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 14: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-14-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 14...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-14" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-14">
        Dynamic element chunk 14: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-14", "secondary")}
    {@render card(title, "Content for chunk 14")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 14: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 14: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 15: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 15</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-15"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 15.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 15.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 15.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 15: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-15-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 15...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-15" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-15">
        Dynamic element chunk 15: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-15", "secondary")}
    {@render card(title, "Content for chunk 15")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 15: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 15: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 16: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 16</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-16"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 16.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 16.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 16.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 16: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-16-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 16...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-16" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-16">
        Dynamic element chunk 16: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-16", "secondary")}
    {@render card(title, "Content for chunk 16")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 16: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 16: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 17: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 17</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-17"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 17.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 17.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 17.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 17: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-17-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 17...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-17" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-17">
        Dynamic element chunk 17: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-17", "secondary")}
    {@render card(title, "Content for chunk 17")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 17: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 17: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 18: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 18</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-18"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 18.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 18.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 18.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 18: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-18-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 18...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-18" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-18">
        Dynamic element chunk 18: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-18", "secondary")}
    {@render card(title, "Content for chunk 18")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 18: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 18: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 19: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 19</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-19"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 19.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 19.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 19.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 19: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-19-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 19...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-19" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-19">
        Dynamic element chunk 19: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-19", "secondary")}
    {@render card(title, "Content for chunk 19")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 19: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 19: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 20: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 20</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-20"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 20.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 20.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 20.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 20: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-20-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 20...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-20" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-20">
        Dynamic element chunk 20: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-20", "secondary")}
    {@render card(title, "Content for chunk 20")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 20: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 20: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 21: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 21</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-21"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 21.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 21.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 21.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 21: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-21-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 21...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-21" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-21">
        Dynamic element chunk 21: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-21", "secondary")}
    {@render card(title, "Content for chunk 21")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 21: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 21: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 22: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 22</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-22"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 22.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 22.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 22.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 22: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-22-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 22...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-22" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-22">
        Dynamic element chunk 22: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-22", "secondary")}
    {@render card(title, "Content for chunk 22")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 22: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 22: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 23: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 23</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-23"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 23.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 23.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 23.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 23: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-23-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 23...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-23" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-23">
        Dynamic element chunk 23: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-23", "secondary")}
    {@render card(title, "Content for chunk 23")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 23: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 23: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 24: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 24</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-24"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 24.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 24.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 24.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 24: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-24-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 24...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-24" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-24">
        Dynamic element chunk 24: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-24", "secondary")}
    {@render card(title, "Content for chunk 24")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 24: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 24: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 25: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 25</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-25"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 25.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 25.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 25.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 25: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-25-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 25...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-25" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-25">
        Dynamic element chunk 25: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-25", "secondary")}
    {@render card(title, "Content for chunk 25")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 25: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 25: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 26: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 26</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-26"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 26.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 26.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 26.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 26: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-26-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 26...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-26" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-26">
        Dynamic element chunk 26: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-26", "secondary")}
    {@render card(title, "Content for chunk 26")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 26: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 26: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 27: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 27</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-27"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 27.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 27.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 27.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 27: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-27-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 27...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-27" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-27">
        Dynamic element chunk 27: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-27", "secondary")}
    {@render card(title, "Content for chunk 27")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 27: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 27: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 28: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 28</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-28"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 28.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 28.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 28.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 28: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-28-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 28...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-28" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-28">
        Dynamic element chunk 28: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-28", "secondary")}
    {@render card(title, "Content for chunk 28")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 28: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 28: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 29: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 29</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-29"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 29.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 29.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 29.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 29: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-29-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 29...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-29" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-29">
        Dynamic element chunk 29: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-29", "secondary")}
    {@render card(title, "Content for chunk 29")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 29: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 29: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 30: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 30</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-30"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 30.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 30.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 30.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 30: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-30-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 30...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-30" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-30">
        Dynamic element chunk 30: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-30", "secondary")}
    {@render card(title, "Content for chunk 30")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 30: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 30: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 31: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 31</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-31"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 31.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 31.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 31.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 31: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-31-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 31...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-31" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-31">
        Dynamic element chunk 31: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-31", "secondary")}
    {@render card(title, "Content for chunk 31")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 31: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 31: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 32: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 32</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-32"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 32.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 32.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 32.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 32: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-32-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 32...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-32" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-32">
        Dynamic element chunk 32: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-32", "secondary")}
    {@render card(title, "Content for chunk 32")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 32: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 32: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 33: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 33</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-33"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 33.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 33.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 33.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 33: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-33-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 33...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-33" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-33">
        Dynamic element chunk 33: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-33", "secondary")}
    {@render card(title, "Content for chunk 33")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 33: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 33: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 34: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 34</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-34"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 34.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 34.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 34.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 34: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-34-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 34...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-34" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-34">
        Dynamic element chunk 34: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-34", "secondary")}
    {@render card(title, "Content for chunk 34")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 34: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 34: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 35: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 35</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-35"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 35.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 35.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 35.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 35: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-35-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 35...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-35" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-35">
        Dynamic element chunk 35: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-35", "secondary")}
    {@render card(title, "Content for chunk 35")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 35: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 35: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 36: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 36</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-36"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 36.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 36.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 36.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 36: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-36-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 36...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-36" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-36">
        Dynamic element chunk 36: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-36", "secondary")}
    {@render card(title, "Content for chunk 36")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 36: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 36: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 37: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 37</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-37"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 37.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 37.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 37.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 37: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-37-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 37...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-37" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-37">
        Dynamic element chunk 37: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-37", "secondary")}
    {@render card(title, "Content for chunk 37")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 37: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 37: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 38: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 38</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-38"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 38.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 38.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 38.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 38: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-38-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 38...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-38" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-38">
        Dynamic element chunk 38: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-38", "secondary")}
    {@render card(title, "Content for chunk 38")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 38: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 38: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 39: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 39</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-39"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 39.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 39.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 39.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 39: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-39-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 39...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-39" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-39">
        Dynamic element chunk 39: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-39", "secondary")}
    {@render card(title, "Content for chunk 39")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 39: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 39: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 40: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 40</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-40"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 40.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 40.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 40.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 40: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-40-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 40...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-40" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-40">
        Dynamic element chunk 40: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-40", "secondary")}
    {@render card(title, "Content for chunk 40")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 40: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 40: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 41: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 41</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-41"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 41.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 41.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 41.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 41: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-41-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 41...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-41" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-41">
        Dynamic element chunk 41: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-41", "secondary")}
    {@render card(title, "Content for chunk 41")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 41: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 41: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 42: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 42</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-42"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 42.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 42.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 42.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 42: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-42-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 42...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-42" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-42">
        Dynamic element chunk 42: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-42", "secondary")}
    {@render card(title, "Content for chunk 42")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 42: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 42: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 43: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 43</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-43"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 43.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 43.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 43.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 43: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-43-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 43...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-43" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-43">
        Dynamic element chunk 43: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-43", "secondary")}
    {@render card(title, "Content for chunk 43")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 43: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 43: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 44: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 44</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-44"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 44.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 44.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 44.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 44: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-44-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 44...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-44" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-44">
        Dynamic element chunk 44: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-44", "secondary")}
    {@render card(title, "Content for chunk 44")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 44: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 44: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 45: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 45</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-45"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 45.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 45.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 45.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 45: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-45-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 45...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-45" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-45">
        Dynamic element chunk 45: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-45", "secondary")}
    {@render card(title, "Content for chunk 45")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 45: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 45: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 46: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 46</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-46"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 46.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 46.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 46.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 46: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-46-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 46...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-46" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-46">
        Dynamic element chunk 46: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-46", "secondary")}
    {@render card(title, "Content for chunk 46")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 46: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 46: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 47: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 47</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-47"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 47.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 47.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 47.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 47: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-47-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 47...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-47" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-47">
        Dynamic element chunk 47: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-47", "secondary")}
    {@render card(title, "Content for chunk 47")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 47: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 47: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 48: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 48</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-48"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 48.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 48.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 48.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 48: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-48-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 48...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-48" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-48">
        Dynamic element chunk 48: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-48", "secondary")}
    {@render card(title, "Content for chunk 48")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 48: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 48: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 49: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}, computed={computed}</p>

    {@html "<b>raw html chunk 49</b>"}
    {@debug counter, state}

    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
        class={{ active: checked, big: counter > 10 }}
        style:color={state}
        style:font-size="14px"
        style:opacity={counter / 100}
        style:--custom="value-49"
        onclick={handleClick}
        onscroll={handleClick}
        onclickcapture={handleClick}
        onfocus={getHandler()}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.

        {#if state}
            {@const localLen = state.length}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor: {localLen}. Chunk 49.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet. Chunk 49.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 49.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#key counter}
        <p transition:slide>Keyed content chunk 49: {counter}</p>
    {/key}

    {#each items as item, idx (item.id)}
        <p {...rest} data-index="chunk-49-{idx}" animate:flip>{item.name}</p>
    {/each}

    {#await promise}
        <p>Loading chunk 49...</p>
    {:then value}
        <p>Resolved: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}

    <input bind:value={state} />
    <input type="checkbox" bind:checked={checked} />
    <input type="radio" bind:group={group} value="opt-49" />
    <div bind:this={inputEl} bind:clientWidth={counter} contenteditable bind:innerHTML={state}>editable</div>
    <video bind:volume={volume} bind:paused={checked}></video>

    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>
    <svelte:element this={state ? "div" : "span"} class="dynamic-49">
        Dynamic element chunk 49: {title}
    </svelte:element>

    <ChildComponent bind:this={componentRef} title={title} onclick={getHandler()} />

    {@render badge("chunk-49", "secondary")}
    {@render card(title, "Content for chunk 49")}
    {@render show?.()}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 49: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 49: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

