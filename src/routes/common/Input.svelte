<script lang=ts>
export let style="";
export let value:string;
export let keydown:(e:KeyboardEvent)=>void;
</script>
<input style={style} bind:value={value} on:keydown={keydown}/>
<style lang="scss">
    input{
        background: #ffffff;
        border: none;
        box-shadow: #d3d3d3 0px 0px 3px;
        border-radius: 4px;
        padding: 5px;
        &:focus{
            outline: 2px solid #669ed5;
        }
    }
</style>