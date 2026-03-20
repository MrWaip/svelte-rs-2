<script lang="ts">
    import type { SvelteComponent } from 'svelte';

    interface User {
        name: string;
        age: number;
    }

    type Status = 'active' | 'inactive';

    let name: string = $state('world');
    let status: Status = $state<Status>('active');

    function greet(user: User): string {
        return `Hello ${user.name}`;
    }
</script>

<p>{name}</p>
<p>{status}</p>
