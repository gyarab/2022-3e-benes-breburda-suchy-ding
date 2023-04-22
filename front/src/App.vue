<template>
  <RouterView></RouterView>

  <div class="testing">
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/login">Log in</RouterLink>
    <RouterLink to="/register">Register</RouterLink>
    <RouterLink to="/profile">Profile</RouterLink>
    <RouterLink to="/post">Post</RouterLink>
    <RouterLink to="/settings">Settings</RouterLink>
  </div>
</template> 


<style>
@import url('https://fonts.googleapis.com/css?family=Inter');


#app {
  font-family: Inter;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: white;
}

body {
  background: #13131b;
}

button {
  padding: 0.75rem 1.5rem;
  border: 2px solid #20201E;

  @apply bg-[#222230] rounded-3xl text-white hover:bg-[#303044]
}

text {
  color: white;
}

input {
  box-sizing: border-box;
  padding: 1rem 2rem;
  margin: 8px 0;
  border-radius: 25px;
  border: none;
  font-size: 1.25rem;

  @apply bg-[#222230]
}

.testing {
  position: fixed;
  left: 50%;
  bottom: 20px;
  transform: translate(-50%, -50%);
  margin: 0 auto;
}

</style>
