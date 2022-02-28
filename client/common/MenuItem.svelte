<script>

// third-party components
import LayoutGrid, { Cell } from '@smui/layout-grid';

/* 2022-02-21 HK: Notes that We cannot dynamically components due to a limitation of bundlers
   so all of these icons will need to imported here, then we will load the correct component
   based on the menu item text. */

   // Icons for Components (my convention, prefix with 'I' as in IComponent)
import ICheck from "svelte-material-icons/Check.svelte";
import ITransitConnectionVariant from "svelte-material-icons/TransitConnectionVariant.svelte";
import IDiskSpace from "svelte-material-icons/Database.svelte";
import IJournal from "svelte-material-icons/MathLog.svelte";
import IUptime from "svelte-material-icons/Clock.svelte";
//import IPowerCycle from "svelte-material-icons/PowerCycle.svelte";
import IPowerCycle from "svelte-material-icons/Flash.svelte";
import ILaunch from "svelte-material-icons/Rocket.svelte";
import ISettings from "svelte-material-icons/WrenchOutline.svelte";
import IReload from "svelte-material-icons/Reload.svelte";
//import Plug from "svelte-material-icons/PowerPlug.svelte";
//import PlugOff from "svelte-material-icons/PowerPlugOff.svelte";
import IPerformance from "svelte-material-icons/ChartLine.svelte";
import IStatus from "svelte-material-icons/ShieldCheckOutline.svelte";
import IFoo from "svelte-material-icons/ChartLine.svelte";

import List, { Item, Text } from "@smui/list";

// styling for the icon
export let size = "2em";
export let width = size;
export let height = size;
export let color = "rgb(98,0,238)";
export let viewBox = "0 0 30 30";

// menu item text
export let text;
import { createEventDispatcher } from 'svelte';
let dispatch = createEventDispatcher();
const menuClick = (text) => {
    dispatch("menuClick",text);
    console.log("MenuItem: dispatch menuClick ", text);
}

</script>

<Cell>
    <div class="grid-cell">
        <Item class="item" href="javascript:void(0)" on:click={() => {menuClick(text)}} >
            {#if text === 'Uptime'}
            <IUptime {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Disk Space'}
            <IDiskSpace {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Journal'}
            <IJournal {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Performance'}
            <IPerformance {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Power Cycle'}
            <IPowerCycle {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Launch Application'}
            <ILaunch {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Reload'}
            <IReload {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Settings'}
            <ISettings {size} {color} {width} {height} {viewBox} />
            {:else if text === 'Status'}
            <IStatus {size} {color} {width} {height} {viewBox} />
            {/if}
            <Text><p>{text}</p></Text>
        </Item>
    </div>
</Cell>

<style>


.grid-cell{
    /*border: 2px solid rgb(98,0,238);*/
    border: 2px solid #ccc;
    border-radius: 6px;
    min-width:200px;
}


</style>