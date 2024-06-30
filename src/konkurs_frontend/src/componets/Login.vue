<script setup>
import DeviceReports from '../componets/DeviceReports.vue'
</script>

<template>
  <div v-if="!zalogowany">
    <h2>Zaloguj się</h2>
    <form action="#" @submit.prevent="zaloguj">
      <div>
        <label for="username">Nazwa użytkownika:</label>
        <input v-model="username" type="text">
      </div>
      <div>
        <label for="password">Hasło:</label>
        <input v-model="password" type="text">
      </div>
      <button type="submit">Zaloguj</button>
    </form>
    <p v-if="bladLogowania" class="error">{{ bladLogowania }}</p>
  </div>
  <div v-else>
    <p>Zalogowano jako {{ username }}</p>
    <DeviceReports />
  </div>
</template>


<script>
import { konkurs_backend } from 'declarations/konkurs_backend/index';

export default {
    data() {
        return {
            username: "",
            password: "",
            zalogowany: false,
            bladLogowania: ""
        };
    },
    methods: {
        async zaloguj() {
            
            await konkurs_backend.zaloguj(this.username, this.password).then((response) => {
                if (response == true) {
                    this.zalogowany = true;
                } else {
                    this.bladLogowania = 'Nieprawidłowa nazwa użytkownika lub hasło ';
                }
            });
        }
    }
}
</script>