
<!-- /user/:userId/posts/:postId -->
<script>
  import { params } from '@sveltech/routify'
  import { fade } from 'svelte/transition';
  console.log($params) /** {userId: 123, postId: 456} **/
</script>

<style>
    h2 {
        font-size: 2rem;
    }
    .userPage {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
    .bod {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
        margin: auto;
        align-content: center;
    }
    .box {
		width: 300px;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
		margin: 0 0 1em 0;
	}
</style>

<div class="userPage">
    <div class="bod">
        <h2>Hello</h2>
        <p>{$params}</p>
        <slot/>
    </div>
</div>

<div class="bod" in:fade>
    <h2>Find user...</h2>
</div>
