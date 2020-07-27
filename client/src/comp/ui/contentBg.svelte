<script>
  import { fade } from 'svelte/transition';
</script>
<style>
    .wrapper {
      /*padding-left: 5vw;*/
    }
    .bg {
      margin-top: 30px;
      min-height: 80%;
      height: 80vh;
      background-color: #fefdfd;
      border: 2px solid #eee;
      border-bottom-color: #fd9;
      border-bottom-width: 3px;
      border-radius: 8px;
      /*padding-top: 5%;*/
      padding-bottom: 10vh;
      margin-bottom: 10vh;
      align-items: center;
      margin: auto;
      /*padding-left: 15.5vw;*/
      /*padding-right: 15.5vw;*/
      box-shadow: 1px 2px 2px rgba(0,0,0,0.1);
    }
    .head {
      display: inline-block;
      margin: auto;
      text-align: center;
    }
    .title {
      font-weight: 300;
      text-shadow: -0px -1px 1px #fff;
      font-size: 2.5rem;
    }
    h1 { 
        font-size: 2rem;
        font-weight: 300;
        text-shadow: 0px 1px 1px #444;
    }
    h2 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    }
</style>

    <div in:fade={{duration:100}} class="bg">
      <div class="head">
        <h1 class="title" in:fade={{duration: 600}}><slot name="title"/></h1>
      </div>
      <slot/>
    </div>
