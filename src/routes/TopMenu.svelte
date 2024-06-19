<script lang="ts">
    import Input from "./common/Input.svelte";
    import FaSearch from 'svelte-icons/fa/FaSearch.svelte'
    import ToggleButton from "./common/ToggleButton.svelte";
    import FaFilter from 'svelte-icons/fa/FaFilter.svelte'
    import FaFolderOpen from 'svelte-icons/fa/FaFolderOpen.svelte'

    export let searchbarMode : "filter"|"search"|"path" = "filter";
</script>
<div class="topmenu">
    <div class="container">
        <ToggleButton selected={searchbarMode == "filter"} style="margin-right:5px"> 
            <FaFilter/>
         </ToggleButton>
        <ToggleButton selected={searchbarMode == "search"} style="margin-right:5px">
             <FaSearch/>
        </ToggleButton>
        <ToggleButton selected={searchbarMode == "path"} style="margin-right:5px">
             <FaFolderOpen/> 
        </ToggleButton>
        <Input style="flex:1"/>
    </div>
</div>

<style lang="scss">
    .topmenu{
        padding: 4px;
        box-sizing: border-box;
        width: 100%;
        height: 100px;
        background: #f7f7f7;
        box-shadow: #d3d3d3 0px 0px 10px;
        z-index: 1;
        position: relative;
    }
    .container{
        display: flex;
        align-items: center;
    }
</style>