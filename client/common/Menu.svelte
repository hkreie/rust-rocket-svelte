<script>

import MenuItem from './MenuItem.svelte';

let menuItems = [
    'Uptime',
    'Disk Space',
    'Journal',
    'Performance',
    'Power Cycle',
    'Launch Application',
    'Reload',
    'Settings',
    'Status'
];

</script>


{#each menuItems as text}
<MenuItem {text} on:menuClick />
{/each}


<style>


</style>