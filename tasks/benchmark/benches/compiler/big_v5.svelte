<script>
    import { onMount } from "svelte";
    import { fade, fly } from "svelte/transition";

    let {
        title = "Default Title",
        count = 0,
        items = [],
        config = $bindable({}),
        multiplier = 2,
        ...rest
    } = $props();

    let state = $state("");
    let counter = $state(0);

    counter = 10;

    let doubled = $derived(count * multiplier);

    $effect(() => {
        console.log("Title:", title, "Count:", count);
    });

    export const APP_VERSION = "1.0.0";

    export function formatTitle(prefix) {
        return prefix + ": " + title;
    }

    function action(node, arg) {
        return { destroy() {} };
    }

    function handleError(error) {
        console.error(error);
    }
</script>

<svelte:head>
    <meta name="description" content="Benchmark component">
    <link rel="canonical" href="/benchmark">
</svelte:head>

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
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 0.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 0.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 0.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-0">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-0">
        Dynamic element chunk 0: {title}
    </svelte:element>

    {@render badge("chunk-0", "secondary")}
    {@render card(title, "Content for chunk 0")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 0: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 0: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 1: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 1.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 1.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 1.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-1">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-1">
        Dynamic element chunk 1: {title}
    </svelte:element>

    {@render badge("chunk-1", "secondary")}
    {@render card(title, "Content for chunk 1")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 1: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 1: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 2: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 2.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 2.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 2.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-2">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-2">
        Dynamic element chunk 2: {title}
    </svelte:element>

    {@render badge("chunk-2", "secondary")}
    {@render card(title, "Content for chunk 2")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 2: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 2: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 3: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 3.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 3.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 3.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-3">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-3">
        Dynamic element chunk 3: {title}
    </svelte:element>

    {@render badge("chunk-3", "secondary")}
    {@render card(title, "Content for chunk 3")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 3: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 3: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 4: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 4.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 4.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 4.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-4">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-4">
        Dynamic element chunk 4: {title}
    </svelte:element>

    {@render badge("chunk-4", "secondary")}
    {@render card(title, "Content for chunk 4")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 4: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 4: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 5: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 5.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 5.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 5.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-5">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-5">
        Dynamic element chunk 5: {title}
    </svelte:element>

    {@render badge("chunk-5", "secondary")}
    {@render card(title, "Content for chunk 5")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 5: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 5: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 6: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 6.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 6.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 6.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-6">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-6">
        Dynamic element chunk 6: {title}
    </svelte:element>

    {@render badge("chunk-6", "secondary")}
    {@render card(title, "Content for chunk 6")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 6: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 6: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 7: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 7.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 7.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 7.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-7">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-7">
        Dynamic element chunk 7: {title}
    </svelte:element>

    {@render badge("chunk-7", "secondary")}
    {@render card(title, "Content for chunk 7")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 7: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 7: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 8: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 8.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 8.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 8.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-8">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-8">
        Dynamic element chunk 8: {title}
    </svelte:element>

    {@render badge("chunk-8", "secondary")}
    {@render card(title, "Content for chunk 8")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 8: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 8: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 9: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 9.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 9.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 9.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-9">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-9">
        Dynamic element chunk 9: {title}
    </svelte:element>

    {@render badge("chunk-9", "secondary")}
    {@render card(title, "Content for chunk 9")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 9: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 9: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 10: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 10.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 10.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 10.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-10">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-10">
        Dynamic element chunk 10: {title}
    </svelte:element>

    {@render badge("chunk-10", "secondary")}
    {@render card(title, "Content for chunk 10")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 10: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 10: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 11: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 11.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 11.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 11.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-11">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-11">
        Dynamic element chunk 11: {title}
    </svelte:element>

    {@render badge("chunk-11", "secondary")}
    {@render card(title, "Content for chunk 11")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 11: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 11: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 12: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 12.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 12.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 12.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-12">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-12">
        Dynamic element chunk 12: {title}
    </svelte:element>

    {@render badge("chunk-12", "secondary")}
    {@render card(title, "Content for chunk 12")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 12: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 12: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 13: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 13.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 13.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 13.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-13">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-13">
        Dynamic element chunk 13: {title}
    </svelte:element>

    {@render badge("chunk-13", "secondary")}
    {@render card(title, "Content for chunk 13")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 13: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 13: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 14: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 14.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 14.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 14.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-14">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-14">
        Dynamic element chunk 14: {title}
    </svelte:element>

    {@render badge("chunk-14", "secondary")}
    {@render card(title, "Content for chunk 14")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 14: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 14: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 15: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 15.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 15.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 15.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-15">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-15">
        Dynamic element chunk 15: {title}
    </svelte:element>

    {@render badge("chunk-15", "secondary")}
    {@render card(title, "Content for chunk 15")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 15: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 15: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 16: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 16.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 16.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 16.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-16">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-16">
        Dynamic element chunk 16: {title}
    </svelte:element>

    {@render badge("chunk-16", "secondary")}
    {@render card(title, "Content for chunk 16")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 16: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 16: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 17: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 17.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 17.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 17.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-17">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-17">
        Dynamic element chunk 17: {title}
    </svelte:element>

    {@render badge("chunk-17", "secondary")}
    {@render card(title, "Content for chunk 17")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 17: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 17: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 18: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 18.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 18.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 18.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-18">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-18">
        Dynamic element chunk 18: {title}
    </svelte:element>

    {@render badge("chunk-18", "secondary")}
    {@render card(title, "Content for chunk 18")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 18: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 18: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 19: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 19.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 19.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 19.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-19">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-19">
        Dynamic element chunk 19: {title}
    </svelte:element>

    {@render badge("chunk-19", "secondary")}
    {@render card(title, "Content for chunk 19")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 19: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 19: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 20: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 20.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 20.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 20.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-20">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-20">
        Dynamic element chunk 20: {title}
    </svelte:element>

    {@render badge("chunk-20", "secondary")}
    {@render card(title, "Content for chunk 20")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 20: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 20: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 21: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 21.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 21.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 21.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-21">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-21">
        Dynamic element chunk 21: {title}
    </svelte:element>

    {@render badge("chunk-21", "secondary")}
    {@render card(title, "Content for chunk 21")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 21: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 21: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 22: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 22.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 22.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 22.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-22">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-22">
        Dynamic element chunk 22: {title}
    </svelte:element>

    {@render badge("chunk-22", "secondary")}
    {@render card(title, "Content for chunk 22")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 22: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 22: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 23: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 23.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 23.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 23.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-23">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-23">
        Dynamic element chunk 23: {title}
    </svelte:element>

    {@render badge("chunk-23", "secondary")}
    {@render card(title, "Content for chunk 23")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 23: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 23: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 24: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 24.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 24.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 24.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-24">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-24">
        Dynamic element chunk 24: {title}
    </svelte:element>

    {@render badge("chunk-24", "secondary")}
    {@render card(title, "Content for chunk 24")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 24: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 24: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 25: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 25.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 25.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 25.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-25">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-25">
        Dynamic element chunk 25: {title}
    </svelte:element>

    {@render badge("chunk-25", "secondary")}
    {@render card(title, "Content for chunk 25")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 25: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 25: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 26: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 26.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 26.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 26.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-26">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-26">
        Dynamic element chunk 26: {title}
    </svelte:element>

    {@render badge("chunk-26", "secondary")}
    {@render card(title, "Content for chunk 26")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 26: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 26: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 27: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 27.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 27.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 27.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-27">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-27">
        Dynamic element chunk 27: {title}
    </svelte:element>

    {@render badge("chunk-27", "secondary")}
    {@render card(title, "Content for chunk 27")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 27: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 27: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 28: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 28.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 28.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 28.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-28">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-28">
        Dynamic element chunk 28: {title}
    </svelte:element>

    {@render badge("chunk-28", "secondary")}
    {@render card(title, "Content for chunk 28")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 28: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 28: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 29: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 29.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 29.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 29.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-29">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-29">
        Dynamic element chunk 29: {title}
    </svelte:element>

    {@render badge("chunk-29", "secondary")}
    {@render card(title, "Content for chunk 29")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 29: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 29: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 30: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 30.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 30.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 30.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-30">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-30">
        Dynamic element chunk 30: {title}
    </svelte:element>

    {@render badge("chunk-30", "secondary")}
    {@render card(title, "Content for chunk 30")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 30: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 30: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 31: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 31.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 31.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 31.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-31">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-31">
        Dynamic element chunk 31: {title}
    </svelte:element>

    {@render badge("chunk-31", "secondary")}
    {@render card(title, "Content for chunk 31")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 31: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 31: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 32: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 32.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 32.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 32.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-32">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-32">
        Dynamic element chunk 32: {title}
    </svelte:element>

    {@render badge("chunk-32", "secondary")}
    {@render card(title, "Content for chunk 32")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 32: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 32: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 33: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 33.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 33.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 33.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-33">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-33">
        Dynamic element chunk 33: {title}
    </svelte:element>

    {@render badge("chunk-33", "secondary")}
    {@render card(title, "Content for chunk 33")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 33: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 33: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 34: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 34.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 34.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 34.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-34">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-34">
        Dynamic element chunk 34: {title}
    </svelte:element>

    {@render badge("chunk-34", "secondary")}
    {@render card(title, "Content for chunk 34")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 34: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 34: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 35: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 35.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 35.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 35.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-35">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-35">
        Dynamic element chunk 35: {title}
    </svelte:element>

    {@render badge("chunk-35", "secondary")}
    {@render card(title, "Content for chunk 35")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 35: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 35: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 36: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 36.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 36.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 36.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-36">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-36">
        Dynamic element chunk 36: {title}
    </svelte:element>

    {@render badge("chunk-36", "secondary")}
    {@render card(title, "Content for chunk 36")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 36: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 36: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 37: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 37.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 37.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 37.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-37">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-37">
        Dynamic element chunk 37: {title}
    </svelte:element>

    {@render badge("chunk-37", "secondary")}
    {@render card(title, "Content for chunk 37")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 37: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 37: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 38: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 38.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 38.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 38.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-38">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-38">
        Dynamic element chunk 38: {title}
    </svelte:element>

    {@render badge("chunk-38", "secondary")}
    {@render card(title, "Content for chunk 38")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 38: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 38: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 39: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 39.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 39.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 39.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-39">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-39">
        Dynamic element chunk 39: {title}
    </svelte:element>

    {@render badge("chunk-39", "secondary")}
    {@render card(title, "Content for chunk 39")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 39: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 39: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 40: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 40.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 40.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 40.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-40">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-40">
        Dynamic element chunk 40: {title}
    </svelte:element>

    {@render badge("chunk-40", "secondary")}
    {@render card(title, "Content for chunk 40")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 40: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 40: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 41: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 41.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 41.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 41.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-41">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-41">
        Dynamic element chunk 41: {title}
    </svelte:element>

    {@render badge("chunk-41", "secondary")}
    {@render card(title, "Content for chunk 41")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 41: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 41: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 42: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 42.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 42.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 42.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-42">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-42">
        Dynamic element chunk 42: {title}
    </svelte:element>

    {@render badge("chunk-42", "secondary")}
    {@render card(title, "Content for chunk 42")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 42: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 42: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 43: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 43.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 43.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 43.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-43">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-43">
        Dynamic element chunk 43: {title}
    </svelte:element>

    {@render badge("chunk-43", "secondary")}
    {@render card(title, "Content for chunk 43")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 43: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 43: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 44: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 44.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 44.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 44.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-44">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-44">
        Dynamic element chunk 44: {title}
    </svelte:element>

    {@render badge("chunk-44", "secondary")}
    {@render card(title, "Content for chunk 44")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 44: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 44: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 45: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 45.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 45.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 45.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-45">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-45">
        Dynamic element chunk 45: {title}
    </svelte:element>

    {@render badge("chunk-45", "secondary")}
    {@render card(title, "Content for chunk 45")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 45: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 45: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 46: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 46.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 46.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 46.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-46">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-46">
        Dynamic element chunk 46: {title}
    </svelte:element>

    {@render badge("chunk-46", "secondary")}
    {@render card(title, "Content for chunk 46")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 46: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 46: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 47: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 47.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 47.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 47.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-47">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-47">
        Dynamic element chunk 47: {title}
    </svelte:element>

    {@render badge("chunk-47", "secondary")}
    {@render card(title, "Content for chunk 47")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 47: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 47: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 48: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 48.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 48.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 48.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-48">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-48">
        Dynamic element chunk 48: {title}
    </svelte:element>

    {@render badge("chunk-48", "secondary")}
    {@render card(title, "Content for chunk 48")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 48: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 48: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

<div>
    Chunk 49: Lorem {state} + {state} = Ipsum;
    <p>Props: title={title}, count={count}, doubled={doubled}</p>
    <div
        class:state
        class:staticly={true}
        class:invinsible
        class:reactive={counter}
    >
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat.

        {#if state}
            <span title="{title}: {doubled}" empty {state} {counter} count={count}>
                Duis aute irure dolor in reprehenderit in voluptate velit esse
                cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                cupidatat non proident, sunt in culpa qui officia deserunt
                mollit anim id est laborum. Chunk 49.
            </span>
        {:else}
            <div>
                <input {title} {state} value={count} />
            </div>

            {#if counter > 30}
                <h1 {state}>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
                    do eiusmod tempor incididunt ut labore et dolore magna
                    aliqua. Chunk 49.
                </h1>
            {:else if counter == 100}
                Lorem ipsum dolor sit amet. Chunk 49.
            {:else}
                <h2>EMPTY</h2>
            {/if}
        {/if}
    </div>

    {#each items as item}
        <p {...rest} data-index="chunk-49">{item}</p>
    {/each}

    <input bind:value={state} />
    <div use:action={state}>action target</div>
    <div transition:fade>transition target</div>
    <div in:fly={{ y: 200 }} out:fade>in/out target</div>

    <svelte:element this={state ? "div" : "span"} class="dynamic-49">
        Dynamic element chunk 49: {title}
    </svelte:element>

    {@render badge("chunk-49", "secondary")}
    {@render card(title, "Content for chunk 49")}

    <svelte:boundary onerror={handleError}>
        <p>Boundary chunk 49: {title}</p>
        {#snippet failed(error)}
            <p>Error in chunk 49: {error.message}</p>
        {/snippet}
    </svelte:boundary>
</div>

