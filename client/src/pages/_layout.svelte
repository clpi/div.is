<!-- _layout.svelte -->
<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { Button, Collapse, Icon, Field, Input, Modal} from 'svelma';
  let login=false;
</script>

<style>
    .Navbar {
        border-top-color: #111111;
        border-bottom-color: #452392;
    }
    .NavLinks {
        border-bottom: 1px solid #bbbbbb;
    }
    #navHome {
        color: #ff7eaa;
        margin-left: 5vw;
        margin-right: 5vw;
        border-bottom: 4px solid #feca91;

    }
    #navHome:hover {
        color: #fe8a00;
        background-color: #ddd;
    }
    .Nav {
        border-top: 4px solid #feba80;
        background-color: #fffefb;
        display: block;
        background-image: linear-gradient(#fffffe, #fcfcf9)
    } 
    ul {
        list-style-type: none;
        justify-items: auto;
        margin: 0;
        padding: 0;
        overflow: hidden;
    }
    li {
        float: left;
        min-width:auto;
        margin: auto;
        align-items: center;
        vertical-align: center;
    }
    #loginNav {
        float: right;
        color: pink !important;
    }
    #signupNav {
        color: red;
        float: right;
    }
    li a {
        display: inline-block;
        color: #1f1f1f;
        text-align: center;
        padding: 1.8vh 1.5vw 1.8vh 1.5vw;
        text-decoration: none;
        font-stretch: semi-expanded;
        font-weight:400;
    }
    li:hover {
        background-color: #feeaea;
        border-bottom: 4px solid #fc9;
    }
    .active {
        background-color: #feddcc;
        border-bottom: 4px solid #fa4;
        align-items: center;
    }
    #navIcon{
        float: right;
    }

</style>
<div class="Navbar">
    <div class="NavLinks">
        <ul class="Nav">
            <li id="navHome"class:active={$isActive("/")}>
                <a href={$url("/")}>memri</a>
            </li>
            <li class:active={$isActive("/dashboard")}>
                <a href={$url("/dashboard")}>dashboard</a>
            </li>
            <li class:active={$isActive("/about")}>
                <a href={$url("/about")}>about</a>
            </li>
            <li class:active={$isActive("/FAQ")}>
                <a href={$url("/about")}>FAQ</a>
            </li>
            <li id="signupNav" class:active={$isActive("/signup")} >
                <a href={$url("/signup")}>signup</a>
            </li>
            <li id="loginNav" class:active={$isActive("/login")}>
                <a href={$url("/login")}>login</a>
            </li>
            <li id="navIcon">
                <a href="http://github.com/pecusys">
                    <Icon pack="fab" size="is-small" icon="github"/>
                </a>
            </li>
        </ul>
    </div>
</div>
<slot />
