<script lang="ts">
    export let style="";
    export let name:string|undefined= undefined;
</script>
<div class="button-group" style={style} >
    {#if name}
        <div class="name">{name}</div>
    {/if}
    <div class="inner-container">
        <slot></slot>
    </div>
</div>

<style lang="scss">
    .name{
        margin-left: 2px;
        font-size: 12px;
        color: #666;
    }
    .button-group{
        align-items: center;
        border: 2px solid #e8e8e8;
        border-radius: 2px;
        background-color: white;
        padding: 2px;
    }
    .inner-container{
        display: flex;
        align-items: center;
    }
</style>