<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
    ul {
      list-style: none;
      position: relative;
    }
    li {
    }

</style>

<div in:fade={{duration:100}} class="db">
    <h1>Database</h1>
            <ul>
            <li>
                <Box title="Create user">
                    <form label=Email>email <input label="Email"/></form>             
                    <form label=Username>username <input label="Username"/></form>             
                    <form label=Password>password <input label="Passsword"/></form>             
                            <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box title="Create record">
                <form label=Name><input label="Email"/></form>             
                <form label=User><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box title="Create item">
                <form label=Name><input label="Email"/></form>             
                <form label=User><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
            <li>
                <Box title="Create form">
                <form label=Name><input label="Email"/></form>             
                <form label=Type><input label="Username"/></form>             
                <form label=Value><input label="Username"/></form>             
                <button>Submit</button>
                </Box>
            </li>
            <ul>
            <li>
                <Box title="Clear table">
                <form label="Table name"><input label="table"/></form>            
                <button type="is-warning">Clear tables</button>
                <button type="is-danger">Clear all tables</button>
                </Box>
            </li>
</div>
