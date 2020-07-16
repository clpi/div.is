<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
    .dbhome {
        display: inline-block;
    }
</style>

<div in:fade={{duration:100}}>
    <h1>Database</h1>
        <Tab label = "Home" class="dbhome">
            <ul>
            <li>
                <Box label="Create user">
                    <form label=Email><input label="Email"/></form>             
                    <form label=Username><input label="Username"/></form>             
                    <form label=Password><input label="Passsword"/></form>             
                            <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box label="Create record">
                <form label=Name><input label="Email"/></form>             
                <form label=User><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box label="Create item">
                <form label=Name><input label="Email"/></form>             
                <form label=User><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box label="Create form">
                <form label=Name><input label="Email"/></form>             
                <form label=Type><input label="Username"/></form>             
                <form label=Value><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
        </Tab>
        <Tab label="Operations">
            <ul>
            <li>
                <Box label="Clear table">
                <form label="Table name"><input label="table"/></form>            
                <button type="is-warning">Clear tables</button>
                <button type="is-danger">Clear all tables</button>
                </Box>
            </li>
        </Tab>
    <Tab label="Performance">
    </Tab>
</div>
