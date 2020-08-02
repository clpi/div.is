<script lang="ts">
</script>
<style>
.nav {
  background-color: #1c1f23;
  padding: 1vh 1vw 1vh 1vh;
  font-size: 1.1rem;
  font-weight: 350;
  overflow: hidden;
  text-align: center;
  width: 100%;
  min-width: 100%;
  height: 50px;
  margin: 0;
  /*padding: 1.8vh 0vw 1.8vh 0vw;*/
  box-shadow: 0px 2px 1px rgba(0,0,0,0.1);
  border-bottom: 1px solid rgba(180, 180, 185, 0.3);

}
.nav ul {
  justify-content: center;
  margin: auto;
  list-style: none;
  /*margin: 0vh 5vw 0vh 5vw;*/
}

.nav li {
  float: left;
  display: block-inline;
  margin: 0 0.8vw 0 0.8vw;
  /*padding: 8px 1.3vw 8px 1.3vw;*/
  border-bottom: 2px solid rgba(100, 100, 100, 0);
}
.nav li:hover {
  border-bottom: 2px solid rgba(100, 100, 100, 80);
  transition: border-bottom 0.3s;
}

.title {
  margin-right: 5vw;
  font-stretch: ultra-expanded;
  font-size: 1.1rem;
  text-shadow: 2px 2px 2px #000;
  font-weight: 400;
}

.link {
  color: #ffca90;
  font-weight: 380;
  text-shadow: 0px 1px 2px rgba(0,0,0,100);
}
.link:hover {
  color: #df9a50;
}
.right-link {
  float: right;
  text-shadow: 0px 1px 2px rgba(0,0,0,100);
}
.right-link a {
  color: #ffca90;
}
.right-link a:hover {
  color: #df9a50;
}

</style>
<div class="nav">
<ul>
  <li class="link">
    <a href="#">
      File
    </a>
  </li>
  <li class="link">
    <a href="#">
      Edit
    </a>
  </li>
  <li class="link">
    <a href="#">
      View
    </a>
  </li>
  <li class="link">
    <a href="#">
      Tools
    </a>
  </li>
  <li class="link">
    <a href="#">
      Prefs
    </a>
  </li>
</ul>
</div>
