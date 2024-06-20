<script lang="ts">
    import Input from "./common/Input.svelte";
    import FaSearch from "svelte-icons/fa/FaSearch.svelte";
    import ToggleButton from "./common/ToggleButton.svelte";
    import FaFilter from "svelte-icons/fa/FaFilter.svelte";
    import FaFolderOpen from "svelte-icons/fa/FaFolderOpen.svelte";
    import ButtonGroup from "./common/ButtonGroup.svelte";
    import FaListUl from "svelte-icons/fa/FaListUl.svelte";
    import FaThLarge from "svelte-icons/fa/FaThLarge.svelte";
    import FaTicketAlt from "svelte-icons/fa/FaTicketAlt.svelte";
    import FaFile from 'svelte-icons/fa/FaFile.svelte'
    export let searchbarMode: "filter" | "search" | "path" = "filter";
</script>

<div class="topmenu">
    <div class="container">
        <ToggleButton
            selected={searchbarMode == "filter"}
            style="margin-right:5px"
        >
            <FaFilter />
        </ToggleButton>
        <ToggleButton
            selected={searchbarMode == "search"}
            style="margin-right:5px"
        >
            <FaSearch />
        </ToggleButton>
        <ToggleButton
            selected={searchbarMode == "path"}
            style="margin-right:5px"
        >
            <FaFolderOpen />
        </ToggleButton>
        <Input style="flex:1" />
    </div>
    <div class="container" style="margin-top:5px">
        <ButtonGroup name="view mode">
            <ToggleButton large style="margin-right:5px;">
                <FaListUl />
            </ToggleButton>
            <ToggleButton large style="margin-right:5px;">
                <FaThLarge />
            </ToggleButton>
            <ToggleButton large style="margin-right:5px;">
                <FaTicketAlt />
            </ToggleButton>
        </ButtonGroup>
        <ButtonGroup name="grouping" style="margin-left:5px">
            <ToggleButton large style="margin-right:5px;">
                ALL
            </ToggleButton>
            <ToggleButton large style="margin-right:5px;">
                <FaFolderOpen/>
            </ToggleButton>
            <ToggleButton large style="margin-right:5px;">
                <FaFile/>
            </ToggleButton>
        </ButtonGroup>
    </div>
</div>

<style lang="scss">
    .topmenu {
        padding: 4px;
        box-sizing: border-box;
        width: 100%;
        height: 100px;
        box-shadow: #d3d3d3 0px 0px 10px;
        z-index: 1;
        position: relative;
    }
    .container {
        display: flex;
        align-items: center;
    }
</style>
