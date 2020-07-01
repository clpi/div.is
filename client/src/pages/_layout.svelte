<!-- _layout.svelte -->
<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { Button, Collapse, Icon, Field, Input, Modal} from 'svelma';
  let login=false;
</script>

<style>
    .Navbar {
        border-top: 4px solid #feba80;
        background-image: linear-gradient(#fffffe, #f9f7f5);
    }
    .NavLinks {
    }
    #navHome {
        margin-left: 5vw;
        margin-right: 5vw;
        border: 4px solid #fedaa1;
        padding: 0px 4px 0px 4px;

    }
    #navHome:hover {
        color: #fe8a00;
        background-color: #feeada;
    }
    .Nav {
        margin-left: auto;
        margin-right: auto;
        max-width: 1200px;
        display: block;
    } 
    ul {
        list-style-type: none;
        justify-items: auto;
        margin-top: 1rem;
        margin-bottom: 1.5rem;
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
        font-stretch: expanded;
        font-weight:400;
        outline:0;
    }
    li:hover {
        border-bottom: 4px solid #ffdca9;
        transition: border-bottom-width 0.2s;
    }
    .active {
        border-bottom: 4px solid #fa4;
        align-items: center;
    }
    .active:hover {
        border-bottom: 4px solid #fa4;
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
            <li class:active={$isActive("/contact")}>
                <a href={$url("/contact")}>contact</a>
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
