<script>

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
export let color = "gray";
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

<Item class="item" href="javascript:void(0)" on:click={() => {menuClick(text)}} >
    {#if text === 'Disk Space'}
    <IDiskSpace {size} {color} {width} {height} {viewBox} />
    {:else if text === 'Uptime'}
    <IUptime {size} {color} {width} {height} {viewBox} />
    {/if}
    <Text>{text}</Text>
  </Item>

<style>

/*
* :global(.item){
    border:2px solid red;
}
*/

</style>